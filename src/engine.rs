use crate::ast::{EvalResult, EvalResultTypes, NestedValue, Statement};
use crate::errors::{EvaluationError, SymbolResolutionError};
use crate::parser;
use crate::utils::py_dict_to_hashmap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pyclass]
pub struct Context {
    // TODO: Is this even needed? We don't have statements so idk how assignments would work
    assignments: HashMap<String, EvalResultTypes>,
    // TODO: Implement
    //  - Symbol resolution
}
impl Context {
    fn new(assignments: Option<HashMap<String, EvalResultTypes>>) -> Self {
        Context {
            assignments: assignments.unwrap_or(HashMap::new()),
        }
    }

    pub fn resolve(
        &self,
        name: &String,
        thing: Option<&HashMap<String, NestedValue>>,
    ) -> Result<EvalResultTypes, SymbolResolutionError> {
        // TODO: Implement me
        //  - Match builtins
        if let Some(value) = self.assignments.get(name) {
            return Ok(value.clone());
        }
        if let Some(thing) = thing {
            if let Some(value) = thing.get(name) {
                let converted_value: EvalResultTypes = value.clone().try_into().map_err(|err| {
                    SymbolResolutionError::new(&format!(
                        "Failed to convert value to EvalResultTypes: {}",
                        err
                    ))
                })?;
                return Ok(converted_value);
            }
        }
        Err(SymbolResolutionError::new(&format!(
            "Symbol {} not found",
            name
        )))
    }

    // TODO: Implement attribute resolution
}

#[pyclass]
pub struct Rule {
    parser: parser::Parser,
    statement: Statement,
}

/// Test docstring for the Rule class
#[pymethods]
impl Rule {
    #[new]
    pub fn new(text: String, context: Option<&Context>) -> Self {
        let parser = parser::Parser::new();
        // FIXME: Handle errors more elegantly
        let statement = parser.parse_internal(text).unwrap();
        Rule { parser, statement }
    }

    /// Test whether or not the rule is syntactically correct. This verifies the grammar is well structured and that
    /// there are no type compatibility issues regarding literals or symbols with known types (see
    /// `Context.resolve_type` for specifying symbol type information).
    ///
    /// # Arguments
    /// * text - The text to parse
    /// * context - The context used for specifying symbol type information.
    #[staticmethod]
    pub fn is_valid(text: String, _ctx: Option<&Context>) -> PyResult<bool> {
        let cls_parser = parser::Parser::new();
        let statement = cls_parser.parse_internal(text);
        match statement {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn evaluate(&self, thing: Option<&PyDict>, ctx: Option<&Context>) -> EvalResult {
        let values = &py_dict_to_hashmap(thing)
            .map_err(|err| EvaluationError::new(&format!("Failed to convert pydict: {}", err)))?;
        self.statement
            .evaluate(&ctx.unwrap_or(&Context::new(None)), values)
    }

    pub fn matches(&self, thing: Option<&PyDict>) -> bool {
        // Should be the equivalent of calling bool(rule.evaluate(thing)) in Python
        match self.evaluate(thing, None) {
            Ok(EvalResultTypes::Boolean(value)) => value,
            Ok(EvalResultTypes::Float(value)) => value != 0f64,
            Ok(EvalResultTypes::Integer(value)) => value != 0i64,
            Ok(EvalResultTypes::String(value)) => !value.is_empty(),
            Err(_) => false,
        }
    }
}

/// Adds the objects within the engine to the module.
/// The module is the engine module created in lib.rs
pub fn engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Rule>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_is_valid() {
        pyo3::prepare_freethreaded_python();
        let valid_statements = vec![
            "1 == 1",
            "1.3421 == 1.3422",
            "23482.324123512 == 23482.324123512",
            "true == true",
        ];
        for statement in valid_statements {
            let result = Rule::is_valid(statement.into(), None).unwrap();
            assert!(result);
        }
        let invalid_statements = vec!["1abc == 1", "true =="];
        for statement in invalid_statements {
            println!("Testing invalid statement: {}", statement);
            assert_eq!(Rule::is_valid(statement.into(), None).unwrap(), false);
        }
    }

    #[test]
    fn test_evaluate_with_symbol_resolution() {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new("age == 1".into(), None);
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("age", 1).unwrap();
            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }

    #[test]
    fn test_evaluate_with_multisymbol_resolution() {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new("age >= required_age".into(), None);
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("age", 23).unwrap();
            dict.set_item("required_age", 21).unwrap();
            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }
}
