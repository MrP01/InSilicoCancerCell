#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait, cfg_eval)]

pub mod cell;
pub mod channels;
pub mod constants;
pub mod patchclampdata;
pub mod pulseprotocol;
pub mod utils;

#[cfg(feature = "pyo3")]
pub mod optimisation;

/// A Python module implemented in Rust.
#[cfg(feature = "pyo3")]
#[pyo3::pymodule]
fn _in_rusty_silico(_py: pyo3::Python, m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
  m.add_class::<patchclampdata::PatchClampProtocol>()?;
  m.add_class::<patchclampdata::CellPhase>()?;
  m.add_class::<patchclampdata::PatchClampData>()?;
  m.add_class::<cell::A549CancerCell>()?;
  m.add_class::<optimisation::InSilicoMethod>()?;
  m.add_class::<optimisation::ChannelCountsProblem>()?;
  m.add_function(pyo3::wrap_pyfunction!(optimisation::find_best_fit_for, m)?)?;
  m.add_function(pyo3::wrap_pyfunction!(utils::setup_logging, m)?)?;
  Ok(())
}
