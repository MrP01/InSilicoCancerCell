#![feature(coroutines, iter_from_coroutine, type_alias_impl_trait)]

mod cell;
mod channels;
mod constants;
mod patchclampdata;
mod pulseprotocol;
mod utils;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
  Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn in_silico_cancer_cell(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
  m.add_class::<cell::A549CancerCell>()?;
  Ok(())
}
