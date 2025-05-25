use crate::libasm::{List, ft_list_size};

#[test]
fn len_valid() {
    let mut lst = List::new(0);
    (1..8).for_each(|i| lst.insert_after(i));

    assert_eq!(
        unsafe { ft_list_size((&*lst as *const List<_>).cast::<List<()>>()) },
        lst.size() as i32
    );
}

#[test]
fn len_null() {
    assert_eq!(unsafe { ft_list_size(std::ptr::null()) }, 0);
}
