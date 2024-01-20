use pyo3::types::{PyAny, PyDict};
use std::collections::HashMap;

use crate::ast::{EvalResultTypes, NestedValue};
use crate::errors::TypeConversionError;

pub fn py_dict_to_hashmap(
    obj: Option<&PyDict>,
) -> Result<HashMap<String, NestedValue>, TypeConversionError> {
    match obj {
        Some(dict) => {
            let mut map = HashMap::new();
            for (key, value) in dict.into_iter() {
                let key = key.extract::<String>().map_err(|_| {
                    TypeConversionError::new("Could not convert value: key is not a string")
                })?;
                let nested_value = convert(value)?;
                map.insert(key, nested_value);
            }
            Ok(map)
        }
        None => Ok(HashMap::new()),
    }
}

fn convert(obj: &PyAny) -> Result<NestedValue, TypeConversionError> {
    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = HashMap::new();
        for (key, value) in dict.into_iter() {
            let key = key.extract::<String>().map_err(|_| {
                TypeConversionError::new("Could not convert value: key is not a string")
            })?;
            let nested_value = convert(value)?;
            map.insert(key, nested_value);
        }
        Ok(NestedValue::Nested(map))
    } else if let Ok(val) = obj.extract::<f64>() {
        Ok(NestedValue::Primitive(EvalResultTypes::Float(val)))
    } else if let Ok(val) = obj.extract::<bool>() {
        Ok(NestedValue::Primitive(EvalResultTypes::Boolean(val)))
    } else if let Ok(val) = obj.extract::<i64>() {
        Ok(NestedValue::Primitive(EvalResultTypes::Integer(val)))
    } else if let Ok(val) = obj.extract::<String>() {
        Ok(NestedValue::Primitive(EvalResultTypes::String(val)))
    } else {
        Err(TypeConversionError::new("Could not convert value"))
    }
}
