use nalgebra::Matrix4;

use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};

define_ion_channel!(KCa31IonChannelCat, "KCa31", 4, 11e-12, (0.0, 0.0, 0.0, 0.0));

impl HasTransitionMatrix<4> for KCa31IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix4<f64> {
    // Constants
    let k12 = 27.0;
    let k23 = 5425.0;
    let k21 = 34.0;
    let k35 = 34.0;
    let k32 = 190.0;
    let k53 = 20.0;

    // Transition probabilities
    let k12_rate = k12 * constants::Ca_i * 1e6;
    let k23_rate = k23 * constants::Ca_i * 1e6;
    let k21_rate = k21;
    let k35_rate = k35;
    let k32_rate = k32;
    let k53_rate = k53;

    // % transition probability = rate constants * s
    let k12_prob = k12_rate * constants::dt;
    let k23_prob = k23_rate * constants::dt;
    let k21_prob = k21_rate * constants::dt;
    let k35_prob = k35_rate * constants::dt;
    let k32_prob = k32_rate * constants::dt;
    let k53_prob = k53_rate * constants::dt;

    // #[rustfmt::skip]
    // return Matrix4::from_row_slice(&[
    //   1.0 - k12_prob, k21_prob, 0.0, 0.0,
    //   k12_prob, 1.0 - k21_prob - k23_prob, k32_prob, 0.0,
    //   0.0, k23_prob, 1.0 - k32_prob - k35_prob, k53_prob,
    //   0.0, 0.0, k35_prob, 1.0 - k53_prob
    // ]);
    return Matrix4::zeros();
  }
}
