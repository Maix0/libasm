extern crate tester;
use tester::*;

fn helper(input: &[u8]) {
    let mut dest_arg: Box<[std::mem::MaybeUninit<u8>]> = Box::new_uninit_slice(input.len() + 10);
    let len = dest_arg.len();
    dest_arg.iter_mut().for_each(|p| {
        p.write(0);
    });
    for (i, ptr) in dest_arg[input.len()..len].iter_mut().enumerate() {
        ptr.write(i as u8);
    }

    let dest_ret =
        unsafe { tester::libasm::ft_strcpy(dest_arg.as_mut_ptr().cast(), input.as_ptr().cast()) };
    assert_eq!(dest_arg.as_mut_ptr().cast(), dest_ret);

    for (i, ptr) in dest_arg[input.len()..len].iter().enumerate() {
        assert_eq!(unsafe { ptr.assume_init_read() }, i as u8);
    }
    for (i, &b) in input.iter().enumerate() {
        assert_eq!(b, unsafe { dest_arg[i].assume_init_read() });
        if b == 0 {
            break;
        }
    }
}

macro_rules! strcmp_test {
    ($name:ident: $input:expr) => {
        #[test]
        fn $name() {
            helper($input);
        }
    };
}

strcmp_test!(empty_string: c"".to_bytes_with_nul());
strcmp_test!(long_string: c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_bytes_with_nul());
strcmp_test!(null_byte_inside: n!(b"01234\x00AAA"));
strcmp_test!(bible: BIBLE);

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
        libasm::ft_strcpy(
            std::ptr::null_mut(),
            c"This is a nice C string you got there".as_ptr(),
        )
    };
    assert!(ret.is_null());
}
