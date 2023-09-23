use std::fmt;

// mod errors {
//     pub struct EvaluationError(pub String);
//     pub const UNDEFINED: &str = "undefined";
// }
use crate::errors;

fn assert_is_integer_number(values: &[i64]) -> Result<(), errors::EvaluationError> {
    if !values.iter().all(|&val| val.is_integer_number()) {
        return Err(errors::EvaluationError{ message: "data type mismatch (not an integer number)".to_string() });
    }
    Ok(())
}

fn assert_is_natural_number(values: &[u64]) -> Result<(), errors::EvaluationError> {
    if !values.iter().all(|&val| val.is_natural_number()) {
        return Err(errors::EvaluationError { message: "data type mismatch (not a natural number)".to_string() });

    }
    Ok(())
}

fn assert_is_numeric(values: &[f64]) -> Result<(), errors::EvaluationError> {
    if !values.iter().all(|&val| val.is_numeric()) {
        return Err(errors::EvaluationError { message: "data type mismatch (not a numeric value)".to_string() });
    }
    Ok(())
}

fn assert_is_string(values: &[&str]) -> Result<(), errors::EvaluationError> {
    if !values.iter().all(|&val| val.is_string()) {
        return Err(errors::EvaluationError { message: "data type mismatch (not a string value)".to_string()});
    }
    Ok(())
}

fn is_reduced(values: &[&LiteralExpressionBase]) -> bool {
    values.iter().all(|&val| val.is_reduced())
}

fn iterable_member_value_type<T>(value: &[T]) -> Result<(), errors::EvaluationError> {
    // Implement the logic for iterable_member_value_type here
    Ok(())
}

struct DataType;

impl DataType {
    fn from_value(value: &str) -> Option<Self> {
        // Implement the logic for DataType::from_value here
        None
    }
}

#[derive(Debug)]
struct Assignment {
    name: String,
    value: String,
    value_type: Option<DataType>,
}

impl Assignment {
    fn new(name: String, value: Option<String>, value_type: Option<DataType>) -> Self {
        let value_type = if value != errors::Undefined:: {
            DataType::from_value(&value)
        } else {
            None
        };
        let value_type = DataType::from_value(&value);

        Assignment {
            name,
            value,
            value_type,
        }
    }
}

impl fmt::Debug for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{} name={!r} value={!r} value_type={!r} >",
            stringify!(Assignment),
            self.name,
            self.value,
            self.value_type
        )
    }
}

#[derive(Debug)]
struct ASTNodeBase;

impl ASTNodeBase {
    fn to_graphviz(&self, digraph: &mut Digraph) {
        digraph.node(self.id(), self.class_name());
    }

    fn build(&self, args: &[&dyn std::any::Any], kwargs: &[(&str, &dyn std::any::Any)]) -> Box<dyn ASTNodeBase> {
        Box::new(self.reduce())
    }

    fn evaluate(&self, thing: &dyn std::any::Any) -> Result<Box<dyn std::any::Any>, errors::EvaluationError> {
        // Implement the logic for evaluate here
        unimplemented!()
    }

    fn reduce(&self) -> Box<dyn ASTNodeBase> {
        Box::new(self.clone())
    }

    fn id(&self) -> &str {
        // Implement the logic for id here
        unimplemented!()
    }

    fn class_name(&self) -> &str {
        // Implement the logic for class_name here
        unimplemented!()
    }
}

#[derive(Debug)]
struct LiteralExpressionBase {
    is_reduced: bool,
}

impl LiteralExpressionBase {
    fn is_reduced(&self) -> bool {
        self.is_reduced
    }
}

#[derive(Debug)]
struct Comment {
    value: String,
}

impl Comment {
    fn new(value: String) -> Self {
        Comment { value }
    }
}

impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {!r}>", stringify!(Comment), self.value)
    }
}

impl ASTNodeBase {
    fn to_graphviz(&self, digraph: &mut Digraph, args: &[&dyn std::any::Any], kwargs: &[(&str, &dyn std::any::Any)]) {
        digraph.node(self.id(), &format!("{}\n{!r}", self.class_name(), self.value));
    }
}