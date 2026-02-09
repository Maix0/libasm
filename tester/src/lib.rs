#![allow(dead_code)]

extern crate constcat;

#[allow(unused, non_camel_case_types)]
mod libasm;

mod strcmp;
mod strlen;

#[repr(C)]
struct Next<const N: usize, const M: usize> {
    l: [u8; N],
    r: [u8; M],
}

#[macro_export]
macro_rules! two {
    ($v1:expr, $v2:expr) => {
        const {
            const L: &[u8] = $v1;
            const R: &[u8] = $v2;
            const NEXT: $crate::Next<{ L.len() }, { R.len() }> = $crate::Next {
                l: *match L.as_array() {
                    Some(n) => n,
                    None => unreachable!(),
                },
                r: *match R.as_array() {
                    Some(n) => n,
                    None => unreachable!(),
                },
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
        $crate::two!($first, $crate::m!($($rest ,)+))
    };
}

const BIBLE: &[u8] = two!(include_bytes!("../data/bible"), b"\0");
const ABCDEF: &[u8] = m!(b"a", b"b", b"c", b"d", b"e", b"f", b"\0");

#[macro_export]
macro_rules! n {
    ($($value:expr),* $(,)?) => {
        $crate::m!($( $value ,)* b"\0")
    };
}
