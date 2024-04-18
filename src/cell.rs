use std::collections::HashMap;

use nalgebra::DVector;

use crate::{
  channels::{base::IsChannel, crac1, kv13, kv34, kv71},
  constants,
  patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol},
  pulseprotocol::{ProtocolGenerator, PulseProtocol},
};

pub struct MembraneCurrentThroughput {
  pub current: Vec<f64>,
  pub states: HashMap<String, Vec<f64>>,
}

impl MembraneCurrentThroughput {
  pub fn empty() -> Self {
    Self {
      current: vec![],
      states: HashMap::new(),
    }
  }

  pub fn as_dvec(self) -> DVector<f64> {
    DVector::<f64>::from_vec(self.current)
  }
}

#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct A549CancerCell {
  crac1_channel: crac1::CRAC1IonChannelCat,
  kv13_channel: kv13::KV13IonChannelCat,
  kv34_channel: kv34::KV34IonChannelCat,
  kv71_channel: kv71::KV71IonChannelCat,
}

impl A549CancerCell {
  pub fn channels(&mut self) -> Vec<&mut dyn IsChannel> {
    return vec![
      &mut self.crac1_channel,
      &mut self.kv13_channel,
      &mut self.kv34_channel,
      &mut self.kv71_channel,
    ];
  }

  pub fn simulate(&mut self, pulse_protocol: PulseProtocol) -> MembraneCurrentThroughput {
    let mut total_time = 0.0;
    let mut recorded = MembraneCurrentThroughput::empty();
    for channel in self.channels() {
      log::info!("{}", channel.display_me());
    }
    for (n, step) in pulse_protocol.enumerate() {
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
        if n % constants::STEPS_PER_MEASUREMENT == 0 {
          recorded.current.push(total_current);
          // for channel in self.channels() {
          // recorded
          //   .states
          //   .insert(channel.namename(), channel.state.iter().cloned().collect());
          // }
        }
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
      kv13_channel: kv13::KV13IonChannelCat::new(),
      kv34_channel: kv34::KV34IonChannelCat::new(),
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
