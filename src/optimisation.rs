use argmin::core::{CostFunction, Error, Executor, Gradient, State};
use argmin::solver::linesearch::HagerZhangLineSearch;
use nalgebra::Vector3;

fn target(params: &Vector3<f64>) -> f64 {
  4.0 * params[0].powi(2) + 7.0 * params[1] + 22.0
}

struct MyProblem {}

// Implement `CostFunction` for `MyProblem`
impl CostFunction for MyProblem {
  type Param = Vector3<f64>;
  type Output = f64;
  fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
    Ok(target(param))
  }
}

impl Gradient for MyProblem {
  type Param = Vector3<f64>;
  type Gradient = Vector3<f64>;
  fn gradient(&self, _param: &Self::Param) -> Result<Self::Gradient, Error> {
    Ok([1.0, 2.0, 3.0].into())
  }
}

fn optimise() {
  // Create new instance of cost function
  let cost = MyProblem {};

  // Define initial parameter vector
  let init_param: Vector3<f64> = Vector3::from_element(2.0);

  // Set up line search needed by `SteepestDescent`
  let linesearch = HagerZhangLineSearch::<Vector3<f64>, Vector3<f64>, f64>::new();

  // Set up solver -- `SteepestDescent` requires a linesearch
  let solver = argmin::solver::gradientdescent::SteepestDescent::new(linesearch);

  // Create an `Executor` object
  let res = Executor::new(cost, solver)
    // Via `configure`, one has access to the internally used state.
    // This state can be initialized, for instance by providing an
    // initial parameter vector.
    // The maximum number of iterations is also set via this method.
    // In this particular case, the state exposed is of type `IterState`.
    // The documentation of `IterState` shows how this struct can be
    // manipulated.
    // Population based solvers use `PopulationState` instead of
    // `IterState`.
    .configure(|state| {
      state
        // Set initial parameters (depending on the solver,
        // this may be required)
        .param(init_param)
        // Set maximum iterations to 10
        // (optional, set to `std::u64::MAX` if not provided)
        .max_iters(10)
        // Set target cost. The solver stops when this cost
        // function value is reached (optional)
        .target_cost(0.0)
    })
    // run the solver on the defined problem
    .run()
    .unwrap();

  // print result
  println!("{}", res);

  // Extract results from state

  // Best parameter vector
  let _best = res.state().get_best_param().unwrap();

  // Cost function value associated with best parameter vector
  let _best_cost = res.state().get_best_cost();

  // Check the execution status
  let _termination_status = res.state().get_termination_status();

  // Optionally, check why the optimizer terminated (if status is terminated)
  let _termination_reason = res.state().get_termination_reason();

  // Time needed for optimization
  let _time_needed = res.state().get_time().unwrap();

  // Total number of iterations needed
  let _num_iterations = res.state().get_iter();

  // Iteration number where the last best parameter vector was found
  let _num_iterations_best = res.state().get_last_best_iter();

  // Number of evaluation counts per method (Cost, Gradient)
  let _function_evaluation_counts = res.state().get_func_counts();
}
