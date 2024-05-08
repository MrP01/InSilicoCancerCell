use nalgebra::Matrix2;

use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};

define_ion_channel!(
  CRAC1IonChannelCat,
  "CRACM1",
  2,      // number of states
  24e-15, // conductance
  (1)     // states which count towards the current
);

impl HasTransitionMatrix<2> for CRAC1IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix2<f64> {
    nalgebra::SVector::<f64, 2>::new(2.0, 3.0);
    if v <= 0.0 {
      let tau_o = 41.0 * (v / 110.0).exp();
      let tau_c = 19.0 * (v / 48.0).exp();
      let alpha = 1.0 / tau_c * constants::dt * 1e3;
      let beta = 1.0 / tau_o * constants::dt * 1e3;
      Matrix2::new(1.0 - alpha, beta, alpha, 1.0 - beta)
    } else {
      Matrix2::new(1.0, 1.0, 0.0, 0.0)
    }
  }
}
