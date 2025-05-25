/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   strcmp.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:49:26 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/25 14:18:19 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ffi::{CStr, CString};

use libc::{size_t, strcmp};

use crate::libasm::ft_strcmp;

fn dup_cstr(s: &CStr) -> CString {
    let v = s.to_bytes().to_vec();
    CString::new(v).unwrap()
}

#[track_caller]
fn check(s1: &CStr, s2: &CStr) {
    let s1 = dup_cstr(s1);
    let s2 = dup_cstr(s2);

    let mine = unsafe { ft_strcmp(s1.as_ptr() as _, s2.as_ptr() as _) }; //.signum();
    let libc = unsafe { strcmp(s1.as_ptr() as _, s2.as_ptr() as _) }; //.signum();

    assert_eq!(mine, libc);
}

#[test]
fn test1() {
    //check(c"This is a string", c"This is a string");
    //check(
    //    c"This is a string@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
    //    c"This is a string@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
    //);
    //check(c"", c"");
    //check(c"AAAAA", c"AAAAAA");
    //check(c"AAAAAAB", c"AAAAABA");
    check(c"AAAAABAA", c"AAAAABA");
}
