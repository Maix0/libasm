extern crate tester;
use tester::*;

fn helper(
    rhs: *const ::std::ffi::c_char,
    lhs: *const ::std::ffi::c_char,
    output: ::std::ffi::c_int,
) {
    let libc_ouput = match (lhs.is_null(), rhs.is_null()) {
        (false, false) => unsafe { ::libc::strcmp(lhs, rhs) },
        (false, true) => -1,
        (true, false) => 1,
        (true, true) => 0,
    };
    assert_eq!(
        libc_ouput.signum(),
        output.signum(),
        "given output doesnt match libc"
    );

    let real_output = unsafe {
        libasm::ft_strcmp(
            lhs as *const ::std::ffi::c_char,
            rhs as *const ::std::ffi::c_char,
        )
    };
    assert_eq!(output.signum(), real_output.signum());
}

macro_rules! strcmp_test {
    ($name:ident: $lhs:expr, $rhs:expr => $output:expr ) => {
        #[test]
        fn $name() {
            helper($lhs as *const _, $rhs as *const _, $output)
        }
    };
}

// EQ
strcmp_test!(empty_string: c"".as_ptr(), c"".as_ptr() => 0);
strcmp_test!(long_string: c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_ptr(), c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".as_ptr() => 0);
strcmp_test!(null_byte_inside: n!(b"01234\x00AAA").as_ptr(), c"01234".as_ptr() => 0);
strcmp_test!(bible: BIBLE.as_ptr(), BIBLE.as_ptr()  => 0);
strcmp_test!(null_null: std::ptr::null(), std::ptr::null() => 0);
strcmp_test!(bible2: BIBLE.as_ptr() , BIBLE.as_ptr()  => 0);

// -1
strcmp_test!(bible_with_diff: BIBLE.as_ptr() , m!(l!(BIBLE), b"A\0").as_ptr() =>  1);
strcmp_test!(smth_null: c"".as_ptr(), std::ptr::null() => 1);
strcmp_test!(aaa_bbb: c"AAA".as_ptr(), c"BBB".as_ptr() => 1);

// 1
strcmp_test!(bible_with_diff_2: m!(l!(BIBLE), b"A\0").as_ptr(), BIBLE.as_ptr() => -1);
strcmp_test!(null_smth: std::ptr::null(), c"".as_ptr() => -1);
strcmp_test!(bbb_aaa: c"BBB".as_ptr(), c"AAA".as_ptr() => -1);
