use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::IntoPy;

use crate::engine::Context;
use crate::errors::EvaluationError;

#[derive(Clone, Debug)]
pub enum EvalResultTypes {
    Boolean(bool),
    Float(f64),
    Integer(i64),
    String(String),
}
impl PartialEq for EvalResultTypes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => lhs == rhs,
            (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => lhs == rhs,
            (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => lhs == rhs,
            (EvalResultTypes::String(lhs), EvalResultTypes::String(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}
impl IntoPy<PyObject> for EvalResultTypes {
    fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
        match self {
            EvalResultTypes::Boolean(value) => value.into_py(py),
            EvalResultTypes::Float(value) => value.into_py(py),
            EvalResultTypes::Integer(value) => value.into_py(py),
            EvalResultTypes::String(value) => value.into_py(py),
        }
    }
}
pub type EvalResult = Result<EvalResultTypes, EvaluationError>;

pub enum Statement {
    Expression(Expression),
}
impl Statement {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            Statement::Expression(expr) => expr.evaluate(ctx, thing),
        }
    }
}

pub enum Expression {
    Logical(LogicalExpression),
}
impl Expression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            Expression::Logical(expr) => expr.evaluate(ctx, thing),
        }
    }
}

pub enum LogicalExpression {
    And(Box<EqualityExpression>, Box<EqualityExpression>),
    Or(Box<EqualityExpression>, Box<EqualityExpression>),
    Equality(EqualityExpression), // Value passthrough
}
impl LogicalExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            LogicalExpression::And(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs && rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            LogicalExpression::Or(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs || rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            LogicalExpression::Equality(eq) => eq.evaluate(ctx, thing),
        }
    }
}

pub enum EqualityExpression {
    Equal(Box<ComparisonExpression>, Box<ComparisonExpression>),
    NotEqual(Box<ComparisonExpression>, Box<ComparisonExpression>),
    Comparison(ComparisonExpression), // Value passthrough
}
impl EqualityExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            EqualityExpression::Equal(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    // TODO: This is a bit of a mess, but it works for now
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs as f64 == rhs))
                    }
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs as f64))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    }
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    }
                    (EvalResultTypes::String(lhs), EvalResultTypes::String(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            EqualityExpression::NotEqual(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs as f64 != rhs))
                    }
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs as f64))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs))
                    }
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs))
                    }
                    (EvalResultTypes::String(lhs), EvalResultTypes::String(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs != rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            EqualityExpression::Comparison(comp) => comp.evaluate(ctx, thing),
        }
    }
}

pub enum ComparisonExpression {
    GreaterThan(Box<AdditiveExpression>, Box<AdditiveExpression>),
    GreaterThanOrEqual(Box<AdditiveExpression>, Box<AdditiveExpression>),
    LessThan(Box<AdditiveExpression>, Box<AdditiveExpression>),
    LessThanOrEqual(Box<AdditiveExpression>, Box<AdditiveExpression>),
    Additive(AdditiveExpression),
}
impl ComparisonExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            ComparisonExpression::GreaterThan(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs > rhs))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs as f64 > rhs))
                    }
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs > rhs as f64))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs > rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            ComparisonExpression::GreaterThanOrEqual(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs >= rhs))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs as f64 >= rhs))
                    }
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs >= rhs as f64))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs >= rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            ComparisonExpression::LessThan(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs < rhs))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean((lhs as f64) < rhs))
                    }
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs < rhs as f64))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs < rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            ComparisonExpression::LessThanOrEqual(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs <= rhs))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean((lhs as f64) <= rhs))
                    }
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs <= rhs as f64))
                    }
                    (EvalResultTypes::Integer(lhs), EvalResultTypes::Integer(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs <= rhs))
                    }
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            }
            ComparisonExpression::Additive(additive) => additive.evaluate(ctx, thing),
        }
    }
}

pub enum AdditiveExpression {
    Factor(FactorExpression),
}
impl AdditiveExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            AdditiveExpression::Factor(factor) => factor.evaluate(ctx, thing),
        }
    }
}

pub enum FactorExpression {
    Unary(UnaryExpression),
}
impl FactorExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            FactorExpression::Unary(unary) => unary.evaluate(ctx, thing),
        }
    }
}

pub enum UnaryExpression {
    Primary(PrimaryExpression),
}
impl UnaryExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            UnaryExpression::Primary(primary) => primary.evaluate(ctx, thing),
        }
    }
}

pub enum PrimaryExpression {
    Float(f64),
    True,
    False,
    Symbol(String),
    Attribute(String),
    String(String),
}
impl PrimaryExpression {
    pub fn evaluate(&self, ctx: &Context, thing: Option<&PyDict>) -> EvalResult {
        match self {
            PrimaryExpression::Float(value) => Ok(EvalResultTypes::Float(*value)),
            PrimaryExpression::True => Ok(EvalResultTypes::Boolean(true)),
            PrimaryExpression::False => Ok(EvalResultTypes::Boolean(false)),
            PrimaryExpression::Symbol(str) => ctx
                .resolve(str, thing)
                .map_err(|err| EvaluationError::new(&err.to_string())),
            PrimaryExpression::Attribute(raw_attr) => {
                let keys: Vec<&str> = raw_attr.split('.').collect();
                ctx.resolve_attribute(&keys, thing)
                    .map_err(|err| EvaluationError::new(&err.to_string()))
            }
            PrimaryExpression::String(str) => Ok(EvalResultTypes::String(str.clone())),
        }
    }
}
