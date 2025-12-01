// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use ngrs::*;
use std::sync::Once;
use num_complex::Complex;

static INIT: Once = Once::new();

#[ctor::ctor]
fn global_init() {
    INIT.call_once(|| {
        Runtime::initialize();
    });
}

#[test]
fn can_convert_int8() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_i8).try_into(), Ok(42_i8));
        assert_eq!(SCM::from(i8::MIN).try_into(), Ok(i8::MIN));
        assert_eq!(SCM::from(i8::MAX).try_into(), Ok(i8::MAX));
    });
}

#[test]
fn can_convert_uint8() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_u8).try_into(), Ok(42_u8));
        assert_eq!(SCM::from(u8::MIN).try_into(), Ok(u8::MIN));
        assert_eq!(SCM::from(u8::MAX).try_into(), Ok(u8::MAX));
    });
}

#[test]
fn can_convert_int16() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_i16).try_into(), Ok(42_i16));
        assert_eq!(SCM::from(i16::MIN).try_into(), Ok(i16::MIN));
        assert_eq!(SCM::from(i16::MAX).try_into(), Ok(i16::MAX));
    });
}

#[test]
fn can_convert_uint16() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_u16).try_into(), Ok(42_u16));
        assert_eq!(SCM::from(u16::MIN).try_into(), Ok(u16::MIN));
        assert_eq!(SCM::from(u16::MAX).try_into(), Ok(u16::MAX));
    });
}

#[test]
fn can_convert_int32() {
    with_guile(|_| {
        assert_eq!(SCM::from(32_i32).try_into(), Ok(32_i32));
        assert_eq!(SCM::from(i32::MIN).try_into(), Ok(i32::MIN));
        assert_eq!(SCM::from(i32::MAX).try_into(), Ok(i32::MAX));
    });
}

#[test]
fn can_convert_uint32() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_u32).try_into(), Ok(42_u32));
        assert_eq!(SCM::from(u32::MIN).try_into(), Ok(u32::MIN));
        assert_eq!(SCM::from(u32::MAX).try_into(), Ok(u32::MAX));
    });
}

#[test]
fn can_convert_int64() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_i64).try_into(), Ok(42_i64));
        assert_eq!(SCM::from(i64::MIN).try_into(), Ok(i64::MIN));
        assert_eq!(SCM::from(i64::MAX).try_into(), Ok(i64::MAX));
    });
}

#[test]
fn can_convert_uint64() {
    with_guile(|_| {
        assert_eq!(SCM::from(42_u64).try_into(), Ok(42_u64));
        assert_eq!(SCM::from(u64::MIN).try_into(), Ok(u64::MIN));
        assert_eq!(SCM::from(u64::MAX).try_into(), Ok(u64::MAX));
    });
}

#[test]
fn can_convert_f64() {
    with_guile(|_| {
        assert_eq!(SCM::from(0.2333_f64).try_into(), Ok(0.2333_f64));
        assert_eq!(SCM::from(f64::MIN).try_into(), Ok(f64::MIN));
        assert_eq!(SCM::from(f64::MAX).try_into(), Ok(f64::MAX));
    });
}

#[test]
fn can_convert_complex() {
    with_guile(|_| {
        let complex: Complex<f64> = Complex::new(1.0, 2.0);
        let scm: SCM = complex.into();
        let result: Result<Complex<f64>, String> = scm.try_into();
        assert_eq!(result, Ok(complex));
    });
}

#[test]
fn can_convert_string() {
    with_guile(|_| {
        let hello = String::from("Hello, Guile!");
        assert_eq!(SCM::from(hello.clone()).try_into(), Ok(hello));
    });
}

#[test]
fn can_condition_bool_true() {
    with_guile(|_| {
        let scm_bool = SCM::from(42);
        assert_eq!(scm_bool.is_true(), true);
    });
}

// #[test]
// fn can_convert_bool() {
//     with_guile(|_| {
//         assert_eq!(SCM::from(true).try_into(), Ok(true));
//         assert_eq!(SCM::from(false).try_into(), Ok(false));
//     });
// }

// #[test]
// fn can_convert_char() {
//     with_guile(|_| {
//         let a = char::from('a');
//         assert_eq!(SCM::from(a.clone()).try_into(), Ok(a));
//     });
// }

// #[test]
// fn can_convert_char_utf8() {
//     with_guile(|_| {
//         let a = char::from('中');
//         assert_eq!(SCM::from(a.clone()).try_into(), Ok(a));
//     });
// }

