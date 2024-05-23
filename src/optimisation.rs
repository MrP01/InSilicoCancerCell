use argmin::core::{CostFunction, Error, Executor, Gradient};
use argmin_math::ArgminInv;
use in_silico_cancer_cell::cell::ChannelCounts;
use nalgebra::SVector;

use crate::cell::SimulationRecorder;
use crate::cell::N_CHANNEL_TYPES;
use crate::cell::{evaluate_current_match, A549CancerCell};
use crate::patchclampdata::PatchClampData;
use crate::pulseprotocol::DefaultPulseProtocol;

pub struct SingleChannelCurrentRecord {
  pub currents: [Vec<f64>; N_CHANNEL_TYPES],
}
impl SingleChannelCurrentRecord {
  pub fn empty() -> Self {
    Self {
      currents: [1; N_CHANNEL_TYPES].map(|_| vec![]),
    }
  }
}
impl SimulationRecorder for SingleChannelCurrentRecord {
  fn record(&mut self, cell: &A549CancerCell, voltage: f64) {
    for (channel, current) in std::iter::zip(cell.channels(), &mut self.currents) {
      current.push(channel.single_channel_current(voltage));
    }
  }
}

struct ChannelCountsProblem {
  fit_to: PatchClampData,
  current_basis: Option<nalgebra::DMatrix<f64>>,
}
type F64ChannelCounts = SVector<f64, N_CHANNEL_TYPES>;

impl ChannelCountsProblem {
  fn new(fit_to: PatchClampData) -> Self {
    Self {
      fit_to,
      current_basis: None,
    }
  }
  fn precompute_single_channel_currents(&mut self) {
    let pulse_protocol = DefaultPulseProtocol {};
    let mut cell = A549CancerCell::new();
    let mut recorded = SingleChannelCurrentRecord::empty();
    cell.simulate(pulse_protocol, &mut recorded, self.fit_to.current.len());
    self.current_basis = Some(nalgebra::DMatrix::from_columns(
      &recorded.currents.map(|c| nalgebra::DVector::from(c)),
    ));
    log::info!("Finished pre-computation of single-channel currents.");
  }
  fn solve_through_projection(&self) -> ChannelCounts {
    let qr = self
      .current_basis
      .clone()
      .unwrap()
      .rows(0, self.fit_to.current.len())
      .qr(); // computes A = QR
    let mut transformed = self.fit_to.current.clone();
    // qr.q_tr_mul(&mut transformed); // computes Q^T I_m
    transformed = qr.q().transpose() * transformed;
    let r_inv = qr.unpack_r().inv().expect("R matrix should be invertible");
    // then finally computes R^(-1) Q^T I_m, the solution of the least-squares problem
    let solution = r_inv * transformed;
    return solution
      .iter()
      .map(|x| x.round() as u32)
      .collect::<Vec<u32>>()
      .try_into()
      .unwrap();
  }
}

impl CostFunction for ChannelCountsProblem {
  type Param = F64ChannelCounts;
  type Output = f64;
  fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
    Ok(evaluate_current_match(
      &self.fit_to,
      self.current_basis.as_ref().unwrap() * param.map(|x| x.round()),
    ))
  }
}

impl Gradient for ChannelCountsProblem {
  type Param = F64ChannelCounts;
  type Gradient = F64ChannelCounts;
  fn gradient(&self, current: &Self::Param) -> Result<Self::Gradient, Error> {
    print!("g");
    let mut grad = Self::Gradient::zeros();
    let current_cost = self.cost(current)?;
    for i in 0..9 {
      let mut temp_params = current.clone();
      temp_params[i] += 1.0;
      grad[i] = (self.cost(&temp_params)? - current_cost) / 2.0;
    }
    Ok(grad)
  }
}

#[derive(Clone, clap::ValueEnum)]
pub enum InSilicoMethod {
  Projection,
  ParticleSwarm,
  SteepestDescent,
  LBFGS,
}

pub fn find_best_fit_for(data: PatchClampData, using: InSilicoMethod) {
  let mut problem = ChannelCountsProblem::new(data);
  problem.precompute_single_channel_currents();
  match using {
    InSilicoMethod::Projection => {
      let solution = problem.solve_through_projection();
      println!("Solution: {:?}", solution);
    }
    InSilicoMethod::ParticleSwarm => {
      let solver = argmin::solver::particleswarm::ParticleSwarm::new(
        ([0.0; N_CHANNEL_TYPES].into(), [1350.0; N_CHANNEL_TYPES].into()),
        200,
      );
      let result = Executor::new(problem, solver)
        .configure(|state| state.max_iters(100))
        .run()
        .unwrap();
      println!("{}", result);
    }
    InSilicoMethod::SteepestDescent => {
      let linesearch =
        argmin::solver::linesearch::HagerZhangLineSearch::<F64ChannelCounts, F64ChannelCounts, f64>::new();
      let solver = argmin::solver::gradientdescent::SteepestDescent::new(linesearch);
      let result = Executor::new(problem, solver)
        .configure(|state| state.max_iters(10))
        .run()
        .unwrap();
      println!("{}", result);
    }
    InSilicoMethod::LBFGS => {
      let linesearch =
        argmin::solver::linesearch::HagerZhangLineSearch::<F64ChannelCounts, F64ChannelCounts, f64>::new();
      let solver = argmin::solver::quasinewton::LBFGS::new(linesearch, 200);
      let result = Executor::new(problem, solver)
        .configure(|state| state.max_iters(10))
        .run()
        .unwrap();
      println!("{}", result);
    }
  };
  // TODO:
  // executor.add_observer(
  //   argmin::core::observers::Observers::new(),
  //   argmin::core::observers::ObserverMode::Every(4),
  // )
  // let _best = result.state().get_best_param().unwrap();
  // let _best_cost = result.state().get_best_cost();
}
