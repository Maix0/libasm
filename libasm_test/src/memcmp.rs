/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   memcmp.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:49:26 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/25 14:14:43 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ffi::{CStr, CString};

use libc::{memcmp, size_t};

use crate::libasm::ft_memcmp;

fn dup(s: &[u8]) -> Box<[u8]> {
    s.to_vec().into_boxed_slice()
}

#[track_caller]
fn check(s1: &[u8], s2: &[u8]) {
    assert_eq!(s1.len(), s2.len());
    let s1 = dup(s1);
    let s2 = dup(s2);

    let mine = unsafe { ft_memcmp(s1.as_ptr() as _, s2.as_ptr() as _, s1.len() as _) }; //.signum();
    let libc = unsafe { memcmp(s1.as_ptr() as _, s2.as_ptr() as _, s1.len() as _) }; //.signum();

    assert_eq!(mine, libc);
}

#[test]
fn test1() {
    let a = [b'@'; 1024 + 5];
    check(&a, &a);
    check(b"abcD", b"abcd");
    check(b"abcdef\0\0", b"abcdefA\0")
}
