use argmin::core::State;
use argmin::core::{CostFunction, Error, Executor};
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
impl CostFunction for MyProblem {
  type Param = SVector<f64, N_CHANNEL_TYPES>;
  type Output = f64;
  fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
    let pulse_protocol = DefaultPulseProtocol {};
    let mut cell = A549CancerCell::new();
    cell.set_channel_counts(param.map(|x| x.round() as u32));
    let mut recorded = TotalCurrentRecord::empty();
    cell.simulate(pulse_protocol, &mut recorded, self.fit_to.current.len());
    Ok(evaluate_match(&self.fit_to, recorded))
  }
}

// impl Gradient for MyProblem {
//   type Param = Vector3<f64>;
//   type Gradient = Vector3<f64>;
//   fn gradient(&self, _param: &Self::Param) -> Result<Self::Gradient, Error> {
//     print!("g");
//     Ok([1.0, 2.0, 3.0].into())
//   }
// }

pub fn find_best_fit_for(data: PatchClampData) {
  let cost = MyProblem { fit_to: data };
  // let linesearch = HagerZhangLineSearch::<Vector3<f64>, Vector3<f64>, f64>::new();
  let solver = argmin::solver::particleswarm::ParticleSwarm::new(
    (
      [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0].into(),
      [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0].into(),
    ),
    200,
  );
  let result = Executor::new(cost, solver)
    .configure(|state| state.max_iters(10))
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
