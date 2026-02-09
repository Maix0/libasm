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
