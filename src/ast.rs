use cfgrammar::Span;
use lrlex::DefaultLexerTypes;
use lrpar::NonStreamingLexer;
use pyo3::IntoPy;
use pyo3::prelude::*;
use pyo3::types::PyBool;
use pyo3::types::PyFloat;

use crate::errors::EvaluationError;

pub enum EvalResultTypes {
    Float(f64),
    Boolean(bool),
}
impl IntoPy<PyObject> for EvalResultTypes {
    fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
        match self {
            EvalResultTypes::Float(value) => {
                PyFloat::new(py, value).to_object(py)
            },
            EvalResultTypes::Boolean(value) => {
                PyBool::new(py, value).to_object(py)
            },
        }
    }
}
type EvalResult = Result<EvalResultTypes, EvaluationError>;

pub enum Expression {
    Equality(EqualityExpression),
}
impl Expression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            Expression::Equality(expr) => expr.evaluate(lexer),
        }
    }
}

pub enum EqualityExpression {
    Equal(Box<ComparisonExpression>, Box<ComparisonExpression>),
    NotEqual(Box<ComparisonExpression>, Box<ComparisonExpression>),
    Comparison(ComparisonExpression), // Value passthrough
}
impl EqualityExpression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            EqualityExpression::Equal(lhs, rhs) => {
                let lhs = lhs.evaluate(lexer)?;
                let rhs = rhs.evaluate(lexer)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    },
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    },
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            },
            EqualityExpression::NotEqual(lhs, rhs) => {
                let lhs = lhs.evaluate(lexer)?;
                let rhs = rhs.evaluate(lexer)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs))
                    },
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs))
                    },
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            },
            EqualityExpression::Comparison(comp) => comp.evaluate(lexer),
        }
    }
}


pub enum ComparisonExpression {
    Additive(AdditiveExpression),
}
impl ComparisonExpression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            ComparisonExpression::Additive(additive) => additive.evaluate(lexer),
        }
    }
}

pub enum AdditiveExpression {
    Factor(FactorExpression),
}
impl AdditiveExpression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            AdditiveExpression::Factor(factor) => factor.evaluate(lexer),
        }
    }
}

pub enum FactorExpression {
    Unary(UnaryExpression),
}
impl FactorExpression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            FactorExpression::Unary(unary) => unary.evaluate(lexer),
        }
    }
}

pub enum UnaryExpression {
    Primary(PrimaryExpression),
}
impl UnaryExpression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            UnaryExpression::Primary(primary) => primary.evaluate(lexer),
        }
    }
}

pub enum PrimaryExpression {
    Float(Span),
    True,
    False,
}
impl PrimaryExpression {
    pub fn evaluate(&self, lexer: &dyn NonStreamingLexer<DefaultLexerTypes<u32>>) -> EvalResult {
        match self {
            PrimaryExpression::Float(span) => {
                let value = lexer
                    .span_str(*span)
                    .parse::<f64>()
                    .map_err(|err| EvaluationError::new(&format!("{}", err)))?;
                Ok(EvalResultTypes::Float(value))
            },
            PrimaryExpression::True => Ok(EvalResultTypes::Boolean(true)),
            PrimaryExpression::False => Ok(EvalResultTypes::Boolean(false)),
        }
    }
}
