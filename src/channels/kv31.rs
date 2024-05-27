use nalgebra::Matrix6;

use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};

define_ion_channel!(
  KV31IonChannelCat,
  "Kv31",
  6,                          // number of states
  constants::IonType::Kalium, // ion type
  40e-3,                      // conductance
  (5)                         // states which count towards the current
);

impl HasTransitionMatrix<6> for KV31IonChannelCat {
  fn transition_matrix(&self, v: f64, dt: f64) -> Matrix6<f64> {
    // TODO: why are these re-evaluated below? Cf. Kv_3_1.m
    // a = 0.925973;
    // b = 35.893170;
    // c = 0.194083;
    // d = 279.735292;
    // alpha_rate = 40.558628;
    // beta_rate = 389.270005;

    // Constants
    let a = 1.027813;
    let b = 35.984422;
    let c = 0.212136;
    let d = 187.126824;
    let alpha_rate = 44.050331;
    let beta_rate = 368.999741;

    // Transition rates
    let a_rate = a * (v * 1e3 / b).exp();
    let c_rate = c * (-v * 1e3 / d).exp();

    // Transition probabilities
    // TODO: I removed the 1e3 factor
    let a_prob = a_rate * dt;
    let c_prob = c_rate * dt;
    let alpha = alpha_rate * dt;
    let beta = beta_rate * dt;

    #[rustfmt::skip]
    return Matrix6::from_row_slice(&[
      1.0 - 4.0 * a_prob, c_prob, 0.0, 0.0, 0.0, 0.0,
      4.0 * a_prob, 1.0 - 3.0 * a_prob - c_prob, 2.0 * c_prob, 0.0, 0.0, 0.0,
      0.0, 3.0 * a_prob, 1.0 - 2.0 * a_prob - 2.0 * c_prob, 3.0 * c_prob, 0.0, 0.0,
      0.0, 0.0, 2.0 * a_prob, 1.0 - a_prob - 3.0 * c_prob, 4.0 * c_prob, 0.0,
      0.0, 0.0, 0.0, a_prob, 1.0 - 4.0 * c_prob - alpha, beta,
      0.0, 0.0, 0.0, 0.0, alpha, 1.0 - beta
    ]);
  }
}
