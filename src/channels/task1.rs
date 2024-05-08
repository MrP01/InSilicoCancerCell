use nalgebra::Matrix3;

use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};

define_ion_channel!(
  Task1IonChannelCat,
  "Task1",
  3,      // number of states
  16e-12, // conductance
  (2)     // states which count towards the current
);

impl HasTransitionMatrix<3> for Task1IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix3<f64> {
    // Constants - given in s^-1
    let a0 = 13.3;
    let za = -0.106;
    let b0 = 17.6;
    let zb = 0.105;
    let x0 = 108.0;
    let zx = -0.095;
    let d0 = 9.7;
    let zd = 0.307;

    // Transition rates
    let factor = v * constants::F / (constants::R * constants::T);
    let a_rate = a0 * (za * factor).exp();
    let b_rate = b0 * (-zb * factor).exp();
    let x_rate = x0 * (zx * factor).exp();
    let d_rate = d0 * (-zd * factor).exp();

    // Transition probabilities
    let a_prob = a_rate * constants::dt;
    let b_prob = b_rate * constants::dt;
    let x_prob = x_rate * constants::dt;
    let d_prob = d_rate * constants::dt;

    #[rustfmt::skip]
    return Matrix3::from_row_slice(&[
      1.0 - a_prob, b_prob, 0.0,
      a_prob, 1.0 - b_prob - x_prob, d_prob,
      0.0, x_prob, 1.0 - d_prob
    ]);
  }
}
