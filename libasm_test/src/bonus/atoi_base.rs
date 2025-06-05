/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   atoi_base.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: maiboyer <maiboyer@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/27 14:29:26 by maiboyer          #+#    #+#             */
/*   Updated: 2025/05/27 15:14:03 by maiboyer         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{ffi::CStr, num::Wrapping};

fn rs_atoi_base(s: &CStr, base: &CStr) -> i32 {
    let base_len = base.count_bytes();
    let mut out = Wrapping::<i64>(0);
    if base_len < 2 {
        return 0;
    }
    for (idx, c) in base.to_bytes().iter().enumerate() {
        if b" \t\x0C\r\n\x0B+-".contains(c) {
            return 0;
        }
        // with_nul so idx+1 is always something valid
        if (base.to_bytes_with_nul()[(idx + 1)..].contains(c)) {
            return 0;
        }
    }
    let mut s = s.to_bytes();
    let mut idx = 0;
    while b" \t\x0C\r\n\x0B".contains(&s[idx]) && idx < s.len() {
        idx += 1;
    }
    s = &s[idx..];
    let mut idx = 0;
    let mut sign = Wrapping(1i64);
    while b"+-".contains(&s[idx]) && idx < s.len() {
        sign *= -(s[idx] as i8 - b',' as i8) as i64;
        idx += 1;
    }
    s = &s[idx..];
    for chr in s {
        let Some(idx) = base.to_bytes().iter().position(|p| p == chr) else {
            break;
        };
        out *= Wrapping(base_len as i64);
        out += Wrapping(idx as i64);
    }

    (out * sign).0 as i32
}

mod rust {
    use super::*;

    #[test]
    fn rs_atoi_base_test() {
        assert_eq!(rs_atoi_base(c"1234", c"0123456789"), 1234);
        assert_eq!(rs_atoi_base(c"++++FF", c"0123456789ABCDEF"), 0xFF);
        assert_eq!(rs_atoi_base(c"  --++++FF", c"0123456789ABCDEF"), 0xFF);
        assert_eq!(rs_atoi_base(c"  --++++0001", c"0123456789ABCDEF"), 1);
        assert_eq!(rs_atoi_base(c"  -0001", c"0123456789ABCDEF"), -1);
        assert_eq!(rs_atoi_base(c"  +++--0001  FF", c"0123456789ABCDEF"), 1);
        assert_eq!(rs_atoi_base(c"not valid  +FF", c"0123456789ABCDEF"), 0);
        assert_eq!(rs_atoi_base(c"-2147483648", c"0123456789"), i32::MIN);
        assert_eq!(rs_atoi_base(c"2147483647", c"0123456789"), i32::MAX);
        assert_eq!(rs_atoi_base(c"2147483648", c"0123456789"), i32::MIN);
    }
}
