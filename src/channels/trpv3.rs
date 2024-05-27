use nalgebra::Matrix1;

use super::base::HasTransitionMatrix;
use crate::{constants::IonType, define_ion_channel};

define_ion_channel!(
  TRPV3IonChannelCat,
  "TRPV3",
  1,                           // number of states
  IonType::Calcium, // ion type
  48e-3,                       // conductance
  (0)                          // states which count towards the current
);

impl HasTransitionMatrix<1> for TRPV3IonChannelCat {
  fn transition_matrix(&self, _v: f64, _dt: f64) -> Matrix1<f64> {
    Matrix1::new(1.0)
  }
}
