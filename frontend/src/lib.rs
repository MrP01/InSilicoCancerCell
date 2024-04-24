use std::{collections::HashMap, vec};

use in_silico_cancer_cell::{
  cell::{A549CancerCell, SimulationRecorder},
  pulseprotocol::{ProtocolGenerator, PulseProtocol},
};
use wasm_bindgen::prelude::*;
use web_sys::console;

// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelThroughput {
  pub current: Vec<f64>,
  pub states: Vec<Vec<f64>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FullRecorder {
  pub total_current: Vec<f64>,
  pub channels: HashMap<String, ChannelThroughput>,
}
impl FullRecorder {
  pub fn new_for_cell(cell: &A549CancerCell) -> Self {
    let mut channels = HashMap::new();
    for channel in cell.channels() {
      channels.insert(
        channel.display_name(),
        ChannelThroughput {
          current: vec![],
          states: vec![],
        },
      );
    }
    Self {
      total_current: vec![],
      channels,
    }
  }
}

impl SimulationRecorder for FullRecorder {
  fn record(&mut self, cell: &A549CancerCell, voltage: f64) {
    let mut total_current = 0.0;
    for channel in cell.channels() {
      channel.display_me();
      let channel_throughput = self
        .channels
        .get_mut(&channel.display_name())
        .expect("recorder should have been initialised");
      let current = channel.current(voltage);
      channel_throughput.current.push(current);
      channel_throughput.states.push(channel.internal_state());
      total_current += current;
    }
    self.total_current.push(total_current);
  }
}

#[wasm_bindgen]
pub fn run() -> JsValue {
  // let measurements = PatchClampData::demo();
  let pulse_protocol = PulseProtocol::default();
  let mut cell = A549CancerCell::new();
  let mut recorded = FullRecorder::new_for_cell(&cell);
  cell.simulate(pulse_protocol, &mut recorded);
  return serde_wasm_bindgen::to_value(&recorded).unwrap();
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
