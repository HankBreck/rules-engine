use pyo3::types::PyDict;
use pyo3::Python;
use rust_rule_engine::engine::Rule;

fn main() {
    pyo3::prepare_freethreaded_python();
    let rule_text = String::from("num1 > num2 or num3 < num4");
    let rule = Rule::new(rule_text).unwrap();
    let _ = &Python::with_gil(|py| {
        let dict = pyo3::types::PyDict::new(py);
        populate_dict(dict, 1000);
        let sub_dict1 = pyo3::types::PyDict::new(py);
        populate_dict(&sub_dict1, 1000);
        let sub_dict2 = pyo3::types::PyDict::new(py);
        populate_dict(&sub_dict2, 1000);
        dict.set_item("dict1", sub_dict1).unwrap();
        dict.set_item("dict2", sub_dict2).unwrap();
        for _ in 0..100_000 {
            rule.matches(Some(dict));
        }
    });
}

fn populate_dict(dict: &PyDict, count: usize) {
    for i in 0..count {
        dict.set_item(&format!("num{}", i), i).unwrap();
    }
}
