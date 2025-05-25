/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   strdup.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/22 14:49:26 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/22 17:52:19 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ffi::{CStr, CString};

use libc::{size_t, strlen};

use crate::libasm::ft_strdup;
fn dup_cstr(s: &CStr) -> CString {
    let v = s.to_bytes().to_vec();
    CString::new(v).unwrap()
}

#[track_caller]
fn check(s: &CStr) {
    unsafe {
        let s = dup_cstr(s);
        let ptr = ft_strdup(s.as_ptr() as *const u8);
        assert!(!ptr.is_null());
        let p = CStr::from_ptr(ptr as *const i8);
        assert_eq!(p.to_bytes(), s.to_bytes());
        assert_ne!(p.as_ptr(), s.as_ptr());
        libc::free(ptr as _);
    };
}

#[test]
fn test1() {
    check(c"");
    check(c"fdshjfdshfsd");
    assert_eq!(unsafe { ft_strdup(std::ptr::null()) }, std::ptr::null_mut());
    check(c"@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
}
