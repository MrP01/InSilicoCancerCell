use in_silico_cancer_cell::{
  cell::A549CancerCell,
  // patchclampdata::{CellPhase, PatchClampData, PatchClampProtocol},
  pulseprotocol::{ProtocolGenerator, PulseProtocol},
};
use wasm_bindgen::prelude::*;
use web_sys::console;

// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Vec<f64> {
  // let measurements = PatchClampData::demo();
  let pulse_protocol = PulseProtocol::default();
  let mut cell = A549CancerCell::new();
  let simulation = cell.simulate(pulse_protocol);
  return simulation.current;
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();
  console::log_1(&JsValue::from_str("Hello from the in-silico Rust library!"));
  Ok(())
}
