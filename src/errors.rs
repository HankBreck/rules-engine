use std::fmt;
use pyo3::exceptions::PyValueError;
use pyo3::PyErr;

macro_rules! define_error {
    ($name:ident, $base:ident) => {
        #[derive(Debug)]
        pub struct $name {
            message: String,
        }

        impl $name {
            pub fn new(message: &str) -> Self {
                $name {
                    message: message.to_string(),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.message)
            }
        }

        impl std::error::Error for $name {}

        impl From<$name> for $base {
            fn from(err: $name) -> Self {
                $base::new(&err.message)
            }
        }

        impl From<$name> for PyErr {
            fn from(err: $name) -> Self {
                // FIXME: Need to solve for real error types
                PyErr::new::<PyValueError, _>(err.message)
            }
        }
    };
}

#[derive(Debug)]
pub struct EngineError {
    message: String,
}
impl EngineError {
    pub fn new(message: &str) -> Self {
        EngineError {
            message: message.to_string(),
        }
    }
}
impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

define_error!(EvaluationError, EngineError);
define_error!(TypeConversionError, EngineError);

#[derive(Debug)]
pub struct ParseError {
    message: String,
}
impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError{
            message: message.to_string(),
        }
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

define_error!(SymbolResolutionError, ParseError);

