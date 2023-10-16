use pyo3::prelude::*;

use crate::engine::Context;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("rule.l");
lrpar_mod!("rule.y");


// TODO: Can we refactor this into a single function without using a struct?
//  - Will this break the interface with Python?
#[pyclass]
pub struct Parser {}

#[pymethods]
impl Parser {
    #[new]
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse<'a>(&self, text: &'a str, _context: &Context) -> Result<Py<PyAny>, PyErr> {
        let lexerdef = rule_l::lexerdef();
        let lexer = lexerdef.lexer(text);
        let (res, errs) = rule_y::parse(&lexer);
        for e in errs {
            println!("{}", e.pp(&lexer, &rule_y::token_epp));
        }
        if let Some(Ok(r)) = res {

            Python::with_gil(|py| -> Result<Py<PyAny>, PyErr> {
                match r.evaluate(&lexer) {
                    Ok(result) => Ok(result.into_py(py)),
                    Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
                }
            })
        } 
        else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to parse expression"))
        }
    }
}
