use nalgebra::DVector;

use crate::{
  channels::{self, base::IsChannel},
  constants,
  patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol},
  pulseprotocol::{ProtocolGenerator, PulseProtocol},
};

pub trait SimulationRecorder {
  fn record(&mut self, cell: &A549CancerCell, voltage: f64);
}

pub struct TotalCurrentRecord {
  pub current: Vec<f64>,
}

impl TotalCurrentRecord {
  pub fn empty() -> Self {
    Self { current: vec![] }
  }

  pub fn as_dvec(self) -> DVector<f64> {
    DVector::<f64>::from_vec(self.current)
  }
}

impl SimulationRecorder for TotalCurrentRecord {
  fn record(&mut self, cell: &A549CancerCell, voltage: f64) {
    self
      .current
      .push(cell.channels().iter().map(|c| c.current(voltage)).sum());
  }
}

#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct A549CancerCell {
  crac1_channel: channels::crac1::CRAC1IonChannelCat,
  kv13_channel: channels::kv13::KV13IonChannelCat,
  kv31_channel: channels::kv31::KV31IonChannelCat,
  kv34_channel: channels::kv34::KV34IonChannelCat,
  kv71_channel: channels::kv71::KV71IonChannelCat,
  kca11_channel: channels::kca11::KCa11IonChannelCat,
  kca31_channel: channels::kca31::KCa31IonChannelCat,
  clc2_channel: channels::clc2::CLC2IonChannelCat,
  task1_channel: channels::task1::Task1IonChannelCat,
}

impl A549CancerCell {
  pub fn channels(&self) -> Vec<&dyn IsChannel> {
    return vec![
      &self.crac1_channel,
      &self.kv13_channel,
      &self.kv31_channel,
      &self.kv34_channel,
      &self.kv71_channel,
      &self.kca11_channel,
      &self.kca31_channel,
      &self.clc2_channel,
      &self.task1_channel,
    ];
  }
  pub fn channels_mut(&mut self) -> Vec<&mut dyn IsChannel> {
    return vec![
      &mut self.crac1_channel,
      &mut self.kv13_channel,
      &mut self.kv31_channel,
      &mut self.kv34_channel,
      &mut self.kv71_channel,
      &mut self.kca11_channel,
      &mut self.kca31_channel,
      &mut self.clc2_channel,
      &mut self.task1_channel,
    ];
  }

  pub fn simulate(&mut self, pulse_protocol: PulseProtocol, recorder: &mut impl SimulationRecorder) {
    let mut total_time = 0.0;
    for channel in self.channels_mut() {
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
        for channel in self.channels_mut() {
          channel.update_state(step.voltage);
        }
        if n % constants::STEPS_PER_MEASUREMENT == 0 {
          recorder.record(self, step.voltage);
        }
        time += constants::dt;
      }
      total_time += time;
    }
    log::info!("Total simulation time: {total_time:.3} s");
  }
}

pub fn evaluate_match(measurements: PatchClampData, simulation: TotalCurrentRecord) -> f64 {
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
      crac1_channel: channels::crac1::CRAC1IonChannelCat::new(),
      kv13_channel: channels::kv13::KV13IonChannelCat::new(),
      kv31_channel: channels::kv31::KV31IonChannelCat::new(),
      kv34_channel: channels::kv34::KV34IonChannelCat::new(),
      kv71_channel: channels::kv71::KV71IonChannelCat::new(),
      kca11_channel: channels::kca11::KCa11IonChannelCat::new(),
      kca31_channel: channels::kca31::KCa31IonChannelCat::new(),
      clc2_channel: channels::clc2::CLC2IonChannelCat::new(),
      task1_channel: channels::task1::Task1IonChannelCat::new(),
    };
  }

  pub fn evaluate(&mut self, protocol: PatchClampProtocol, phase: CellPhase) -> f64 {
    let measurements = PatchClampData::load(protocol, phase).unwrap();
    let pulse_protocol = PulseProtocol::default();
    let mut recorded = TotalCurrentRecord::empty();
    self.simulate(pulse_protocol, &mut recorded);
    return evaluate_match(measurements, recorded);
  }
}
