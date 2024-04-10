use nalgebra::DVector;

use crate::{
  channels::{
    base::{Constructable, Named, Simulatable},
    crac1, kv71,
  },
  constants,
  patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol},
  pulseprotocol::{ProtocolGenerator, PulseProtocol},
};

pub struct MembraneCurrentThroughput {
  current: Vec<f64>,
}

impl MembraneCurrentThroughput {
  pub fn empty() -> Self {
    Self { current: vec![] }
  }

  pub fn as_dvec(self) -> DVector<f64> {
    DVector::<f64>::from_vec(self.current)
  }
}

#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct A549CancerCell {
  crac1_channel: crac1::CRAC1IonChannelCat,
  kv71_channel: kv71::KV71IonChannelCat,
}

impl A549CancerCell {
  pub fn channels(&mut self) -> Vec<Box<&mut dyn Simulatable>> {
    return vec![Box::new(&mut self.crac1_channel), Box::new(&mut self.kv71_channel)];
  }

  pub fn simulate(&mut self, pulse_protocol: PulseProtocol) -> MembraneCurrentThroughput {
    let mut total_time = 0.0;
    let mut recorded = MembraneCurrentThroughput::empty();
    log::info!(
      "Simulating {} with {} channels.",
      crac1::CRAC1IonChannelCat::name(),
      self.crac1_channel.n_channels
    );
    log::info!(
      "Simulating {} with {} channels.",
      kv71::KV71IonChannelCat::name(),
      self.kv71_channel.n_channels
    );
    for step in pulse_protocol {
      log::info!(
        "Pulse protocol step {} ({:.3} V) for {:.3} s",
        step.label,
        step.voltage,
        step.duration
      );
      let mut time: f64 = 0.0;
      while time < step.duration {
        let mut total_current = 0.0;
        for channel in self.channels() {
          channel.update_state(step.voltage);
          total_current += channel.current(step.voltage);
        }
        recorded.current.push(total_current);
        time += constants::dt;
      }
      total_time += time;
    }
    log::info!("Total simulation time: {total_time:.3} s");
    return recorded;
  }
}

pub fn evaluate_match(measurements: PatchClampData, simulation: MembraneCurrentThroughput) -> f64 {
  let rows = measurements.current.len();
  let error = (simulation.as_dvec().rows_range(0..rows) - measurements.current).norm_squared();
  log::info!("Simulation match with measurements: {:.3}", error);
  return error;
}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pyo3::pymethods)]
impl A549CancerCell {
  #[cfg_attr(feature = "pyo3", staticmethod)]
  pub fn new() -> A549CancerCell {
    return A549CancerCell {
      crac1_channel: crac1::CRAC1IonChannelCat::new(),
      kv71_channel: kv71::KV71IonChannelCat::new(),
    };
  }

  pub fn evaluate(&mut self, protocol: PatchClampProtocol, phase: CellPhase) -> f64 {
    let measurements = PatchClampData::load(protocol, phase).unwrap();
    let pulse_protocol = PulseProtocol::default();
    let simulation = self.simulate(pulse_protocol);
    return evaluate_match(measurements, simulation);
  }
}
