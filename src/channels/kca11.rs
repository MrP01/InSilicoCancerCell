use super::base::HasTransitionMatrix;
use crate::{constants, define_ion_channel};

pub type Matrix10<T> = nalgebra::Matrix<T, nalgebra::U10, nalgebra::U10, nalgebra::ArrayStorage<T, 10, 10>>;

define_ion_channel!(
  KCa11IonChannelCat,
  "KCa11",
  10,
  250e-12,
  (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
);

impl HasTransitionMatrix<10> for KCa11IonChannelCat {
  fn transition_matrix(&self, v: f64) -> Matrix10<f64> {
    // Constants
    let on_rate = 1000.0; // Mol^-1s^1
    let ca_i = constants::Ca_i * 1e6 * on_rate;
    let c0 = 1.8225;
    let c1 = 1.215;
    let c2 = 0.855;
    let c3 = 0.49;
    let c4 = 0.11;
    let d = 200.0;
    let b = 36.0;
    let a4 = 0.396;
    let ko_u = 1.5 * on_rate;
    let kc_u = 13.5 * on_rate;
    let a0 = 0.001;
    let a1 = 0.006;
    let a2 = 0.038;
    let a3 = 0.196;

    // Transition probabilities
    let a_rate = c0 * (-v * 1e3 / d).exp();
    let b_rate = a0 * (v * 1e3 / b).exp();
    let c_rate = c1 * (-v * 1e3 / d).exp();
    let d_rate = a1 * (v * 1e3 / b).exp();
    let e_rate = c2 * (-v * 1e3 / d).exp();
    let f_rate = a2 * (v * 1e3 / b).exp();
    let g_rate = c3 * (-v * 1e3 / d).exp();
    let h_rate = a3 * (v * 1e3 / b).exp();
    let i_rate = c4 * (-v * 1e3 / d).exp();
    let j_rate = a4 * (v * 1e3 / b).exp();

    // Transition probability alpha = rate constants * ms
    // TODO: I removed the * 1e3 here (compare the KCa_1_1.m file), which is correct?
    let a = a_rate * constants::dt;
    let b = b_rate * constants::dt;
    let c = c_rate * constants::dt;
    let d = d_rate * constants::dt;
    let e = e_rate * constants::dt;
    let f = f_rate * constants::dt;
    let g = g_rate * constants::dt;
    let h = h_rate * constants::dt;
    let i = i_rate * constants::dt;
    let j = j_rate * constants::dt;

    let ko = ko_u * constants::dt;
    let kc = kc_u * constants::dt;
    let ca_i_dt = ca_i * constants::dt;

    #[rustfmt::skip]
    return Matrix10::from_row_slice(&[
      1.0-4.0 * ca_i_dt - b, kc, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, a,
      4.0 * ca_i_dt, 1.0- kc -3.0 * ca_i_dt - d, 2.0 * kc, 0.0, 0.0, 0.0, 0.0, 0.0, c, 0.0,
      0.0, 3.0 * ca_i_dt, 1.0-2.0 * kc -2.0 * ca_i_dt - f, 3.0 * kc, 0.0, 0.0, 0.0, e, 0.0, 0.0,
      0.0, 0.0, 2.0 * ca_i_dt, 1.0-3.0 * kc - ca_i_dt - h, 4.0 * kc, 0.0, g, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, ca_i_dt, 1.0-4.0 * kc - j, i, 0.0, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, 0.0, j, 1.0 - i - 4.0 * ko, ca_i_dt, 0.0, 0.0, 0.0,
      0.0, 0.0, 0.0, h, 0.0, 4.0 * ko, 1.0 - g - ca_i_dt -3.0 * ko, 2.0 * ca_i_dt, 0.0, 0.0,
      0.0, 0.0, f, 0.0, 0.0, 0.0, 3.0 * ko, 1.0 - 2.0 * ca_i_dt - e - 2.0 * ko, 3.0 * ca_i_dt, 0.0,
      0.0, d, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0 * ko, 1.0 - c - 3.0 * ca_i_dt - ko, 4.0 * ca_i_dt,
      b, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ko, 1.0 - 4.0 * ca_i_dt - a
    ]);
  }
}
