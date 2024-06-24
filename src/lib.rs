use dep_crate::Example;
use pyo3::*;
use types::PyModule;

#[pymodule]
pub fn pybinding_repro(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Example>()?;
    Ok(())
}
