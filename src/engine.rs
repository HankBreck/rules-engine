use crate::ast::{EvalResult, EvalResultTypes, Statement};
use crate::builtins::resolve_builtin_methods;
use crate::errors::{EvaluationError, SymbolResolutionError};
use crate::parser;
use crate::utils::get_value_from_py_dict;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::any::Any;
use std::collections::HashMap;

#[pyclass]
pub struct Context {
    // TODO: Is this even needed? We don't have statements so idk how assignments would work
    assignments: HashMap<String, EvalResultTypes>,
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
        thing: Option<&PyDict>,
    ) -> Result<EvalResultTypes, SymbolResolutionError> {
        // TODO: Match builtins
        if let Some(value) = self.assignments.get(name) {
            return Ok(value.clone());
        }
        if let Some(dict) = thing {
            match get_value_from_py_dict(dict, &[name]) {
                Ok(Some(value)) => return Ok(value),
                Err(_) => return Err(SymbolResolutionError::new("Failed to get value")),
                _ => {}
            }
        }
        Err(SymbolResolutionError::new(&format!(
            "Symbol {} not found",
            name
        )))
    }

    pub fn resolve_attribute(
        &self,
        keys: &[&str],
        thing: Option<&PyDict>,
    ) -> Result<EvalResultTypes, SymbolResolutionError> {
        // If the last key is a builtin method, we need to resolve the value of the attribute and then call the method
        if let Ok(builtin_method) = resolve_builtin_methods(keys[keys.len() - 1]) {
            let value = self.resolve_attribute(&keys[..keys.len() - 1], thing)?;
            return builtin_method(value)
                .map_err(|err| SymbolResolutionError::new(&err.to_string()));
        }
        // Fetch attribute's value from original python object
        if let Some(dict) = thing {
            match get_value_from_py_dict(dict, keys) {
                Ok(Some(value)) => return Ok(value),
                Err(_) => return Err(SymbolResolutionError::new("Failed to get value")),
                _ => {}
            }
        }
        Err(SymbolResolutionError::new(&format!(
            "Symbol {} not found",
            keys.join("."),
        )))
    }
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
    pub fn new(text: String) -> PyResult<Self> {
        let parser = parser::Parser::new();
        let statement = parser.parse_internal(text)?;
        Ok(Rule { parser, statement })
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
        self.statement
            .evaluate(&ctx.unwrap_or(&Context::new(None)), thing)
    }

    pub fn matches(&self, thing: Option<&PyDict>) -> bool {
        // Should be the equivalent of calling bool(rule.evaluate(thing)) in Python
        match self.evaluate(thing, None) {
            Ok(result) => result.is_truthy(),
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
        let rule = Rule::new("age == 1".into()).unwrap();
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("age", 1).unwrap();
            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }

    #[test]
    fn test_evaluate_with_multisymbol_resolution() {
        let attrs = &["hi", "im", "hank", "in", "utah"];
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new("age >= required_age".into()).unwrap();
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("age", 23).unwrap();
            dict.set_item("required_age", 21).unwrap();
            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }

    #[test]
    fn test_evaluate_with_builtin_method() {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new("age.as_lower == \"hank\"".into()).unwrap();
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("age", "HANK").unwrap();
            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }

    #[test]
    fn test_evaluate_with_value_from_builtin() {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new(
            "provider1.language.language_code == provider2.language.language_code".into(),
        )
        .unwrap();
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            let provider1 = PyDict::new(py);
            provider1.set_item("language", "en-US").unwrap();
            dict.set_item("provider1", provider1).unwrap();

            let provider2 = PyDict::new(py);
            provider2.set_item("language", "en").unwrap();
            dict.set_item("provider2", provider2).unwrap();

            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }

    #[test]
    fn test_evaluate_unary_not() {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new("not true".into()).unwrap();
        let _ = &Python::with_gil(|py| {
            let result = rule.evaluate(None, None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(false));
        });
    }

    #[test]
    fn test_evaluate_unary_minus() {
        pyo3::prepare_freethreaded_python();
        let true_rule = Rule::new("1 > -1".into()).unwrap();
        let false_rule = Rule::new("-1 < -2".into()).unwrap();
        let _ = &Python::with_gil(|py| {
            assert_eq!(
                true_rule.evaluate(None, None).unwrap(),
                EvalResultTypes::Boolean(true),
            );
            assert_eq!(
                false_rule.evaluate(None, None).unwrap(),
                EvalResultTypes::Boolean(false),
            );
        });
    }

    #[test]
    fn test_evaluation_test() {
        pyo3::prepare_freethreaded_python();
        let rule =
            Rule::new("attribution.provider_facility_id and not attribution.unassigned".into())
                .unwrap();
        let _ = &Python::with_gil(|py| {
            let dict = PyDict::new(py);
            let attribution = PyDict::new(py);
            attribution.set_item("unassigned", false).unwrap();
            attribution
                .set_item("provider_facility_id", "1234")
                .unwrap();
            dict.set_item("attribution", attribution).unwrap();
            let result = rule.evaluate(Some(dict), None).unwrap();
            assert_eq!(result, EvalResultTypes::Boolean(true));
        });
    }

    #[test]
    fn test_addition() {
        pyo3::prepare_freethreaded_python();
        let rule = Rule::new("1.0 + 1".into()).unwrap();
        let result = rule.evaluate(None, None).unwrap();
        assert_eq!(result, EvalResultTypes::Integer(2));
    }
}
