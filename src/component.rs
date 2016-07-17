extern crate libc;
extern crate libqmlbind_sys as ffi;

use engine;

use std::ffi::{CString,CStr};


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

pub struct ComponentInstance {
    instance: *mut ffi::qmlbind_value,
}

impl Drop for ComponentInstance {
    fn drop(&mut self) {
        unsafe {
            assert!(!self.instance.is_null());
            ffi::qmlbind_value_release(self.instance);
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
                let error_string = ffi::qmlbind_component_get_error_string(component);
                if error_string.is_null() {
                    component_option = Some(Component { component: component })
                } else {
                    let error_char = CStr::from_ptr(ffi::qmlbind_string_get_chars(error_string));
                    println!("Error loading component: {:?}", error_char);
                }
            }
        }

        component_option
    }

    pub fn create(&self) -> Option<ComponentInstance> {
        // FIXME: Proper error handling
        let instance = unsafe { ffi::qmlbind_component_create(self.component) };
        Some(ComponentInstance {
            instance: instance
        })
    }
}

impl ComponentInstance {
    // FIXME: Change to Result
    pub fn get_property(&self, property: &str) -> Option<f64> {
        // FIXME: Set Result instead of unwrapping
        let s = CString::new(property).unwrap();
        let value_ptr = unsafe { ffi::qmlbind_value_get_property(self.instance, s.as_ptr()) };

        assert!(!value_ptr.is_null());

        let property_value = unsafe { ffi::qmlbind_value_get_number(value_ptr) };

        unsafe { ffi::qmlbind_value_release(value_ptr) };

        Some(property_value)
    }

}
