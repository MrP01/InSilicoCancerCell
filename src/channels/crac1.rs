use super::base::{HasTransitionMatrix, IonChannelCat, Named};

pub type CRAC1IonChannelCat = IonChannelCat<2>;
impl HasTransitionMatrix<2> for CRAC1IonChannelCat {
  fn transition_matrix(&self) -> nalgebra::Matrix2<f32> {
    return nalgebra::Matrix2::new(1.0, 2.0, 3.0, 4.0);
  }
}
impl Named for CRAC1IonChannelCat {
  fn name() -> String {
    return String::from("CRAC1");
  }
}
