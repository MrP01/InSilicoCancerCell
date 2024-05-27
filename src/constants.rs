#![allow(non_upper_case_globals)]

pub const dt: f64 = 5e-7; // timestep
pub const Ca_i: f64 = 0.0647e-6; // initial calcium concentration
pub const F: f64 = 96485.3329; // As/mol
pub const R: f64 = 8.3144598; // kgm^2/s^2molK
pub const T: f64 = 273.0; // K, TODO: 0°C? melting point?

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
