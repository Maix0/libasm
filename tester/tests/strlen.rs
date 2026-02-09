extern crate tester;
use tester::*;

fn helper(input: &[u8], expected: usize) {
    assert_eq!(
        input.iter().position(|v| *v == 0),
        Some(expected),
        "Test is wrongly setup: null byte is at wrong position (or none exists)"
    );
    let real_output = unsafe { tester::libasm::ft_strlen(input.as_ptr().cast()) };
    assert_eq!(expected, real_output);
}

macro_rules! strlen_test {
    ($name:ident: $input:expr => $output:expr ) => {
        #[test]
        fn $name() {
            helper($input, $output)
        }
    };
}

strlen_test!(empty_string: n!(b"") => 0);
strlen_test!(long_string: n!(b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA") => 60);
strlen_test!(null_byte_inside: n!(b"01234\x00AAA") => 5);
strlen_test!(bible: BIBLE  => BIBLE.len() - 1);

#[test]
fn null() {
    assert_eq!(unsafe { tester::libasm::ft_strlen(std::ptr::null()) }, 0);
}
