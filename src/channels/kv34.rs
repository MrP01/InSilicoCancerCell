use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};
pub type Matrix7<T> = nalgebra::Matrix<T, nalgebra::U7, nalgebra::U7, nalgebra::ArrayStorage<T, 7, 7>>;

define_ion_channel!(
  KV34IonChannelCat,
  "Kv34",
  7,
  14e-12,
  (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
);

impl HasTransitionMatrix<7> for KV34IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix7<f64> {
    // Constants
    let a = 3352.0;
    let za = 0.06;
    let b = 3230.0;
    let zb = -0.8;
    let eps = 434.0;
    let zeps = 0.52;
    let phi = 70.0;
    let zphi = -0.37;
    let k1 = 55.0;
    let l1 = 0.8;

    // Transition rates
    let factor = v * constants::F / (constants::R * constants::T);
    let a_rate = a * (za * factor).exp();
    let b_rate = b * (zb * factor).exp();
    let eps_rate = eps * (zeps * factor).exp();
    let phi_rate = phi * (zphi * factor).exp();

    // Transition probabilities
    // transition probability = rate constants * ms
    let a_prob = a_rate * constants::dt;
    let b_prob = b_rate * constants::dt;
    let eps_prob = eps_rate * constants::dt;
    let phi_prob = phi_rate * constants::dt;
    let k1_prob = k1 * constants::dt;
    let l1_prob = l1 * constants::dt;

    #[rustfmt::skip]
    return Matrix7::from_row_slice(&[
      1.0 - 4.0 * a_prob, b_prob, 0.0, 0.0, 0.0, 0.0, 0.0,
      4.0 * a_prob, 1.0 - b_prob - 3.0 * a_prob, 2.0 * b_prob, 0.0, 0.0, 0.0, 0.0,
      0.0, 3.0 * a_prob, 1.0 - 2.0 * b_prob - 2.0 * a_prob, 3.0 * b_prob, 0.0, 0.0, 0.0,
      0.0, 0.0, 2.0 * a_prob, 1.0 - 3.0 * b_prob - a_prob, 4.0 * b_prob, 0.0, 0.0,
      0.0, 0.0, 0.0, a_prob, 1.0 - 4.0 * b_prob - eps_prob, phi_prob, 0.0,
      0.0, 0.0, 0.0, 0.0, eps_prob, 1.0 - phi_prob - k1_prob, l1_prob,
      0.0, 0.0, 0.0, 0.0, 0.0, k1_prob, 1.0 - l1_prob
    ]);
  }
}
