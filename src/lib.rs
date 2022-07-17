//! How to use this crate
//! # Adding this as a dependency
//! ```rust, ignore
//! [dependencies]
//! wasmedge_hostfunctionmatrix_interface = "^1.0.0"
//! ```
//!
//! # Bringing this into scope
//! ```rust, ignore
//! use wasmedge_hostfunctionmatrix_interface::*;
//! ```
use std::ffi::CString;

pub mod wasmedge_hostfunctionmatrix {
    use std::os::raw::c_char;
    #[link(wasm_import_module = "host_function_matrix")]
    extern "C" {
        pub fn host_function_matrix_determinant(index: u32) -> f64;
        pub fn host_function_matrix_print(index: u32);
        pub fn host_function_matrix_input(newmatrix: *const c_char, len: u32) -> u32;
    }
}

pub fn determinant(index: u32) -> f64 {
    let res: f64;
    unsafe {
        res = wasmedge_hostfunctionmatrix::host_function_matrix_determinant(index);
    }
    res
}


pub fn print(index: u32) {
    unsafe {
        wasmedge_hostfunctionmatrix::host_function_matrix_print(index);
    }
}

pub fn input<S: AsRef<str>>(input: S) -> u32 {
    let input = CString::new((input.as_ref()).as_bytes()).expect("");
    let queue_size: u32;
    unsafe {
        queue_size = wasmedge_hostfunctionmatrix::host_function_matrix_input(
            input.as_ptr(),
            input.as_bytes().len() as u32,
            );
    }
    queue_size
}