extern crate libc;
extern crate libqmlbind_sys as ffi;

use engine;

use std::ffi::CString;


pub struct Component {
    component: *mut ffi::qmlbind_component,
}

impl Drop for Component {
    fn drop(&mut self) {
        unsafe {
            assert!(!self.component.is_null());
            ffi::qmlbind_component_release(self.component);
        }
    }
}


impl Component {
    // FIXME: Change to Result instead of Option
    pub fn load_path(engine: &engine::Engine, path: &str) -> Option<Component> {
        let component = unsafe { ffi::qmlbind_component_new(engine.get_engine()) };
        let mut component_option: Option<Component> = None;
        if !component.is_null() {
            // FIXME: Remove unwrap
            let s = CString::new(path).unwrap();
            unsafe { ffi::qmlbind_component_load_path(component, s.as_ptr() ); }

            // Verify error
            unsafe {
                let errorString = ffi::qmlbind_component_get_error_string(component);
                println!("errorString: {:?}", errorString);
            }

            // let errorChar = ffi::qmlbind_string_get_chars(errorString);
            // println!("errorChar: {:?}", errorChar);

            component_option = Some(Component { component: component })
        }

        component_option
    }
}
