use pyo3::*;

#[pyclass]
pub struct Example {}

mod submodule {
    use super::*;

    #[pymethods]
    impl Example {
        #[new]
        fn new() -> Self {
            Example {}
        }
    }
}
