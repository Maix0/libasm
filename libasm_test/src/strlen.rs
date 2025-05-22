/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   strlen.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:49:26 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/22 15:03:44 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ffi::{CStr, CString};

use libc::{size_t, strlen};

use crate::libasm::ft_strlen;
fn dup_cstr(s: &CStr) -> CString {
    let v = s.to_bytes().to_vec();
    CString::new(v).unwrap()
}

macro_rules! check {
    ($s:literal) => {{
        let s = dup_cstr($s);
        let ptr = s.as_ptr();
        unsafe { assert_eq!(strlen(ptr as *const _), ft_strlen(ptr as *const _) as _) }
    }};
    (@raw_ptr: $ptr:expr) => {{
        let ptr = $ptr;
        unsafe { assert_eq!(strlen(ptr as *const _), ft_strlen(ptr as *const _) as _) }
    }};
}

#[test]
fn test1() {
    check!(c"");
    check!(c"some small string");
    check!(c"somet stuff");
    check!(c"00123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF123456789ABCDEF");
    check!(c"me smol");

    assert_eq!(unsafe { ft_strlen(std::ptr::null()) }, 0);
}
