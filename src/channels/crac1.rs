use super::base::{HasTransitionMatrix, IonChannelCat, Named};
use crate::constants;
use nalgebra::{Matrix2, SVector, Vector2};

pub type CRAC1IonChannelCat = IonChannelCat<2>;
impl HasTransitionMatrix<2> for CRAC1IonChannelCat {
  const conductance: f64 = 5.0;

  fn initial_state() -> SVector<f64, 2> {
    return Vector2::new(0.0, 1.0);
  }

  fn transition_matrix(&self, v: f64) -> Matrix2<f64> {
    // let g = 24e-15;
    if v <= 0.0 {
      let tau_o = 41.0 * (v / 110.0).exp();
      let tau_c = 19.0 * (v / 48.0).exp();
      let alpha = 1.0 / tau_c * constants::dt * 1e3;
      let beta = 1.0 / tau_o * constants::dt * 1e3;
      return Matrix2::new(1.0 - alpha, beta, alpha, 1.0 - beta);
    } else {
      return Matrix2::new(1.0, 1.0, 0.0, 0.0);
    }
  }
}
impl Named for CRAC1IonChannelCat {
  fn name() -> String {
    return String::from("CRACM1");
  }
}
