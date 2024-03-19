#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait)]

use cell::A549CancerCell;
use patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol};
use pulseprotocol::{ProtocolGenerator, PulseProtocol};
use simple_logger::SimpleLogger;

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod pulseprotocol;
mod utils;

fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .without_timestamps()
    .init()
    .unwrap();

  let measurements = PatchClampData::load(PatchClampProtocol::Ramp, CellPhase::G0).unwrap();
  let pulse_protocol = PulseProtocol::default();
  let mut cell = A549CancerCell::new();
  let simulation = cell.simulate(pulse_protocol);
  let rows = measurements.current.len();
  let error = (simulation.as_dvec().rows_range(0..rows) - measurements.current).norm_squared();
  log::info!("Simulation match with measurements: {:.3}", error);
}
