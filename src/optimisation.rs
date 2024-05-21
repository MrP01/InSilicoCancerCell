use argmin::core::State;
use argmin::core::{CostFunction, Error, Executor, Gradient};
use nalgebra::SVector;

use crate::cell::evaluate_match;
use crate::cell::A549CancerCell;
use crate::cell::TotalCurrentRecord;
use crate::cell::N_CHANNEL_TYPES;
use crate::patchclampdata::PatchClampData;
use crate::pulseprotocol::DefaultPulseProtocol;

struct MyProblem {
  fit_to: PatchClampData,
}
type F64ChannelCounts = SVector<f64, N_CHANNEL_TYPES>;
impl CostFunction for MyProblem {
  type Param = F64ChannelCounts;
  type Output = f64;
  fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
    let pulse_protocol = DefaultPulseProtocol {};
    let mut cell = A549CancerCell::new();
    cell.set_channel_counts(param.map(|x| x.round() as u32).into());
    let mut recorded = TotalCurrentRecord::empty();
    cell.simulate(pulse_protocol, &mut recorded, self.fit_to.current.len());
    Ok(evaluate_match(&self.fit_to, recorded))
  }
}

impl Gradient for MyProblem {
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
  // SteepestDescent,
  // LBFGS,
}

pub fn find_best_fit_for(data: PatchClampData, using: InSilicoOptimiser) {
  let cost = MyProblem { fit_to: data };
  let executor = match using {
    InSilicoOptimiser::ParticleSwarm => {
      let solver = argmin::solver::particleswarm::ParticleSwarm::new(
        (
          [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0].into(),
          [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0].into(),
        ),
        200,
      );
      Executor::new(cost, solver).configure(|state| state.max_iters(10))
    } // InSilicoOptimiser::LBFGS => {
      //   let linesearch =
      //     argmin::solver::linesearch::HagerZhangLineSearch::<F64ChannelCounts, F64ChannelCounts, f64>::new();
      //   let solver = argmin::solver::quasinewton::LBFGS::new(linesearch, 200);
      //   Executor::new(cost, solver).configure(|state| state.max_iters(10))
      // }
  };
  let result = executor
    .add_observer(
      argmin::core::observers::Observers::new(),
      argmin::core::observers::ObserverMode::Every(4),
    )
    .run()
    .unwrap();

  println!("{}", result);
  let _best = result.state().get_best_param().unwrap();
  let _best_cost = result.state().get_best_cost();
}
