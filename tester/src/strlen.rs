macro_rules! strlen_test {
    ($name:ident: $input:expr => $output:expr ) => {
        #[test]
        fn $name() {
            let input: *const ::std::ffi::c_char = { $input };
            let output: usize = { $output };

            let real_output = unsafe { crate::libasm::ft_strlen(input) };
            assert_eq!(output, real_output);
        }
    };
}

strlen_test!(empty_string: c"".as_ptr() => 0);
strlen_test!(long_string: c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_ptr() => 60);
strlen_test!(null_byte_inside: crate::n!(b"01234\x00AAA").as_ptr() as *const _ => 5);
strlen_test!(bible: crate::BIBLE.as_ptr() as _  => crate::BIBLE.len() - 1);
strlen_test!(null: std::ptr::null() => 0);
