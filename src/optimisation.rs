use argmin::core::{CostFunction, Error, Executor, Gradient};
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
      currents: [1; 9].map(|_| vec![]),
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
  _precomputed_currents: Option<nalgebra::DMatrix<f64>>,
}
type F64ChannelCounts = SVector<f64, N_CHANNEL_TYPES>;

impl ChannelCountsProblem {
  fn new(fit_to: PatchClampData) -> Self {
    Self {
      fit_to,
      _precomputed_currents: None,
    }
  }
  fn precompute_single_channel_currents(&mut self) {
    let pulse_protocol = DefaultPulseProtocol {};
    let mut cell = A549CancerCell::new();
    let mut recorded = SingleChannelCurrentRecord::empty();
    cell.simulate(pulse_protocol, &mut recorded, self.fit_to.current.len());
    self._precomputed_currents = Some(nalgebra::DMatrix::from_columns(
      &recorded.currents.map(|c| nalgebra::DVector::from(c)),
    ));
    log::info!("Finished pre-computation of single-channel currents.");
  }
}

impl CostFunction for ChannelCountsProblem {
  type Param = F64ChannelCounts;
  type Output = f64;
  fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
    Ok(evaluate_current_match(
      &self.fit_to,
      self._precomputed_currents.as_ref().unwrap() * param,
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

pub enum InSilicoOptimiser {
  ParticleSwarm,
  SteepestDescent,
  LBFGS,
}

pub fn find_best_fit_for(data: PatchClampData, using: InSilicoOptimiser) {
  let mut problem = ChannelCountsProblem::new(data);
  problem.precompute_single_channel_currents();
  match using {
    InSilicoOptimiser::ParticleSwarm => {
      let solver = argmin::solver::particleswarm::ParticleSwarm::new(
        (
          [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0].into(),
          [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0].into(),
        ),
        200,
      );
      let result = Executor::new(problem, solver)
        .configure(|state| state.max_iters(10))
        .run()
        .unwrap();
      println!("{}", result);
    }
    InSilicoOptimiser::SteepestDescent => {
      let linesearch =
        argmin::solver::linesearch::HagerZhangLineSearch::<F64ChannelCounts, F64ChannelCounts, f64>::new();
      let solver = argmin::solver::gradientdescent::SteepestDescent::new(linesearch);
      let result = Executor::new(problem, solver)
        .configure(|state| state.max_iters(10))
        .run()
        .unwrap();
      println!("{}", result);
    }
    InSilicoOptimiser::LBFGS => {
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
