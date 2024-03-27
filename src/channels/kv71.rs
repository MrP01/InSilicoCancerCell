use super::base::{HasTransitionMatrix, IonChannelCat, Named};
use crate::constants;
use nalgebra::{Matrix5, Vector5};

pub type KV71IonChannelCat = IonChannelCat<5>;
impl HasTransitionMatrix<5> for KV71IonChannelCat {
  const conductance: f64 = 3.2e-12;

  fn initial_state() -> Vector5<f64> {
    return Vector5::new(0.0, 0.0, 1.0, 0.0, 0.0);
  }

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

impl Named for KV71IonChannelCat {
  fn name() -> String {
    return String::from("Kv_7_1");
  }
}

// fn kv_7_1(v: f64) -> StateSpace {
// let a = system_matrix_7_1(0.0, 1.0); // Assuming dt = 1.0 for simplicity
// let b = DVector::from_vec(vec![0.0, 0.0, 0.0, 0.0, 0.0]);
// let c = DVector::from_vec(vec![0.0, 0.0, 1.0, 1.0, 0.0]);
// let d = 0.0;
// StateSpace { a, b, c, d, g_k: g }
// }
