use pyo3::{prelude::*, types::PyList};


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
fn fibonacci(n: usize) -> Vec<u64> {
    get_fibonacci(n)
}

#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
