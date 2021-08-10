use lexical_parse_float::float::RawFloat;
use lexical_parse_float::limits::ExactFloat;

#[test]
fn exponent_fast_path_test() {
    assert_eq!(f32::min_exponent_fast_path(10), -10);
    assert_eq!(f32::max_exponent_fast_path(10), 10);
    assert_eq!(f32::max_exponent_disguised_fast_path(10), 17);

    assert_eq!(f64::min_exponent_fast_path(10), -22);
    assert_eq!(f64::max_exponent_fast_path(10), 22);
    assert_eq!(f64::max_exponent_disguised_fast_path(10), 37);
}

fn slow_f32_power(exponent: usize, radix: u32) -> f32 {
    let mut value: f32 = 1.0;
    for _ in 0..exponent {
        value *= radix as f32;
    }
    value
}

fn slow_f64_power(exponent: usize, radix: u32) -> f64 {
    let mut value: f64 = 1.0;
    for _ in 0..exponent {
        value *= radix as f64;
    }
    value
}

fn pow_fast_path(radix: u32) {
    for exponent in 0..f32::exponent_limit(radix).1 + 1 {
        let exponent = exponent as usize;
        let actual = unsafe { f32::pow_fast_path(exponent, radix) };
        assert_eq!(actual, slow_f32_power(exponent, radix));
    }
    for exponent in 0..f64::exponent_limit(radix).1 + 1 {
        let exponent = exponent as usize;
        let actual = unsafe { f64::pow_fast_path(exponent, radix) };
        assert_eq!(actual, slow_f64_power(exponent, radix));
    }
}

#[test]
fn pow_fast_path_test() {
    pow_fast_path(10);
    if cfg!(feature = "power-of-two") {
        pow_fast_path(2);
        pow_fast_path(4);
        pow_fast_path(8);
        pow_fast_path(16);
        pow_fast_path(32);
    }
    if cfg!(feature = "radix") {
        pow_fast_path(3);
        pow_fast_path(5);
        pow_fast_path(6);
        pow_fast_path(7);
        pow_fast_path(9);
        pow_fast_path(11);
        pow_fast_path(12);
        pow_fast_path(13);
        pow_fast_path(14);
        pow_fast_path(15);
        pow_fast_path(17);
        pow_fast_path(18);
        pow_fast_path(19);
        pow_fast_path(20);
        pow_fast_path(21);
        pow_fast_path(22);
        pow_fast_path(23);
        pow_fast_path(24);
        pow_fast_path(25);
        pow_fast_path(26);
        pow_fast_path(27);
        pow_fast_path(28);
        pow_fast_path(29);
        pow_fast_path(30);
        pow_fast_path(31);
        pow_fast_path(33);
        pow_fast_path(34);
        pow_fast_path(35);
        pow_fast_path(36);
    }
}

fn slow_int_power(exponent: usize, radix: u32) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..exponent {
        value *= radix as u64;
    }
    value
}

fn int_pow_fast_path(radix: u32) {
    for exponent in 0..f64::mantissa_limit(radix) {
        let exponent = exponent as usize;
        let actual = unsafe { f64::int_pow_fast_path(exponent, radix) };
        assert_eq!(actual, slow_int_power(exponent, radix));
    }
}

#[test]
fn int_pow_fast_path_test() {
    int_pow_fast_path(10);
    if cfg!(feature = "power-of-two") {
        int_pow_fast_path(2);
        int_pow_fast_path(4);
        int_pow_fast_path(8);
        int_pow_fast_path(16);
        int_pow_fast_path(32);
    }
    if cfg!(feature = "radix") {
        int_pow_fast_path(3);
        int_pow_fast_path(5);
        int_pow_fast_path(6);
        int_pow_fast_path(7);
        int_pow_fast_path(9);
        int_pow_fast_path(11);
        int_pow_fast_path(12);
        int_pow_fast_path(13);
        int_pow_fast_path(14);
        int_pow_fast_path(15);
        int_pow_fast_path(17);
        int_pow_fast_path(18);
        int_pow_fast_path(19);
        int_pow_fast_path(20);
        int_pow_fast_path(21);
        int_pow_fast_path(22);
        int_pow_fast_path(23);
        int_pow_fast_path(24);
        int_pow_fast_path(25);
        int_pow_fast_path(26);
        int_pow_fast_path(27);
        int_pow_fast_path(28);
        int_pow_fast_path(29);
        int_pow_fast_path(30);
        int_pow_fast_path(31);
        int_pow_fast_path(33);
        int_pow_fast_path(34);
        int_pow_fast_path(35);
        int_pow_fast_path(36);
    }
}
