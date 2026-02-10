#![allow(dead_code)]

#[allow(unused, non_camel_case_types)]
pub mod libasm;

#[repr(C)]
pub struct Next<const N: usize, const M: usize> {
    pub l: [u8; N],
    pub r: [u8; M],
}

#[macro_export]
macro_rules! c {
    ($v1:expr, $v2:expr) => {
        const {
            const L: &[u8] = $v1;
            const R: &[u8] = $v2;
            const NEXT: $crate::Next<{ L.len() }, { R.len() }> = $crate::Next {
                l: *L.as_array().unwrap(),
                r: *R.as_array().unwrap(),
            };
            &unsafe {
                std::mem::transmute::<$crate::Next<{ L.len() }, { R.len() }>, [u8; L.len() + R.len()]>(NEXT)
            }
        }
    };
}

#[macro_export]
macro_rules! m {
    ($v:expr $(,)?) => {
        $v
    };

    ($first:expr, $($rest:expr),+ $(,)?) => {
        $crate::c!($first, $crate::m!($($rest ,)+))
    };
}

#[macro_export]
macro_rules! n {
    ($($value:expr),* $(,)?) => {
        $crate::m!($( $value ,)* b"\0")
    };
}

#[macro_export]
macro_rules! l {
    ($value:expr) => {{
        const SLICE: &[u8] = { $value };
        const SLICE_LEN: usize = if SLICE.is_empty() { 0 } else { SLICE.len() - 1 };
        SLICE.first_chunk::<SLICE_LEN>().unwrap()
    }};
}

pub const BIBLE: &[u8] = m!(include_bytes!("../data/bible"), b"\0");
pub const ABCDEF: &[u8] = m!(b"a", b"b", b"c", b"d", b"e", b"f", b"\0");

pub fn create_list<T>(values: &'static [&'static T]) -> *mut libasm::t_list {
    assert!(
        size_of::<*mut T>() == size_of::<*mut ()>(),
        "You can't use fat pointers"
    );

    let mut fake_head: libasm::t_list = libasm::t_list {
        next: std::ptr::null_mut(),
        data: std::ptr::null_mut(),
    };

    let mut cursor = &mut fake_head;
    for ptr in values {
        let new_value: *mut libasm::t_list =
            unsafe { libc::malloc(size_of::<libasm::t_list>()).cast() };
        assert!(!new_value.is_null(), "malloc failed");
        assert!(new_value.is_aligned(), "malloc not aligned");

        unsafe {
            new_value.write(libasm::t_list {
                data: (*ptr as *const T).cast_mut().cast(),
                next: std::ptr::null_mut(),
            })
        }
        cursor.next = new_value;
        unsafe {
            cursor = &mut *cursor.next;
        }
    }

    fake_head.next
}

pub fn create_list_ptr<T>(values: &[*const T]) -> *mut libasm::t_list {
    assert!(
        size_of::<*mut T>() == size_of::<*mut ()>(),
        "You can't use fat pointers"
    );

    let mut fake_head: libasm::t_list = libasm::t_list {
        next: std::ptr::null_mut(),
        data: std::ptr::null_mut(),
    };

    let mut cursor = &mut fake_head;
    for ptr in values {
        let new_value: *mut libasm::t_list =
            unsafe { libc::malloc(size_of::<libasm::t_list>()).cast() };
        assert!(!new_value.is_null(), "malloc failed");
        assert!(new_value.is_aligned(), "malloc not aligned");

        unsafe {
            new_value.write(libasm::t_list {
                data: ptr.cast_mut().cast(),
                next: std::ptr::null_mut(),
            })
        }
        cursor.next = new_value;
        unsafe {
            cursor = &mut *cursor.next;
        }
    }

    fake_head.next
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn list_to_vec<T>(mut list: *mut libasm::t_list) -> Vec<*mut T> {
    assert!(
        size_of::<*mut T>() == size_of::<*mut ()>(),
        "You can't use fat pointers"
    );
    let mut out = Vec::new();

    while !list.is_null() {
        out.push(unsafe { (*list).data.cast::<T>() });
        list = unsafe { (*list).next };
    }

    out
}

#[macro_export]
macro_rules! clist {
    ($($cstr:literal),* $(,)?) => {
        [$($cstr.as_ptr(),)*]
    };
}
#[macro_export]
macro_rules! rlist {
    ($($v:literal),* $(,)?) => {
        [$(&$v,)*]
    };
}
