use std::collections::HashMap;
use pyo3::prelude::*;

use crate::engine::Context;

#[pyclass]
pub struct Parser {
    // TODO: Implement https://github.com/zeroSteiner/rule-engine/blob/master/lib/rule_engine/parser.py#L107
    op_names: HashMap<String, String>,
}

#[pymethods]
impl Parser {
    #[new]
    pub fn new() -> Self {
        Parser {
            op_names: HashMap::new(),
        }
    }

    pub fn parse(&self, text: &str, context: &Context) -> PyResult<bool> {
        // TODO: look into https://github.com/lalrpop/lalrpop
        Ok(true)
    }
}
