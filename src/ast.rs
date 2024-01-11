use std::collections::HashMap;
use pyo3::IntoPy;
use pyo3::prelude::*;
use pyo3::types::PyBool;
use pyo3::types::PyFloat;

use crate::engine::Context;
use crate::errors::EvaluationError;

#[derive(Clone)]
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
pub type EvalResult = Result<EvalResultTypes, EvaluationError>;

pub enum Statement {
    Expression(Expression),
}
impl Statement {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            Statement::Expression(expr) => expr.evaluate(ctx, thing),
        }
    }
}

pub enum Expression {
    Equality(EqualityExpression),
}
impl Expression {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            Expression::Equality(expr) => expr.evaluate(ctx, thing),
        }
    }
}

pub enum EqualityExpression {
    Equal(Box<ComparisonExpression>, Box<ComparisonExpression>),
    NotEqual(Box<ComparisonExpression>, Box<ComparisonExpression>),
    Comparison(ComparisonExpression), // Value passthrough
}
impl EqualityExpression {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            EqualityExpression::Equal(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
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
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
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
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            ComparisonExpression::GreaterThan(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs > rhs))
                    },
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            },
            ComparisonExpression::GreaterThanOrEqual(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs >= rhs))
                    },
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            },
            ComparisonExpression::LessThan(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs < rhs))
                    },
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            },
            ComparisonExpression::LessThanOrEqual(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs <= rhs))
                    },
                    _ => Err(EvaluationError::new("Cannot compare different types")),
                }
            },
            ComparisonExpression::Additive(additive) => additive.evaluate(ctx, thing),
        }
    }
}

pub enum AdditiveExpression {
    Factor(FactorExpression),
}
impl AdditiveExpression {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            AdditiveExpression::Factor(factor) => factor.evaluate(ctx, thing),
        }
    }
}

pub enum FactorExpression {
    Unary(UnaryExpression),
}
impl FactorExpression {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            FactorExpression::Unary(unary) => unary.evaluate(ctx, thing),
        }
    }
}

pub enum UnaryExpression {
    Primary(PrimaryExpression),
}
impl UnaryExpression {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
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
}
impl PrimaryExpression {
    pub fn evaluate(&self, ctx: &Context, thing: HashMap<String, EvalResultTypes>) -> EvalResult {
        match self {
            PrimaryExpression::Float(value) => {
                Ok(EvalResultTypes::Float(*value))
            },
            PrimaryExpression::True => Ok(EvalResultTypes::Boolean(true)),
            PrimaryExpression::False => Ok(EvalResultTypes::Boolean(false)),
            PrimaryExpression::Symbol(str) => ctx.resolve(str.clone(), Some(thing)).map_err(|e| EvaluationError::new(&e.to_string())),
        }
    }
}
