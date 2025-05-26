use crate::libasm::{List, ft_list_new};

#[test]
fn lst_new() {
    let new_node = unsafe {
        Box::from_raw(ft_list_new(Box::into_raw(Box::new(127i32)).cast()).cast::<List<i32>>())
    };
    assert_eq!(new_node.data(), Some(&127));
    assert!(new_node.next().is_none());
}
