use pyo3::prelude::*;

use crate::engine::Context;
use crate::ast::Statement;

use lrlex::{DefaultLexerTypes, lrlex_mod};
use lrpar::lrpar_mod;
use crate::errors::ParseError;

lrlex_mod!("rule.l");
lrpar_mod!("rule.y");


// TODO: Can we refactor this into a single function without using a struct?
//  - Will this break the interface with Python?
#[pyclass]
pub struct Parser {
    pub lexerdef: lrlex::LRNonStreamingLexerDef<DefaultLexerTypes>
}

impl Parser {
    pub fn parse_internal<>(&self, text: String) -> Result<Statement, ParseError> {
        let lexer = self.lexerdef.lexer(&text);
        let (res, _errs) = rule_y::parse(&lexer);
        if let Some(Ok(r)) = res {
            Ok(Statement::Expression(r))
        } else {
            Err(ParseError::new("Failed to parse expression"))
        }
    }
}

fn map_err_to_py(e: ParseError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
}

#[pymethods]
impl Parser {
    #[new]
    pub fn new() -> Self {
        Parser {
            lexerdef: rule_l::lexerdef(),
        }
    }

    pub fn parse(&self, text: String, context: &Context) -> Result<Py<PyAny>, PyErr> {
        // FIXME: Map error to python type
        let res = self.parse_internal(text).map_err(map_err_to_py)?;
        Python::with_gil(|py| -> Result<Py<PyAny>, PyErr> {
            // FIXME: We should be returning a statement from this parse function
            //  - Can an enum be a python class? Probably not so we'll need to wrap it in a struct or something
            match res.evaluate(context) {
                Ok(result) => Ok(result.into_py(py)),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            }
        })
    }
}
