use pyo3::{prelude::*, types::{PyType, PyString}};
use crate::parser;

#[pyclass]
pub struct Context {
    // TODO: Implement
}

#[pymethods]
impl Context {
    #[new]
    fn new() -> Self {
        Context {}
    }
}

#[pyclass]
pub struct Rule {
    parser:  parser::Parser,
}

#[pymethods]
impl Rule {

    // TODO: How to create default values like parser = Parser()?

    #[new]
    fn new() -> Self {
        Rule {
            parser: parser::Parser::new(),
        }
    }

    /// Test whether or not the rule is syntactically correct. This verifies the grammar is well structured and that
    /// there are no type compatibility issues regarding literals or symbols with known types (see
    /// `Context.resolve_type` for specifying symbol type information).
    /// 
    /// # Arguments
    /// * text - The text to parse
    /// * context - The context used for specifying symbol type information.
    #[classmethod]
    fn is_valid(_cls: &PyType, text: String, context: Option<&Context>) -> PyResult<bool> {
        let cls_parser = parser::Parser::new();
        match cls_parser.parse(&text, context.unwrap_or(&Context::new())) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false), // Match EngineError
        }
    }
}

/// Adds the objects within the engine to the module.
/// The module is th engine module created in lib.rs
pub fn engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Rule>()?;
    Ok(())
}