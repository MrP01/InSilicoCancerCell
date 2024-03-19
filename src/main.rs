#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait)]

use cell::A549CancerCell;
use patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol};
use pulseprotocol::{ProtocolGenerator, PulseProtocol};
use utils::evaluate_match;

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod pulseprotocol;
mod utils;

fn main() {
  utils::setup_logging();
  let measurements = PatchClampData::load(PatchClampProtocol::Ramp, CellPhase::G0).unwrap();
  let pulse_protocol = PulseProtocol::default();
  let mut cell = A549CancerCell::new();
  let simulation = cell.simulate(pulse_protocol);
  evaluate_match(measurements, simulation);
}
