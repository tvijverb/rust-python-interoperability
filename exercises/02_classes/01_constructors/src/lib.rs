use pyo3::{exceptions::PyValueError, prelude::*};

// TODO: Add a `__new__` constructor to the `ShoppingOrder` class that takes the following arguments:
//  - `name` (non-empty string)
//  - `price` (non-zero integer)
//  - `quantity` (non-zero integer)
//  The constructor should raise a `ValueError` if any of the arguments are invalid.

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    price: u64,
    #[pyo3(get, set)]
    quantity: u64,
}

#[pymethods]
impl ShoppingOrder {
    #[new]
    fn new(name: String, price: f64, quantity: f64) -> PyResult<Self> {
        if name.is_empty() {
            return Err(PyValueError::new_err("name cannot be empty"));
        }
        if price <= 0.0 {
            return Err(PyValueError::new_err("price cannot be zero"));
        }
        if quantity <= 0.0 {
            return Err(PyValueError::new_err("quantity cannot be zero"));
        }
        Ok(ShoppingOrder {
            name,
            price: price as u64,
            quantity: quantity as u64,
        })
    }
}

#[pymodule]
fn constructors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    Ok(())
}
