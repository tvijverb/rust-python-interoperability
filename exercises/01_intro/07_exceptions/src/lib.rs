use pyo3::{prelude::*, types::*, exceptions::*};

fn get_fibonacci(n: usize) -> Vec<u64> {
    let mut fibonacci = vec![];
    if n == 0 {
        return fibonacci;
    }
    if n > 0 {
        fibonacci.push(0);
    }
    if n > 1 {
        fibonacci.push(1);
    }
    for i in 2..n {
        fibonacci.push(fibonacci[i - 1] + fibonacci[i - 2]);
    }
    fibonacci
}

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
fn fibonacci(_py: Python<'_>, item: Bound<'_, PyAny>) -> PyResult<Vec<u64>> {
    let n = item.extract::<i64>()?;
    if n < 0 {
        return Err(PyErr::new::<PyTypeError, _>("n must be greater than or equal to 0"));
    }
    Ok(get_fibonacci(n as usize))
}

#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
