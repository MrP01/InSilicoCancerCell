use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};
pub type Matrix7<T> = nalgebra::Matrix<T, nalgebra::U7, nalgebra::U7, nalgebra::ArrayStorage<T, 7, 7>>;

define_ion_channel!(
  KV13IonChannelCat,
  "Kv13",
  7,                                   // number of states
  15e-12,                              // conductance
  (0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0), // initial state
  (5)                                  // states which count towards the current
);

impl HasTransitionMatrix<7> for KV13IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix7<f64> {
    // Constants
    let a = 0.488;
    let b = 0.043;
    let c = 0.003;
    let d = 0.00008;
    let const_a = 280.035;
    let const_b = 1.648;
    let m = 27.530;
    let n = 17.528;
    let p = 174.961;
    let q = 1016.330;

    // Transition rates
    let alpha_rate = a * (v * 1e3 / m).exp();
    let beta_rate = b * (-v * 1e3 / n).exp();
    let gamma_rate = c * (v * 1e3 / p).exp();
    let phi_rate = d * (-v * 1e3 / q).exp();

    // Transition probabilities = rate constants * ms
    // TODO: I removed the * 1e3 here (compare the Kv_1_3.m file), which is correct?
    let alpha = alpha_rate * constants::dt;
    let beta = beta_rate * constants::dt;
    let gamma = gamma_rate * constants::dt;
    let phi = phi_rate * constants::dt;
    let a_prob = const_a * constants::dt;
    let b_prob = const_b * constants::dt;

    #[rustfmt::skip]
    return Matrix7::from_row_slice(&[
      1.0 - 4.0 * alpha, beta, 0.0, 0.0, 0.0, 0.0, 0.0,
      4.0 * alpha, 1.0 - 3.0 * alpha - beta, 2.0 * beta, 0.0, 0.0, 0.0, 0.0,
      0.0, 3.0 * alpha, 1.0 - 2.0 * alpha - 2.0 * beta, 3.0 * beta, 0.0, 0.0, 0.0,
      0.0, 0.0, 2.0 * alpha, 1.0 - alpha - 3.0 * beta, 4.0 * beta, 0.0, 0.0,
      0.0, 0.0, 0.0, alpha, 1.0 - 4.0 * beta - a_prob, b_prob, 0.0,
      0.0, 0.0, 0.0, 0.0, a_prob, 1.0 - gamma - b_prob, phi,
      0.0, 0.0, 0.0, 0.0, 0.0, gamma, 1.0 - phi
    ]);
  }
}
