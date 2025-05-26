use crate::libasm::{List, ft_list_push_front};

#[test]
fn push_front_existing() {
    let mut lst = List::new(0);
    (1..8).rev().for_each(|i| lst.insert_after(i));
    let mut ptr = Box::into_raw(lst);
    unsafe {
        ft_list_push_front(
            (&raw mut ptr) as *mut *mut _,
            Box::into_raw(Box::new(-1i32)) as *mut _,
        );
    }
    let lst = unsafe { Box::from_raw(ptr) };
    dbg!(&lst);
    assert_eq!(lst.data(), Some(&-1i32));
    assert_eq!(lst.next().and_then(|s| s.data()), Some(&0));
}
