#![allow(long_running_const_eval)]

macro_rules! strcmp_test {
    ($name:ident: $lhs:expr, $rhs:expr => $output:expr ) => {
        #[test]
        fn $name() {
            let lhs: *const _ = $lhs;
            let rhs: *const _ = $rhs;
            let output: ::std::ffi::c_int = { $output };

            let real_output = unsafe {
                crate::libasm::ft_strcmp(
                    lhs as *const ::std::ffi::c_char,
                    rhs as *const ::std::ffi::c_char,
                )
            };
            assert_eq!(output.signum(), real_output.signum());
        }
    };
}

// EQ
strcmp_test!(empty_string: c"".as_ptr(), c"".as_ptr() => 0);
strcmp_test!(long_string: c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_ptr(), c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_ptr() => 0);
strcmp_test!(null_byte_inside: crate::n!(b"01234\x00AAA").as_ptr(), c"01234".as_ptr() => 0);
strcmp_test!(bible: crate::BIBLE.as_ptr(), crate::BIBLE.as_ptr()  => 0);
strcmp_test!(null_null: std::ptr::null(), std::ptr::null() => 0);
strcmp_test!(bible2: crate::BIBLE.as_ptr() , crate::BIBLE.as_ptr()  => 0);

// -1
strcmp_test!(bible_with_diff: crate::BIBLE.as_ptr() , crate::m!(crate::BIBLE.first_chunk::<{crate::BIBLE.len() -1} >().unwrap(), b"A", b"0").as_ptr() => -1);
strcmp_test!(smth_null: c"".as_ptr(), std::ptr::null() => -1);
strcmp_test!(aaa_bbb: c"AAA".as_ptr(), c"BBB".as_ptr() => -1);

// 1
strcmp_test!(bible_with_diff_2: crate::m!(crate::BIBLE.first_chunk::<{crate::BIBLE.len() -1} >().unwrap(), b"A", b"0").as_ptr(), crate::BIBLE.as_ptr() =>  1);
strcmp_test!(null_smth: std::ptr::null(), c"".as_ptr() =>  1);
strcmp_test!(bbb_aaa: c"BBB".as_ptr(), c"AAA".as_ptr() =>  1);
