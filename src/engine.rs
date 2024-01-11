use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::ast::{EvalResult, EvalResultTypes, Statement};
use crate::ast::EvalResultTypes::Boolean;
use crate::errors::SymbolResolutionError;
use crate::parser;

#[pyclass]
pub struct Context {
    assignments: HashMap<String, EvalResultTypes>,
    // TODO: Implement
    //  - Symbol resolution
    //      - Need to add SymbolExpression
    //          - Should be a placeholder that evaluated to a value based on the context
}

impl Context {

    fn new(assignments: Option<HashMap<String, EvalResultTypes>>) -> Self {
        Context {
            assignments: assignments.unwrap_or(HashMap::new()),
        }
    }

    pub fn resolve(self, name: String, thing: Option<HashMap<String, EvalResultTypes>>) -> Result<EvalResultTypes, SymbolResolutionError> {
        // TODO: Implement me
        //  - Match builtins
        if let Some(value) = self.assignments.get(&name) {
            return Ok((*value).clone());
        }
        if let Some(thing) = thing {
            if let Some(value) = thing.get(&name) {
                return Ok((*value).clone());
            }
        }
        Err(SymbolResolutionError::new(&format!("Symbol {} not found", name)))
    }
}

#[pyclass]
pub struct Rule {
    parser: parser::Parser,
    statement: Statement,
}

#[pymethods]
impl Rule {

    #[new]
    fn new(text: String, context: Option<&Context>) -> Self {
        let parser = parser::Parser::new();
        // FIXME: Handle errors more elegantly
        let statement = parser.parse_internal(text).unwrap();
        Rule {
            parser,
            statement,
        }
    }

    /// Test whether or not the rule is syntactically correct. This verifies the grammar is well structured and that
    /// there are no type compatibility issues regarding literals or symbols with known types (see
    /// `Context.resolve_type` for specifying symbol type information).
    /// 
    /// # Arguments
    /// * text - The text to parse
    /// * context - The context used for specifying symbol type information.
    #[staticmethod]
    fn is_valid(text: String, context: Option<Context>) -> PyResult<bool> {
        let cls_parser = parser::Parser::new();
        match cls_parser.parse(text, &context.unwrap_or(Context::new(None))) {
            Ok(_) => Ok(true),
            Err(err) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string()))
        }
    }

    fn evaluate(&self, ctx: Option<Context>, thing: Option<PyDict>) -> EvalResult {
        // FIXME: Convert pydict into hashmap (or accept PyDict in AST)
        self.statement.evaluate(&ctx.unwrap_or(Context::new(None)), thing.unwrap_or(HashMap::new()))
    }

    fn matches(&self, thing: Option<PyDict>) -> bool {
        match self.evaluate(None, thing) {
            Err(err) => false,
            Ok(Boolean(value)) => value,
            Ok(EvalResultTypes::Float(value)) => value != 0f64,
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
        let invalid_statements = vec!["true ==", "1abc == 1"];
        for statement in invalid_statements {
            assert!(Rule::is_valid(statement.into(), None).is_err());
        }
    }

}