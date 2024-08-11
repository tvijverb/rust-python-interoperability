// TODO: Expose a function named `max_k` that takes a list of unsigned integers and return as output
//   a list containing the `k` largest numbers in the list, in descending order.
//
// Hint: you can use the `num_bigint` crate if you think it'd be useful.

use pyo3::{exceptions::{PyTypeError, PyValueError}, prelude::*, types::PyList};

#[pyfunction]
fn max_k(py: Python<'_>, numbers: Bound<'_, PyList>, k: usize) -> PyResult<Vec<u128>> {
    let numbers_result = numbers.extract::<Vec<u128>>();
    if numbers_result.is_err() {
        return Err(PyErr::new::<PyTypeError, _>("The list must contain only unsigned integers"));
    }
    let mut numbers = numbers_result.unwrap();
    if numbers.len() < k {
        return Err(PyErr::new::<PyValueError, _>("k must be less than or equal to the length of the list"));
    }
    numbers.sort_unstable();
    numbers.reverse();
    Ok(numbers.into_iter().take(k).collect())
}

#[pymodule]
fn outro1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_k, m)?)?;
    Ok(())
}