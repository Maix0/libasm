#![allow(unused)]
use std::ffi::CStr;

use tester::*;

use tester::libasm::ft_list_remove_if;
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

unsafe extern "C" fn less_than<T: std::cmp::Ord>(
    l: *mut libc::c_void,
    r: *mut libc::c_void,
) -> std::ffi::c_int {
    match cmp::<T>(l, r) {
        -1 => 0,
        _ => 1,
    }
}

#[test]
fn cstr() {
    let passthru = (&c"missing" as *const &CStr)
        .cast_mut()
        .cast::<libc::c_void>();

    let slice = &rlist![
        c"This", c" ", c"phrase", c" ", c"is", c" ", c"missing", c" ", c"the", c" ", c"start!",
    ];
    let slice2 = slice.map(|c| c as *const &CStr);
    let mut list = create_list_ptr::<&CStr>(&slice2[..]);
    unsafe { ft_list_remove_if(&raw mut list, passthru, Some(cmp::<&CStr>), None) };

    let out = list_to_vec::<&CStr>(list)
        .into_iter()
        .map(|s| unsafe { *s.as_ref().unwrap() })
        .collect::<Vec<_>>();

    let mut filtered = slice
        .iter()
        .map(|c| **c)
        .filter(|c| unsafe { cmp::<&CStr>(passthru, (c as *const &CStr).cast_mut().cast()) != 0 })
        .collect::<Vec<&CStr>>();
    assert_eq!(out, filtered);
    free_list::<&CStr, false>(list);
}

#[test]
fn i32() {
    let passthru = (&3 as *const i32).cast_mut().cast::<libc::c_void>();

    let slice = &rlist![1, 2, 5, 7, 1, 2, 3, 5, 6, 8, 5, 2, 1];
    let slice2 = slice.map(|c| c as *const i32);
    let mut list = create_list_ptr::<i32>(&slice2[..]);
    unsafe { ft_list_remove_if(&raw mut list, passthru, Some(cmp::<i32>), None) };

    let out = list_to_vec::<i32>(list)
        .into_iter()
        .map(|s| unsafe { *s.as_ref().unwrap() })
        .collect::<Vec<_>>();

    let mut filtered = slice
        .iter()
        .map(|c| **c)
        .filter(|c| unsafe { cmp::<i32>(passthru, (c as *const i32).cast_mut().cast()) != 0 })
        .collect::<Vec<i32>>();
    assert_eq!(out, filtered);
    free_list::<&CStr, false>(list);
}

#[test]
fn i32_less_than() {
    let passthru = (&3 as *const i32).cast_mut().cast::<libc::c_void>();

    let slice = &rlist![1, 2, 5, 7, 1, 2, 3, 5, 6, 8, 5, 2, 1];
    let slice2 = slice.map(|c| c as *const i32);
    let mut list = create_list_ptr::<i32>(&slice2[..]);
    unsafe { ft_list_remove_if(&raw mut list, passthru, Some(less_than::<i32>), None) };

    let out = list_to_vec::<i32>(list)
        .into_iter()
        .map(|s| unsafe { *s.as_ref().unwrap() })
        .collect::<Vec<_>>();

    let mut filtered = slice
        .iter()
        .map(|c| **c)
        .filter(|c| unsafe { less_than::<i32>((c as *const i32).cast_mut().cast(), passthru) != 0 })
        .collect::<Vec<i32>>();
    assert_eq!(out, filtered);
    free_list::<&CStr, false>(list);
}
