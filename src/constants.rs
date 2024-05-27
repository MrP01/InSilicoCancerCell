#![allow(non_upper_case_globals)]

pub const slowest_dt: f64 = 5e-7; // initial timestep
pub const delta_tolerance: f64 = 1e-6; // how much of a change in \Delta state the system is aiming for (squared)
pub const Ca_i: f64 = 0.0647e-6; // initial calcium concentration
pub const F: f64 = 96485.3329; // As/mol
pub const R: f64 = 8.3144598; // kgm^2/s^2molK
pub const T: f64 = 293.0; // K, TODO: 20°C good?

pub enum IonType {
  Kalium,
  Calcium,
  Chlorine,
}

pub fn reversal_potential(ion: IonType) -> f64 {
  match ion {
    IonType::Kalium => -77.4e-3,  // reversal potential K,
    IonType::Calcium => 95.6e-3,  // reversal potential Ca,
    IonType::Chlorine => -7.9e-3, // reversal potential Cl,
  }
}
