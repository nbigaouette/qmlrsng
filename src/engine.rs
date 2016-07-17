extern crate libc;
extern crate libqmlbind_sys as ffi;

use std;

use self::libc::c_char;
use self::libc::c_int;

use std::ffi::CString;


pub struct Engine {
    app: *mut ffi::qmlbind_application,
    engine: *mut ffi::qmlbind_engine,
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            // FIXME: Validate pointers before releasing them
            assert!(!self.engine.is_null());
            assert!(!self.app.is_null());
            ffi::qmlbind_engine_release(self.engine);
            ffi::qmlbind_application_release(self.app);
        }
    }
}


impl Engine {
    pub fn new() -> Engine {
        // Get argc, argv, to pass to application creation.
        // FIXME: Is that really required?
        let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
        // convert the strings to raw pointers
        let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

        let app = unsafe { ffi::qmlbind_application_new(c_args.len() as c_int, c_args.as_ptr()) };
        assert!(!app.is_null());
        let engine = unsafe { ffi::qmlbind_engine_new() };
        assert!(!engine.is_null());
        Engine {
            app: app,
            engine: engine,
        }
    }

    pub fn get_engine(&self) -> *mut ffi::qmlbind_engine {
        self.engine
    }

    pub fn exec(&self) {
        let exit_code = unsafe {
            ffi::qmlbind_application_exec(self.app)
        };
    }
}
