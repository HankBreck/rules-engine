use pyo3::types::{PyAny, PyDict, PyFloat, PyInt, PyString};
use pyo3::PyResult;

use crate::ast::EvalResultTypes;

/// Get a potentially nested value from a python dict.
///
/// # Arguments
///
/// * `py_dict` - The python dict to get the value from
/// * `keys` - The keys to traverse to get the value
///
/// # Returns
///
/// * `Ok(Some(EvalResultTypes))` - The value if it exists
/// * `Ok(None)` - The value does not exist
/// * `Err(PyErr)` - An error occurred
pub fn get_value_from_py_dict(
    py_dict: &PyDict,
    keys: &[&str],
) -> PyResult<Option<EvalResultTypes>> {
    let mut current_value: &PyAny = py_dict.as_ref();
    for &key in keys {
        match current_value.get_item(key) {
            Ok(value) => current_value = value,
            Err(_) => return Ok(None),
        }
    }
    try_into_eval_result_types(current_value).map(Some)
}

fn try_into_eval_result_types(value: &PyAny) -> PyResult<EvalResultTypes> {
    if let Ok(py_str) = value.extract::<&PyString>() {
        return Ok(EvalResultTypes::String(py_str.to_string()));
    }
    if let Ok(py_int) = value.extract::<&PyInt>() {
        return Ok(EvalResultTypes::Float(py_int.extract()?));
    }
    if let Ok(py_float) = value.extract::<&PyFloat>() {
        return Ok(EvalResultTypes::Float(py_float.extract()?));
    }
    if let Ok(py_bool) = value.extract::<bool>() {
        return Ok(EvalResultTypes::Boolean(py_bool));
    }
    Err(pyo3::exceptions::PyTypeError::new_err("Unsupported type"))
}
