use std::{fmt, time::SystemTime};
use pyo3::create_exception;

// TODO: Define a base class named `Discount`, with a `percentage` attribute.
//  It should be possible to access the `percentage` attribute of a `Discount`.
//  It should also be possible to modify the `percentage` attribute of a `Discount`.
//  It must be enforced that the `percentage` attribute is a float between 0. and 1.
//  Then define two subclasses:
//  - `SeasonalDiscount` that inherits from `Discount` with two additional attributes, `to` and `from_`.
//    `from_` is a datetime object that represents the start of the discount period.
//    `to` is a datetime object that represents the end of the discount period.
//     Both `from_` and `to` should be accessible and modifiable.
//     The class should enforce that `from` is before `to`.
//  - `CappedDiscount` that inherits from `Discount` with an additional attribute `cap`.
//    `cap` is a float that represents the maximum discount (in absolute value) that can be applied.
//    It should be possible to access and modify the `cap` attribute.
//    The class should enforce that `cap` is a non-zero positive float.
//
// All classes should have a method named `apply` that takes a price (float) as input and
// returns the discounted price.
// `SeasonalDiscount` should raise an `ExpiredDiscount` exception if `apply` is called but
// the current date is outside the discount period.
use pyo3::{exceptions::{PyOSError, PyValueError}, prelude::*};

// #[derive(Debug)]
// struct ExpiredDiscount;

// impl std::error::Error for ExpiredDiscount {}

// impl fmt::Display for ExpiredDiscount {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Oh no!")
//     }
// }

// impl std::convert::From<ExpiredDiscount> for PyErr {
//     fn from(err: ExpiredDiscount) -> PyErr {
//         PyOSError::new_err(err.to_string())
//     }
// }

create_exception!(outro2, ExpiredDiscount, pyo3::exceptions::PyException);

#[pyclass(subclass)]
struct Discount {
    #[pyo3(get)]
    percentage: f64,
}

#[pymethods]
impl Discount {
    #[new]
    fn new(percentage: f64) -> PyResult<Self> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err(PyValueError::new_err("Percentage must be between 0 and 1"));
        }
        Ok(Discount { percentage })
    }

    #[setter]
    fn set_percentage(&mut self, percentage: f64) -> PyResult<()> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err(PyValueError::new_err("Percentage must be between 0 and 1"));
        }
        self.percentage = percentage;
        Ok(())
    }

    fn apply(&self, price: f64) -> f64 {
        price * (1.0 - self.percentage)
    }
}

#[pyclass(extends=Discount)]
struct SeasonalDiscount {
    from_: SystemTime,
    to: SystemTime,
}

#[pymethods]
impl SeasonalDiscount {
    #[new]
    fn new(percentage: f64, from_: SystemTime, to: SystemTime) -> PyResult<PyClassInitializer<Self>> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err(PyValueError::new_err("Percentage must be between 0 and 1"));
        }
        let discount = Discount::new(percentage).unwrap();
        let seasonal_discount = SeasonalDiscount { from_, to };
        Ok(PyClassInitializer::from(discount).add_subclass(seasonal_discount))
    }

    #[getter]
    fn from_(&self) -> PyResult<SystemTime> {
        Ok(self.from_)
    }

    #[setter]
    fn set_from_(&mut self, from_: SystemTime) -> PyResult<()> {
        if from_ > self.to {
            return Err(PyValueError::new_err("`from_` date must be before `to` date"));
        }
        self.from_ = from_;
        Ok(())
    }

    #[getter]
    fn to(&self) -> PyResult<SystemTime> {
        Ok(self.to)
    }

    #[setter]
    fn set_to(&mut self, to: SystemTime) -> PyResult<()> {
        if to < self.from_ {
            return Err(PyValueError::new_err("to must be after from_."));
        }
        self.to = to;
        Ok(())
    }

    fn apply(mut self_: PyRefMut<'_, Self>, price: f64) -> PyResult<f64> {
        let now = SystemTime::now();
        if now < self_.from_ || now > self_.to {
            return Err(ExpiredDiscount::new_err("Discount period has expired."));
        }
        Ok(price * (1.0 - self_.as_super().percentage))
    }
}

#[pyclass(extends=Discount)]
struct CappedDiscount {
    cap: f64,
}

#[pymethods]
impl CappedDiscount {
    #[new]
    fn new(percentage: f64, cap: f64) -> PyResult<PyClassInitializer<Self>> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err(PyValueError::new_err("Percentage must be between 0 and 1"));
        }
        if cap <= 0.0 {
            return Err(PyValueError::new_err("Cap must be a positive number"));
        }   
        let discount = Discount::new(percentage).unwrap();
        let capped_discount = CappedDiscount { cap };
        Ok(PyClassInitializer::from(discount).add_subclass(capped_discount))
    }

    #[getter]
    fn cap(&self) -> f64 {
        self.cap
    }

    #[setter]
    fn set_cap(&mut self, cap: f64) -> PyResult<()> {
        if cap <= 0.0 {
            return Err(PyValueError::new_err("Cap must be a positive number"));
        }
        self.cap = cap;
        Ok(())
    }

    fn apply(mut self_: PyRefMut<'_, Self>, price: f64) -> f64 {
        let discount = self_.as_super().percentage;
        let discounted_price = price * (1.0 - discount);
        if (price - discounted_price) > self_.cap {
            return self_.cap;
        }
        discounted_price
    }
}

#[pymodule]
fn outro2(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Discount>()?;
    m.add_class::<SeasonalDiscount>()?;
    m.add_class::<CappedDiscount>()?;
    m.add("ExpiredDiscount", py.get_type_bound::<ExpiredDiscount>())?;
    Ok(())
}
