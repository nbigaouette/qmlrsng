extern crate libc;
extern crate libqmlbind_sys as ffi;

use libc::c_char;
use libc::c_int;

use std::ffi::CString;


fn main() {
    println!("Example");

    // http://stackoverflow.com/a/34379937/178154
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe {
        ffi::qmlbind_application_new(c_args.len() as c_int, c_args.as_ptr());
    }
}
