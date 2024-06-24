#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct Example {}

#[cfg(feature = "python")]
mod submodule {
    use super::*;

    use pyo3::*;

    #[pymethods]
    impl Example {
        #[new]
        fn new() -> Self {
            Example {}
        }
    }
}
