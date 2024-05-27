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

  pub fn current_as_dvec(self) -> DVector<f64> {
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

pub const N_CHANNEL_TYPES: usize = 11;
pub type ChannelCounts = [u32; N_CHANNEL_TYPES];

#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct A549CancerCell {
  crac1_channel: channels::crac1::CRAC1IonChannelCat,
  trpc6_channel: channels::trpc6::TRPC6IonChannelCat,
  trpv3_channel: channels::trpv3::TRPV3IonChannelCat,
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
      &self.trpc6_channel,
      &self.trpv3_channel,
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
      &mut self.trpc6_channel,
      &mut self.trpv3_channel,
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

  pub fn adapt_timestep(&self, current_dt: f64, state_delta: f64, max_dt: f64) -> f64 {
    f64::max(
      current_dt * (constants::delta_tolerance / state_delta).powf(0.5),
      constants::slowest_dt,
    )
    .min(max_dt)
  }

  pub fn simulate(
    &mut self,
    pulse_protocol: impl ProtocolGenerator + RepeatingGenerator,
    recorder: &mut impl SimulationRecorder,
    min_points: usize,
  ) {
    let total_duration = pulse_protocol.single_duration();
    let measurements_dt = total_duration / (min_points as f64);
    log::info!(
      "Starting simulation. Duration according to pulse protocol: {:.3} s. Recording at least {} times.",
      total_duration,
      min_points
    );
    let mut n = 0;
    let mut total_time = 0.0;
    let mut t_next_measurement = 0.0;
    for channel in self.channels_mut() {
      log::info!("{}", channel.display_me());
    }
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    for step in pulse_protocol.generator() {
      let mut dt = constants::slowest_dt;
      let mut step_time: f64 = 0.0;
      while step_time < step.duration {
        let mut state_delta = 0.0;
        for channel in self.channels_mut() {
          state_delta += channel.update_state(step.voltage, dt);
        }
        if total_time + step_time >= t_next_measurement {
          recorder.record(self, step.voltage);
          t_next_measurement += measurements_dt;
        }
        n += 1;
        step_time += dt;

        #[cfg(feature = "adaptive-timestepping")]
        {
          dt = self.adapt_timestep(dt, state_delta, measurements_dt);
        }

        #[cfg(all(debug_assertions, feature = "pause-each-step"))]
        {
          print!("Break (t = {step_time:.7}, dt = {dt:.3e}); press return to continue");
          std::io::stdout().flush().unwrap();
          std::io::stdin().lock().read_line(&mut String::new()).unwrap();
        }
      }
      total_time += step_time;

      log::info!(
        "Pulse protocol step {:7} ({:6.3} V) for {:.3} s done, overall average dt = {:.3e} s",
        step.label,
        step.voltage,
        step.duration,
        total_time / (n as f64)
      );
    }
    log::info!("Total time passed from the cell perspective: {total_time:.3} s");
    #[cfg(not(target_arch = "wasm32"))]
    {
      let runtime = start.elapsed().as_secs_f64();
      log::info!("Total simulation runtime: {runtime:.3} s");
    }
  }
}

pub fn evaluate_current_match(measurements: &PatchClampData, current: DVector<f64>) -> f64 {
  log::info!(
    "Collected data: {} points from simulation, {} points from measurements.",
    current.len(),
    measurements.current.len()
  );
  let rows = measurements.current.len();
  let error = (current.rows_range(0..rows) - measurements.current.clone()).norm_squared();
  log::info!("Simulation match with measurements: {:.3}", error);
  error
}
pub fn evaluate_match(measurements: &PatchClampData, simulation_record: TotalCurrentRecord) -> f64 {
  evaluate_current_match(measurements, simulation_record.current_as_dvec())
}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pyo3::pymethods)]
impl A549CancerCell {
  #[cfg_attr(feature = "pyo3", staticmethod)]
  pub fn new() -> A549CancerCell {
    A549CancerCell {
      crac1_channel: channels::crac1::CRAC1IonChannelCat::new(),
      trpc6_channel: channels::trpc6::TRPC6IonChannelCat::new(),
      trpv3_channel: channels::trpv3::TRPV3IonChannelCat::new(),
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

  pub fn set_channel_counts(&mut self, counts: ChannelCounts) {
    self.kv13_channel.n_channels = counts[0];
    self.kv31_channel.n_channels = counts[1];
    self.kv34_channel.n_channels = counts[2];
    self.kv71_channel.n_channels = counts[3];
    self.kca11_channel.n_channels = counts[4];
    self.kca31_channel.n_channels = counts[5];
    self.task1_channel.n_channels = counts[6];
    self.crac1_channel.n_channels = counts[7];
    self.trpc6_channel.n_channels = counts[8];
    self.trpv3_channel.n_channels = counts[9];
    self.clc2_channel.n_channels = counts[10];
  }

  pub fn set_langthaler_et_al_channel_counts(&mut self, phase: CellPhase) {
    match phase {
      CellPhase::G0 => {
        self.set_channel_counts([22, 78, 5, 1350, 40, 77, 19, 200, 17, 12, 13].into());
      }
      CellPhase::G1 => {
        self.set_channel_counts([20, 90, 54, 558, 15, 63, 10, 200, 12, 13, 11].into());
      }
    }
  }

  #[cfg(feature = "default")]
  pub fn evaluate(&mut self, protocol: PatchClampProtocol, phase: CellPhase) -> f64 {
    let measurements = PatchClampData::load(protocol, phase).unwrap();
    let pulse_protocol = DefaultPulseProtocol {};
    let mut recorded = TotalCurrentRecord::empty();
    self.simulate(pulse_protocol, &mut recorded, measurements.current.len());
    evaluate_match(&measurements, recorded)
  }
}

impl Default for A549CancerCell {
  fn default() -> Self {
    Self::new()
  }
}
