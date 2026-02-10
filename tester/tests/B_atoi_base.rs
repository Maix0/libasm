fn rs_atoi_base(input: &[u8], base: &[u8]) -> i32 {
    assert!(input.contains(&0), "input is wrong");
    assert!(base.contains(&0), "base is wrong");
    let mut out: i64 = 0;
    // lets get only up to the null terminator (not included)
    let input = &input[..input.iter().position(|&b| b == 0).unwrap()];
    let base = &base[..base.iter().position(|&b| b == 0).unwrap()];

    // base check
    const INVALID_CHARS: &[u8] = b"+- \x09\x0A\x0B\x0C\x0D";
    const SPACE_CHARS: &[u8] = b" \x09\x0A\x0B\x0C\x0D";
    const SIGN_CHARS: &[u8] = b"+-";

    if base.len() < 2 {
        return 0;
    }

    for i in 0..base.len() {
        let (c, rest) = base.split_at(i + 1);
        let Some(c) = c.last() else {
            continue;
        };
        // no duplicate
        if rest.contains(c) {
            return 0;
        }
        // no invalid chars
        if INVALID_CHARS.contains(c) {
            return 0;
        }
    }
    // bash check done
    let mut idx = 0;
    while idx < input.len() && SPACE_CHARS.contains(&input[idx]) {
        idx += 1;
    }

    let mut sign = 1;
    while idx < input.len() && SIGN_CHARS.contains(&input[idx]) {
        if input[idx] == b'-' {
            sign = -sign;
        }
        idx += 1;
    }

    while let Some(pos) = input
        .get(idx)
        .and_then(|b| base.iter().position(|n| n == b))
    {
        idx += 1;
        out = out.wrapping_mul(base.len() as i64);
        out = out.wrapping_add(pos as i64);
    }

    out.wrapping_mul(sign) as i32
}

fn helper(input: &[u8], base: &[u8], expected: i32) {
    assert!(input.contains(&0), "input is wrong");
    assert!(base.contains(&0), "base is wrong");

    assert_eq!(
        expected,
        rs_atoi_base(input, base),
        "Result not consistant with rust impl"
    );

    let res = unsafe {
        tester::libasm::ft_atoi_base(
            input.as_ptr().cast::<i8>().cast_mut(),
            base.as_ptr().cast::<i8>().cast_mut(),
        )
    };

    assert_eq!(res, expected);
}

macro_rules! atoi_test {
    ($name:ident: $input:expr, $base:expr => $result:expr) => {
        #[test]
        fn $name() {
            helper(tester::n!($input), tester::n!($base), $result);
        }
    };
}

const BASE_10: &[u8] = b"0123456789";

atoi_test!(base_10_normal: b"10992", BASE_10 => 10992);
atoi_test!(base_10_1sign: b"-10992", BASE_10 => -10992);
atoi_test!(base_10_space: b"  \n\n\t\t\t 10992", BASE_10 => 10992);
atoi_test!(base_10_9sign: b"+++---+-+10992", BASE_10 => 10992);
atoi_test!(base_10_space_sign: b"   ++---10992", BASE_10 => -10992);
atoi_test!(base_10_after: b"10992 11111111", BASE_10 => 10992);

const BASE_16U: &[u8] = b"0123456789ABCDEF";

atoi_test!(base_16u_normal: b"10992", BASE_16U => 0x10992);
atoi_test!(base_16u_1sign: b"-10992", BASE_16U => -0x10992);
atoi_test!(base_16u_space: b"  \n\n\t\t\t 10992", BASE_16U => 0x10992);
atoi_test!(base_16u_9sign: b"+++---+-+10992", BASE_16U => 0x10992);
atoi_test!(base_16u_space_sign: b"   ++---10992", BASE_16U => -0x10992);
atoi_test!(base_16u_after: b"10992 11111111", BASE_16U => 0x10992);
atoi_test!(base_16u_full: b"AB192AB", BASE_16U => 0xAB192AB);

const BASE_16L: &[u8] = b"0123456789abcdef";

atoi_test!(base_16l_normal: b"10992", BASE_16L => 0x10992);
atoi_test!(base_16l_1sign: b"-10992", BASE_16L => -0x10992);
atoi_test!(base_16l_space: b"  \n\n\t\t\t 10992", BASE_16L => 0x10992);
atoi_test!(base_16l_9sign: b"+++---+-+10992", BASE_16L => 0x10992);
atoi_test!(base_16l_space_sign: b"   ++---10992", BASE_16L => -0x10992);
atoi_test!(base_16l_after: b"10992 11111111", BASE_16L => 0x10992);
atoi_test!(base_16l_full: b"ab192ab", BASE_16L => 0xab192ab);

atoi_test!(invalid_base_dup: b"1234", b"123356789" => 0);
atoi_test!(invalid_base_short0: b"1234", b"" => 0);
atoi_test!(invalid_base_short1: b"1234", b"A" => 0);
atoi_test!(invalid_base_illegal1: b"1234", b"+1234" => 0);
atoi_test!(invalid_base_illegal2: b"1234", b"123456789A bcdef" => 0);
