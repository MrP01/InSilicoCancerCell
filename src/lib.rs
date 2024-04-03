#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait)]

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod pulseprotocol;
mod utils;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn _in_rusty_silico(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<patchclampdata::PatchClampProtocol>()?;
  m.add_class::<patchclampdata::CellPhase>()?;
  m.add_class::<cell::A549CancerCell>()?;
  Ok(())
}
