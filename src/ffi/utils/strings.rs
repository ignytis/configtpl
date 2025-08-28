use std::ffi::{c_char, CStr, CString};

///Converts a *char C type into Rust string.
/// ```
/// use torustiq_common::ffi::utils::strings;
/// assert_eq!(strings::cchar_to_string(c"Hello, World!".as_ptr()),
///            String::from("Hello, World!"));
/// ```
pub fn cchar_to_string(c: *const c_char) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().to_string() }
}

/// Converts String to char* C type.
/// NB: the output of this function must be deallocated later using 'cchar_const_deallocate' function
pub fn string_to_cchar<S: Into<String>>(s: S) -> *const c_char {
    CString::new(s.into())
        .expect("Failed to convert String to c_char")
        .into_raw()
}

/// Deallocates memory for C string.
/// This function should be called for strings created by *_to_cchar functions above
pub fn cchar_const_deallocate(c: *const c_char) {
    if c.is_null() {
        return
    }

    let _ = unsafe { CString::from_raw(c as *mut c_char) };
}