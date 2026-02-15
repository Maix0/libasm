#![allow(unused)]
use tester::*;

use tester::libasm::ft_list_size;
use tester::libasm::t_list;

#[test]
fn cstr() {
    let slice = &clist![
        c"This", c" ", c"phrase", c" ", c"is", c" ", c"missing", c" ", c"the", c" ", c"start!",
    ];
    let mut list = create_list_ptr::<i8>(&slice[..]);
    let mut list_size = unsafe { ft_list_size(list) };
    assert!(list_size >= 0);
    assert_eq!(slice.len(), list_size as usize);
    free_list::<i8, false>(list);
}

#[test]
fn u8() {
    let slice = &rlist![1, 2, 3, 4, 5, 6, 7, 8,];
    let mut list = create_list::<u8>(&slice[..]);
    let mut list_size = unsafe { ft_list_size(list) };
    assert!(list_size >= 0);
    assert_eq!(slice.len(), list_size as usize);
    free_list::<u8, false>(list);
}

#[test]
fn null() {
    assert_eq!(unsafe { ft_list_size(std::ptr::null_mut()) }, 0);
}
