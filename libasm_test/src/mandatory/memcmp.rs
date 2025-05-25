/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   memcmp.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:49:26 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/25 22:15:43 by maiboyer         ###   ########.fr       */
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

    let mine = unsafe { ft_memcmp(s1.as_ptr() as _, s2.as_ptr() as _, s1.len() as _) };
    let libc = unsafe { memcmp(s1.as_ptr() as _, s2.as_ptr() as _, s1.len() as _) };

    assert_eq!(mine, libc);
}

#[track_caller]
fn check_raw(s1: &[u8], s2: &[u8], len: usize) {
    let s1 = dup(s1);
    let s2 = dup(s2);
    assert!(s1.len() >= len);
    assert!(s2.len() >= len);

    let mine = unsafe { ft_memcmp(s1.as_ptr() as _, s2.as_ptr() as _, len as _) };
    let libc = unsafe { memcmp(s1.as_ptr() as _, s2.as_ptr() as _, len as _) };
}

#[test]
fn eq() {
    let a = [b'@'; 1024 + 5];
    check(&a, &a);
    check(b"Something", b"Something");
}

#[test]
fn eq_trunc() {
    check_raw(b"abcD", b"abcd", 3);
    check_raw(b"aAAAAAAA", b"abcd", 1);
}

#[test]
fn same_ptr() {
    let a = dup(b"This is a very long string aaaaaaaa");
    assert_eq!(
        unsafe { ft_memcmp(a.as_ptr() as _, a.as_ptr() as _, a.len() as _) },
        0
    );
    assert_eq!(
        unsafe { ft_memcmp(a.as_ptr() as _, a.as_ptr() as _, (a.len() * 1024) as _) },
        0
    );
    assert_eq!(unsafe { ft_memcmp(a.as_ptr() as _, a.as_ptr() as _, 0) }, 0);
}

fn zero_length() {
    check_raw(
        b"This is a string 1",
        b"I'm not a all like the other one",
        0,
    );
}

#[test]
fn nulls() {
    assert_eq!(
        unsafe { ft_memcmp(std::ptr::null(), b"@@@@".as_ptr(), 4) },
        -256
    );
    assert_eq!(
        unsafe { ft_memcmp(b"@@@@".as_ptr(), std::ptr::null(), 4) },
        256
    );
    assert_eq!(
        unsafe { ft_memcmp(std::ptr::null(), std::ptr::null(), 4) },
        0
    );
}
