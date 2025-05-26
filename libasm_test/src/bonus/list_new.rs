use crate::libasm::{List, ft_list_new};

#[test]
fn lst_new() {
    let new_node_ptr = unsafe { ft_list_new(Box::into_raw(Box::new(127)).cast()) };
}
