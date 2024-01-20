use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::IntoPy;
use std::collections::HashMap;

use crate::engine::Context;
use crate::errors::{EvaluationError, TypeConversionError};

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

#[derive(Clone)]
pub enum NestedValue {
    Primitive(EvalResultTypes),
    Nested(HashMap<String, NestedValue>),
}
impl FromPyObject<'_> for NestedValue {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        if let Ok(dict) = ob.downcast::<PyDict>() {
            let mut map = HashMap::new();
            for (key, value) in dict.into_iter() {
                let key = key.extract::<String>()?;
                let nested_value = NestedValue::extract(value)?;
                map.insert(key, nested_value);
            }
            Ok(NestedValue::Nested(map))
        } else if let Ok(val) = ob.extract::<bool>() {
            Ok(NestedValue::Primitive(EvalResultTypes::Boolean(val)))
        } else if let Ok(val) = ob.extract::<f64>() {
            Ok(NestedValue::Primitive(EvalResultTypes::Float(val)))
        } else if let Ok(val) = ob.extract::<i64>() {
            Ok(NestedValue::Primitive(EvalResultTypes::Integer(val)))
        } else if let Ok(val) = ob.extract::<String>() {
            Ok(NestedValue::Primitive(EvalResultTypes::String(val)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Could not convert value",
            ))
        }
    }
}
impl TryInto<EvalResultTypes> for NestedValue {
    type Error = TypeConversionError;
    fn try_into(self) -> Result<EvalResultTypes, TypeConversionError> {
        match self {
            NestedValue::Primitive(value) => Ok(value),
            NestedValue::Nested(_) => Err(TypeConversionError::new(
                "Cannot convert nested value to primitive",
            )),
        }
    }
}

pub enum Statement {
    Expression(Expression),
}
impl Statement {
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
        match self {
            Statement::Expression(expr) => expr.evaluate(ctx, thing),
        }
    }
}

pub enum Expression {
    Equality(EqualityExpression),
}
impl Expression {
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
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
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
        match self {
            EqualityExpression::Equal(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
                        Ok(EvalResultTypes::Boolean(lhs == rhs))
                    }
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
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
                    (EvalResultTypes::Boolean(lhs), EvalResultTypes::Boolean(rhs)) => {
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
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
        match self {
            ComparisonExpression::GreaterThan(lhs, rhs) => {
                let lhs = lhs.evaluate(ctx, thing)?;
                let rhs = rhs.evaluate(ctx, thing)?;
                match (lhs, rhs) {
                    (EvalResultTypes::Float(lhs), EvalResultTypes::Float(rhs)) => {
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
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
        match self {
            AdditiveExpression::Factor(factor) => factor.evaluate(ctx, thing),
        }
    }
}

pub enum FactorExpression {
    Unary(UnaryExpression),
}
impl FactorExpression {
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
        match self {
            FactorExpression::Unary(unary) => unary.evaluate(ctx, thing),
        }
    }
}

pub enum UnaryExpression {
    Primary(PrimaryExpression),
}
impl UnaryExpression {
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
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
    pub fn evaluate(&self, ctx: &Context, thing: &HashMap<String, NestedValue>) -> EvalResult {
        match self {
            PrimaryExpression::Float(value) => Ok(EvalResultTypes::Float(*value)),
            PrimaryExpression::True => Ok(EvalResultTypes::Boolean(true)),
            PrimaryExpression::False => Ok(EvalResultTypes::Boolean(false)),
            PrimaryExpression::Symbol(str) => ctx
                .resolve(str, Some(thing))
                .map_err(|err| EvaluationError::new(&err.to_string())),
        }
    }
}
