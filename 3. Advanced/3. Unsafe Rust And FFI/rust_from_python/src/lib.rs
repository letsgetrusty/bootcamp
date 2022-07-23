use std::collections::HashMap;
use pyo3::prelude::*;

#[pyfunction]
fn count_words(s: String) -> Py<PyAny> {
    let mut hm = HashMap::new();
    for sub_str in s.split(' ') {
        let count = hm
            .entry(sub_str)
            .or_insert(0);
        *count += 1;
    }
    
    return Python::with_gil(|py| {
        hm.to_object(py)
    });
}

#[pymodule]
fn word_counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}