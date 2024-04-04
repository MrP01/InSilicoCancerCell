#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait)]

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod pulseprotocol;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Vec<f64> {
  return vec![2.0, 3.45, 11.0];
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();
  console::log_1(&JsValue::from_str("Hello world!"));
  Ok(())
}

/// A Python module implemented in Rust.
#[pyo3::pymodule]
fn _in_rusty_silico(_py: pyo3::Python, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
  m.add_class::<patchclampdata::PatchClampProtocol>()?;
  m.add_class::<patchclampdata::CellPhase>()?;
  m.add_class::<cell::A549CancerCell>()?;
  Ok(())
}
