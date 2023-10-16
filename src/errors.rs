use std::fmt;

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

// class EvaluationError(EngineError):
// 	"""
// 	An error raised for issues which occur while the rule is being evaluated. This can occur at parse time while AST
// 	nodes are being evaluated during the reduction phase.
// 	"""
// 	pass

