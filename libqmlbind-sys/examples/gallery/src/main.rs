extern crate libc;
extern crate libqmlbind_sys as ffi;

use libc::c_char;
use libc::c_int;

use std::ffi::CString;
use std::ffi::CStr;


fn main() {
    // Port of https://github.com/seanchas116/libqmlbind/blob/master/examples/hello_world_window/main.c

    // http://stackoverflow.com/a/34379937/178154
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe {
        let app = ffi::qmlbind_application_new(c_args.len() as c_int, c_args.as_ptr());
        let engine = ffi::qmlbind_engine_new();
        let component = ffi::qmlbind_component_new(engine);
        ffi::qmlbind_component_load_url(component, CString::new("qrc:/gallery.qml").unwrap().as_ptr());

        let error_string = ffi::qmlbind_component_get_error_string(component);
        if !error_string.is_null() {
            let error_char = CStr::from_ptr(ffi::qmlbind_string_get_chars(error_string));
            println!("Error loading component: {:?}", error_char);
        }

        let instance = ffi::qmlbind_component_create(component);

        let exit_code = ffi::qmlbind_application_exec(app);

        ffi::qmlbind_value_release(instance);
        ffi::qmlbind_component_release(component);
        ffi::qmlbind_engine_release(engine);
        ffi::qmlbind_application_release(app);
    }
}
