use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!name.is_null());

        CStr::from_ptr(name)
    };

    let name = c_str.to_str().unwrap();

    CString::new(format!("Hello, {}", name)).unwrap().into_raw()
}
