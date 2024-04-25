use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};
pub type Matrix12<T> = nalgebra::Matrix<T, nalgebra::U12, nalgebra::U12, nalgebra::ArrayStorage<T, 12, 12>>;

define_ion_channel!(
  CLC2IonChannelCat,
  "CLC2",
  12,
  2.8e-12,
  (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
);

impl HasTransitionMatrix<12> for CLC2IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix12<f64> {
    // Constants
    let factor = v * constants::F / (constants::R * constants::T);
    let a1 = 4.1 * (-0.57 * factor).exp();
    let b1 = 100.3 * (0.18 * factor).exp();
    let a2 = 6.4 * (-0.2 * factor).exp();
    let b2 = 10.6 * (0.32 * factor).exp();
    let l = 1.7 * (-0.3 * factor).exp() + 8.8 * (0.14 * factor).exp();
    let u = 4.9 * (0.18 * factor).exp();

    // Transition probabilities
    let a1 = a1 * constants::dt;
    let b1 = b1 * constants::dt;
    let a2 = a2 * constants::dt;
    let b2 = b2 * constants::dt;
    let u = u * constants::dt;
    let l = l * constants::dt;

    #[rustfmt::skip]
    return Matrix12::from_row_slice(&[
      1.0 - 2.0 * a1 - l, u, b1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
      l, 1.0 - u - 2.0 * a1, 0.0, b1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
      2.0 * a1, 0.0, 1.0 - b1 - l - a2 - a1, u, b2, 2.0 * b1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
      0.0, 2.0 * a1, l, 1.0 - b1 - u - a1 - a2, 0.0, 0.0, 2.0 * b1, b2, 0.0, 0.0, 0.0, 0.0,
      0.0, 0.0, a2, 0.0, 1.0 - b2 - l - a1, 0.0, 0.0, u, b1, 0.0, 0.0, 0.0,
      0.0, 0.0, a1, 0.0, 0.0, 1.0 - 2.0 * b1 - l - 2.0 * a2, u, 0.0, 2.0 * b2, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, a1, 0.0, l, 1.0 - 2.0 * b1 - u - 2.0 * a2, 0.0, 0.0, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, a2, l, 0.0, 0.0, 1.0 - b2 - u - a1, 0.0, b1, 0.0, 0.0,
      0.0, 0.0, 0.0, 0.0, a1, 2.0 * a2, 0.0, 0.0, 1.0 - b1 - b2 - l - a2, u, b2, 0.0,
      0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0 * a2, a1, l, 1.0 - b2 - b1 - u - a2, 0.0, b2,
      0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, a2, 0.0, 1.0 - b2 - l, u,
      0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, a2, l, 1.0 - b2 - u
    ]);
  }
}
