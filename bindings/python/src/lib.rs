use pyo3::prelude::*;

#[pyfunction]
fn hello_world() -> &'static str {
    zlicenser::hello_world()
}

#[pymodule]
fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}
