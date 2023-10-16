pub mod ast;
mod engine;
mod errors;
mod parser;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn rust_rule_engine(py: Python, m: &PyModule) -> PyResult<()> {
    let engine_module = PyModule::new(py, "engine")?;
    engine::engine(py, engine_module)?;
    m.add_submodule(engine_module)?;


    Ok(())
}
