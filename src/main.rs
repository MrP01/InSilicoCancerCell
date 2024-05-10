#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait, cfg_eval)]
#![allow(dead_code)]

mod cell;
mod channels;
mod constants;
mod optimisation;
mod patchclampdata;
mod pulseprotocol;
mod utils;

use cell::evaluate_match;
use cell::A549CancerCell;
use cell::TotalCurrentRecord;
use patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol};
use pulseprotocol::DefaultPulseProtocol;

fn main() {
  utils::setup_logging();
  let measurements = PatchClampData::load(PatchClampProtocol::Ramp, CellPhase::G0).unwrap();
  let pulse_protocol = DefaultPulseProtocol {};
  let mut cell = A549CancerCell::new();
  let mut recorded = TotalCurrentRecord::empty();
  cell.simulate(pulse_protocol, &mut recorded, measurements.current.len());
  evaluate_match(measurements, recorded);
}
