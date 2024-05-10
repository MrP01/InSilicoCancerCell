#[cfg(all(debug_assertions, feature = "pause-each-step"))]
use std::io::{BufRead, Write};

use nalgebra::DVector;

use crate::{
  channels::{self, base::IsChannel},
  constants,
  patchclampdata::{CellPhase, PatchClampData},
  pulseprotocol::{ProtocolGenerator, RepeatingGenerator},
};
#[cfg(feature = "default")]
use crate::{patchclampdata::PatchClampProtocol, pulseprotocol::DefaultPulseProtocol};

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
    vec![
      &self.crac1_channel,
      &self.kv13_channel,
      &self.kv31_channel,
      &self.kv34_channel,
      &self.kv71_channel,
      &self.kca11_channel,
      &self.kca31_channel,
      &self.clc2_channel,
      &self.task1_channel,
    ]
  }
  pub fn channels_mut(&mut self) -> Vec<&mut dyn IsChannel> {
    vec![
      &mut self.crac1_channel,
      &mut self.kv13_channel,
      &mut self.kv31_channel,
      &mut self.kv34_channel,
      &mut self.kv71_channel,
      &mut self.kca11_channel,
      &mut self.kca31_channel,
      &mut self.clc2_channel,
      &mut self.task1_channel,
    ]
  }

  pub fn simulate(
    &mut self,
    pulse_protocol: impl ProtocolGenerator + RepeatingGenerator,
    recorder: &mut impl SimulationRecorder,
    min_points: usize,
  ) {
    let total_duration = pulse_protocol.single_duration();
    let steps_per_measurement = ((total_duration / constants::dt) / (min_points as f64)).floor() as usize;
    if steps_per_measurement == 0 {
      panic!("constants::dt is too small for the supplied amount of minimum record points!");
    }
    log::info!(
      "Starting simulation. Duration according to pulse protocol: {:.3} s. Recording every {} iterations.",
      total_duration,
      steps_per_measurement
    );
    let mut n = 0;
    let mut total_time = 0.0;
    for channel in self.channels_mut() {
      log::info!("{}", channel.display_me());
    }
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    for step in pulse_protocol.generator() {
      log::info!(
        "Pulse protocol step {:7} ({:6.3} V) for {:.3} s -> {:8.0} iterations",
        step.label,
        step.voltage,
        step.duration,
        step.duration / constants::dt
      );
      let mut time: f64 = 0.0;
      while time < step.duration {
        for channel in self.channels_mut() {
          channel.update_state(step.voltage);
        }
        if n % steps_per_measurement == 0 {
          recorder.record(self, step.voltage);
        }
        n += 1;
        time += constants::dt;
        #[cfg(all(debug_assertions, feature = "pause-each-step"))]
        {
          print!("Break (t = {time:.7}); press return to continue");
          std::io::stdout().flush().unwrap();
          std::io::stdin().lock().read_line(&mut String::new()).unwrap();
        }
      }
      total_time += time;
    }
    log::info!("Total time passed from the cell perspective: {total_time:.3} s");
    #[cfg(not(target_arch = "wasm32"))]
    {
      let runtime = start.elapsed().as_secs_f64();
      log::info!("Total simulation runtime: {runtime:.3} s");
    }
  }
}

pub fn evaluate_match(measurements: PatchClampData, simulation: TotalCurrentRecord) -> f64 {
  log::info!(
    "Collected data: {} points from simulation, {} points from measurements.",
    simulation.current.len(),
    measurements.current.len()
  );
  let rows = measurements.current.len();
  let error = (simulation.as_dvec().rows_range(0..rows) - measurements.current).norm_squared();
  log::info!("Simulation match with measurements: {:.3}", error);
  error
}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pyo3::pymethods)]
impl A549CancerCell {
  #[cfg_attr(feature = "pyo3", staticmethod)]
  pub fn new() -> A549CancerCell {
    A549CancerCell {
      crac1_channel: channels::crac1::CRAC1IonChannelCat::new(),
      kv13_channel: channels::kv13::KV13IonChannelCat::new(),
      kv31_channel: channels::kv31::KV31IonChannelCat::new(),
      kv34_channel: channels::kv34::KV34IonChannelCat::new(),
      kv71_channel: channels::kv71::KV71IonChannelCat::new(),
      kca11_channel: channels::kca11::KCa11IonChannelCat::new(),
      kca31_channel: channels::kca31::KCa31IonChannelCat::new(),
      clc2_channel: channels::clc2::CLC2IonChannelCat::new(),
      task1_channel: channels::task1::Task1IonChannelCat::new(),
    }
  }

  pub fn set_langthaler_et_al_channel_counts(&mut self, phase: CellPhase) {
    match phase {
      CellPhase::G0 => {
        self.kv13_channel.n_channels = 22;
        self.kv31_channel.n_channels = 78;
        self.kv34_channel.n_channels = 5;
        self.kv71_channel.n_channels = 1350;
        self.kca11_channel.n_channels = 40;
        self.kca31_channel.n_channels = 77;
        self.task1_channel.n_channels = 19;
        self.crac1_channel.n_channels = 200;
        self.clc2_channel.n_channels = 13;
      }
      CellPhase::G1 => {
        self.kv13_channel.n_channels = 20;
        self.kv31_channel.n_channels = 90;
        self.kv34_channel.n_channels = 54;
        self.kv71_channel.n_channels = 558;
        self.kca11_channel.n_channels = 15;
        self.kca31_channel.n_channels = 63;
        self.task1_channel.n_channels = 10;
        self.crac1_channel.n_channels = 200;
        self.clc2_channel.n_channels = 11;
      }
    }
  }

  #[cfg(feature = "default")]
  pub fn evaluate(&mut self, protocol: PatchClampProtocol, phase: CellPhase) -> f64 {
    let measurements = PatchClampData::load(protocol, phase).unwrap();
    let pulse_protocol = DefaultPulseProtocol {};
    let mut recorded = TotalCurrentRecord::empty();
    self.simulate(pulse_protocol, &mut recorded, measurements.current.len());
    evaluate_match(measurements, recorded)
  }
}

impl Default for A549CancerCell {
  fn default() -> Self {
    Self::new()
  }
}
