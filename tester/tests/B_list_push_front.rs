#![allow(unused)]
use tester::*;

use tester::libasm::ft_list_push_front;
use tester::libasm::t_list;

#[test]
fn cstr() {
    let slice = &clist![
        c"This", c" ", c"phrase", c" ", c"is", c" ", c"missing", c" ", c"the", c" ", c"start!",
    ];
    let mut list = create_list_ptr::<i8>(&slice[1..]);
    unsafe { ft_list_push_front(&raw mut list, slice[0].cast_mut().cast()) };
    let out = list_to_vec::<i8>(list);
    assert_eq!(slice.len(), out.len());
    assert!(slice
        .iter()
        .zip(out.iter().map(|p| p.cast_const()))
        .all(|(r, l)| *r == l))
}

#[test]
fn u8() {
    let slice = &rlist![1, 2, 3, 4, 5, 6, 7, 8,];
    let mut list = create_list::<u8>(&slice[1..]);
    unsafe { ft_list_push_front(&raw mut list, (slice[0] as *const u8).cast_mut().cast()) };
    let out = list_to_vec::<u8>(list);
    assert_eq!(slice.len(), out.len());
    assert!(slice
        .iter()
        .zip(out.iter().map(|p| p.cast_const()))
        .all(|(r, l)| unsafe { **r == *l }))
}

#[test]
fn from_empty() {
    let slice = &rlist!("hello", " I am ", "Some strings");
    let mut list = std::ptr::null_mut::<t_list>();
    for &p in slice {
        unsafe { ft_list_push_front(&raw mut list, (p as *const &str).cast_mut().cast()) };
    }
    let out = list_to_vec::<&str>(list)
        .into_iter()
        .map(|p| unsafe { *p })
        .collect::<Vec<&str>>();
    assert_eq!(slice.len(), out.len());
    let mut reversed = slice.iter().map(|s| **s).collect::<Vec<&str>>();
    reversed.reverse();
    assert_eq!(reversed, out);
}
