extern crate core;

pub mod ast;
pub mod engine;
mod errors;
mod parser;
mod utils;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn rust_rule_engine(py: Python, m: &PyModule) -> PyResult<()> {
    let engine_module = PyModule::new(py, "engine")?;
    engine::engine(py, engine_module)?;
    m.add_submodule(engine_module)?;

    Ok(())
}
