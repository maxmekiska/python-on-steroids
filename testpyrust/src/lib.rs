use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn diff_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a - b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn testpyrust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(diff_as_string, m)?)?;
    Ok(())
}