#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait)]

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod pulseprotocol;
mod utils;

/// A Python module implemented in Rust.
#[pyo3::pymodule]
fn _in_rusty_silico(_py: pyo3::Python, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
  m.add_class::<patchclampdata::PatchClampProtocol>()?;
  m.add_class::<patchclampdata::CellPhase>()?;
  m.add_class::<cell::A549CancerCell>()?;
  Ok(())
}
