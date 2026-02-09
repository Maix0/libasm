use std::ffi::{c_char, c_void, c_int, c_uint};

#[repr(C)]
struct List {
    data: *mut c_char,
    next: *mut List,
}

unsafe extern "C" {
    unsafe fn ft_strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}
