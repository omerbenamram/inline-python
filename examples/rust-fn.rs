use inline_python::{python, Context};
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn rust_print(x: i32) {
	println!("rust: x = {}", x);
}

pub(crate) fn __pyo3_get_function_rust_print_2<'a>(
	args: Python<'a>,
) -> pyo3::PyResult<&'a pyo3::types::PyCFunction> {
	let name = "rust_print\u{0}";
	let name = std::ffi::CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
	let doc = std::ffi::CStr::from_bytes_with_nul(b"\x00").unwrap();
	pyo3::types::PyCFunction::internal_new(
		name,
		doc,
		pyo3::class::PyMethodType::PyCFunctionWithKeywords(__pyo3_raw_rust_print),
		pyo3::ffi::METH_VARARGS | pyo3::ffi::METH_KEYWORDS,
		args.into(),
	)
}

fn main() {
	let c = Context::new();

	c.add_wrapped(wrap_pyfunction!(rust_print));

	c.run(python! {
		x = 123
		print("python: x =", x)
		rust_print(x)
	});
}
