#![allow(unused)]
use std::ffi::CStr;

use tester::*;

use tester::libasm::ft_list_sort;
use tester::libasm::t_list;

unsafe extern "C" fn reverse_cmp<T: std::cmp::Ord>(
    l: *mut libc::c_void,
    r: *mut libc::c_void,
) -> std::ffi::c_int {
    -cmp::<T>(l, r)
}

unsafe extern "C" fn cmp<T: std::cmp::Ord>(
    l: *mut libc::c_void,
    r: *mut libc::c_void,
) -> std::ffi::c_int {
    let l: *mut T = l.cast();
    let r: *mut T = r.cast();

    if l == r {
        return 0;
    }
    if l.is_null() {
        return -1;
    }
    if r.is_null() {
        return 1;
    }

    assert!(!l.is_null() && l.is_aligned());
    assert!(!r.is_null() && r.is_aligned());

    (unsafe { std::cmp::Ord::cmp(l.as_ref().unwrap(), r.as_ref().unwrap()) } as i8).into()
}

#[test]
fn cstr() {
    let slice = &rlist![
        c"This", c" ", c"phrase", c" ", c"is", c" ", c"missing", c" ", c"the", c" ", c"start!",
    ];
    let slice2 = slice.map(|c| c as *const &CStr);
    let mut list = create_list_ptr::<&CStr>(&slice2[..]);
    unsafe { ft_list_sort(&raw mut list, Some(cmp::<&CStr>)) };

    let out = list_to_vec::<&CStr>(list)
        .into_iter()
        .map(|s| unsafe { *s.as_ref().unwrap() })
        .collect::<Vec<_>>();

    let mut sorted = slice.iter().map(|c| **c).collect::<Vec<&CStr>>();
    sorted.sort();
    assert_eq!(out, sorted);
    free_list::<&CStr, false>(list);
}

#[test]
fn i32() {
    let slice = &rlist![1, 2, 5, 7, 1, 2, 3, 5, 6, 8, 5, 2, 1];
    let slice2 = slice.map(|c| c as *const i32);
    let mut list = create_list_ptr::<i32>(&slice2[..]);
    unsafe { ft_list_sort(&raw mut list, Some(cmp::<i32>)) };

    let out = list_to_vec::<i32>(list)
        .into_iter()
        .map(|s| unsafe { *s.as_ref().unwrap() })
        .collect::<Vec<_>>();

    let mut sorted = slice.iter().map(|c| **c).collect::<Vec<i32>>();
    sorted.sort();
    assert_eq!(out, sorted);
    free_list::<i32, false>(list);
}


#[test]
fn i32_reversed() {
    let slice = &rlist![1, 2, 5, 7, 1, 2, 3, 5, 6, 8, 5, 2, 1];
    let slice2 = slice.map(|c| c as *const i32);
    let mut list = create_list_ptr::<i32>(&slice2[..]);
    unsafe { ft_list_sort(&raw mut list, Some(reverse_cmp::<i32>)) };

    let out = list_to_vec::<i32>(list)
        .into_iter()
        .map(|s| unsafe { *s.as_ref().unwrap() })
        .collect::<Vec<_>>();

    let mut sorted = slice.iter().map(|c| **c).collect::<Vec<i32>>();
    sorted.sort();
    sorted.reverse();
    assert_eq!(out, sorted);
    free_list::<i32, false>(list);
}
