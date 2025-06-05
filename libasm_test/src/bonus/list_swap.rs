/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   list_swap.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/03 18:19:03 by maiboyer          #+#    #+#             */
/*   Updated: 2025/06/05 18:45:17 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::libasm::{List, ft_list_size, ft_list_swap};

fn to_ptr<T>(s: &mut Box<List<T>>) -> *mut List<()> {
    (&mut **s) as *mut List<T> as *mut List<()>
}

#[test]
fn len2() {
    let mut lst = List::new(0);
    lst.insert_after(1);

    assert_eq!(lst.to_vec(), [0, 1]);
    let ptr_lhs = to_ptr(lst.nth_mut(0).unwrap());
    let ptr_rhs = to_ptr(lst.nth_mut(1).unwrap());
    unsafe {
        ft_list_swap(ptr_lhs, ptr_rhs);
    }

    assert_eq!(lst.to_vec(), [1, 0]);
}

#[test]
fn len8() {
    let mut lst = List::new(0);
    for i in (1..8).rev() {
        lst.insert_after(i);
    }

    const IDX: &[(usize, usize)] = &[(0, 1), (2, 5), (5, 1), (7, 3)];

    let mut lvec = lst.to_vec();
    assert_eq!(lvec, [0, 1, 2, 3, 4, 5, 6, 7]);
    for &(l, r) in IDX {
        let ptr_lhs = to_ptr(lst.nth_mut(l).unwrap());
        let ptr_rhs = to_ptr(lst.nth_mut(r).unwrap());
        unsafe {
            ft_list_swap(ptr_lhs, ptr_rhs);
        }
        lvec.swap(l, r);
        assert_eq!(lst.to_vec(), lvec);
    }
}

#[test]
fn len_null() {
    assert_eq!(unsafe { ft_list_size(std::ptr::null()) }, 0);
}
