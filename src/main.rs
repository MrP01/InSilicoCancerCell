use cell::A549CancerCell;
use patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol};

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod utils;

fn main() {
  let data = PatchClampData::load(PatchClampProtocol::Deactivation, CellPhase::G0).unwrap();
  let mut cell = A549CancerCell::new();
  cell.simulate(10.0);
}
