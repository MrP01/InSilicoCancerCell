use nalgebra::Matrix5;

use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};

define_ion_channel!(
  KV71IonChannelCat,
  "Kv71",
  5,                         // number of states
  3.2e-12,                   // conductance
  (1.0, 0.0, 0.0, 0.0, 0.0), // initial state
  (2, 3)                     // states which count towards the current
);

impl HasTransitionMatrix<5> for KV71IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix5<f64> {
    // Constants
    let factor = v * constants::F / (constants::R * constants::T);
    let a_rate = 4.6 * (0.47 * factor).exp();
    let b_rate = 33.0 * (-0.35 * factor).exp();
    let a2_rate = 24.0 * (0.006 * factor).exp();
    let b2_rate = 19.0 * (-0.007 * factor).exp();
    let eps_rate = 4.6 * (0.8 * factor).exp();
    let delta_rate = 1.4 * (-0.7 * factor).exp();
    let lambda = 142.0;
    let micro = 52.0;

    // Transition probabilities
    let a_prob = a_rate * constants::dt;
    let b_prob = b_rate * constants::dt;
    let a2_prob = a2_rate * constants::dt;
    let b2_prob = b2_rate * constants::dt;
    let eps_prob = eps_rate * constants::dt;
    let delta_prob = delta_rate * constants::dt;
    let lambda_prob = lambda * constants::dt;
    let micro_prob = micro * constants::dt;

    #[rustfmt::skip]
    return Matrix5::from_row_slice(&[
      1.0 - a_prob, b_prob, 0.0, 0.0, 0.0,
      a_prob, 1.0 - a2_prob - b_prob, b2_prob, 0.0, 0.0,
      0.0, a2_prob, 1.0 - b2_prob - eps_prob, delta_prob, 0.0,
      0.0, 0.0, eps_prob, 1.0 - delta_prob - lambda_prob, micro_prob,
      0.0, 0.0, 0.0, lambda_prob, 1.0 - micro_prob,
    ]);
  }
}
