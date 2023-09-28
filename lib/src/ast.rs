use std::any::{Any};
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::errors::*;
use crate::types::DataType;

////////////////////////////////////////////////////////////////////////////////
// AST Node Base Trait
////////////////////////////////////////////////////////////////////////////////
trait ASTNode {
    fn evaluate(&self, thing: &dyn Context) -> Result<Box<dyn Any>, EvaluationError>;
    fn reduce(&self) -> Box<dyn ASTNode>;
}

////////////////////////////////////////////////////////////////////////////////
// Comment Expression
////////////////////////////////////////////////////////////////////////////////
struct Comment {
    value: String,
}

impl ASTNode for Comment {
    fn evaluate(&self, _thing: &dyn Context) -> Result<Box<dyn Any>, EvaluationError> {
        // Comments have no evaluation logic, return a placeholder value
        Ok(Box::new(()))
    }

    fn reduce(&self) -> Box<dyn ASTNode> {
        // Since comments cannot be reduced, return a clone of itself.
        Box::new(*self)
    }
}

impl Comment {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Base Expression Traits
////////////////////////////////////////////////////////////////////////////////
trait Expression: ASTNode {
    fn result_type(&self) -> DataType;
}

trait LiteralExpression: Expression {
    fn value(&self) -> Literal;
}

////////////////////////////////////////////////////////////////////////////////
/// Literal Expressions
////////////////////////////////////////////////////////////////////////////////
struct LiteralExpressionBase {
    context: Box<dyn Context>,
    value: Literal,
}

impl Expression for LiteralExpressionBase {
    fn result_type(&self) -> DataType {
        // TODO: Implement result type logic here
    }
}

impl LiteralExpression for LiteralExpressionBase {
    fn value(&self) -> Literal {
        self.value
    }
}

impl ASTNode for LiteralExpressionBase {
    fn evaluate(&self, _thing: &dyn Context) -> Result<Box<dyn Any>, EvaluationError> {
        Ok(self.value)
    }

    fn reduce(&self) -> Box<dyn ASTNode> {
        // Implement reduction logic here
    }
}

// TODO: Implement other Literal Expressions similarly (BooleanExpression, DatetimeExpression, etc.)

////////////////////////////////////////////////////////////////////////////////
// Context Trait
////////////////////////////////////////////////////////////////////////////////
trait Context {
    // TOOD: Implement context in engine.rs
}

// TODO: THis is a placeholder for now, we need to implement the Context trait
enum Literal {
    Undefined,
    Boolean(bool),
    Datetime(/* datetime value */),
    Timedelta(/* timedelta value */),
    Float(/* float value */),
    Mapping(HashMap<Literal, Literal>),
    Null,
    Set(HashSet<Literal>),
    String(String),
    Array(Vec<Literal>),
}
