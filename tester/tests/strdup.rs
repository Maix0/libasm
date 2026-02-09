extern crate tester;
use tester::*;

fn helper(input: &[u8]) {
    let dest_ret = unsafe { tester::libasm::ft_strdup(input.as_ptr().cast()) };
    assert!(!dest_ret.is_null());

    for (i, &b) in input.iter().enumerate() {
        assert_eq!(b, unsafe { dest_ret.cast::<u8>().wrapping_add(i).read() });
        if b == 0 {
            break;
        }
    }
    unsafe { ::libc::free(dest_ret.cast()) };
}

macro_rules! strdup_test {
    ($name:ident: $input:expr) => {
        #[test]
        fn $name() {
            helper($input);
        }
    };
}

strdup_test!(empty_string: c"".to_bytes_with_nul());
strdup_test!(long_string: c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_bytes_with_nul());
strdup_test!(null_byte_inside: n!(b"01234\x00AAA"));
strdup_test!(bible: BIBLE);

#[test]
fn null_input() {
    let mut buf = [255u8; 32];

    let ret = unsafe { libasm::ft_strcpy(buf.as_mut_ptr().cast(), std::ptr::null()) };
    assert_eq!(buf.as_mut_ptr().cast(), ret);
    assert_eq!(buf, [255; 32]);
}

#[test]
fn null_output() {
    let ret = unsafe {
        libasm::ft_strdup(
            std::ptr::null_mut(),
        )
    };
    assert!(ret.is_null());
}
