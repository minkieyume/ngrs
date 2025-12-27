// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use ngrs::*;
use std::sync::Once;
use std::collections::HashMap;
use num_complex::Complex;
use std::ffi::c_void;

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

#[test]
fn can_convert_bool() {
    with_guile(|_| {
        assert_eq!(SCM::from(true).try_into(), Ok(true));
        assert_eq!(SCM::from(false).try_into(), Ok(false));
    });
}

#[test]
fn can_convert_char() {
    with_guile(|_| {
        let a = char::from('a');
        assert_eq!(SCM::from(a.clone()).try_into(), Ok(a));
    });
}

#[test]
fn can_convert_char_utf8() {
    with_guile(|_| {
        let a = char::from('中');
        assert_eq!(SCM::from(a.clone()).try_into(), Ok(a));
    });
}

#[test]
fn can_convert_object_to_string() {
    with_guile(|_| {
        let scm = SCM::from(42_i32);
        assert!(scm.to_string().contains("42"));
    });
}

#[test]
fn can_reconize_common_types() {
    with_guile(|_| {
        let scm_str = SCM::from(String::from("Hello"));
        assert!(scm_str.is_string());
        assert!(scm_str.is_true());
        assert!(scm_str.is_array());
        
        assert!(!scm_str.is_number());
        assert!(!scm_str.is_bool());
        assert!(!scm_str.is_char());
        assert!(!scm_str.is_integer());
        assert!(!scm_str.is_real());
        assert!(!scm_str.is_rational());
        assert!(!scm_str.is_complex());
        assert!(!scm_str.is_false());
        assert!(!scm_str.is_null());
        assert!(!scm_str.is_pair());
        assert!(!scm_str.is_symbol());
        assert!(!scm_str.is_keyword());
        assert!(!scm_str.is_vector());
        assert!(!scm_str.is_bytevector());
        assert!(!scm_str.is_procedure());
        assert!(!scm_str.is_thunk());
        assert!(!scm_str.is_variable());
    });
}

#[test]
fn can_reconize_extract_numbers() {
    with_guile(|_| {
        let scm_str = SCM::from(i32::MAX);
        assert!(scm_str.is_number());
        assert!(scm_str.is_integer());
        assert!(scm_str.is_exact());
        assert!(scm_str.is_exact_integer());
        assert!(!scm_str.is_inexact());
    });
}

#[test]
fn can_convert_pair() {
    with_guile(|_| {
        let scm_pair = SCM::from(Pair::new(SCM::from(1), SCM::from(2)));
        assert!(scm_pair.is_pair());
    });
}

#[test]
fn can_convert_scm_to_pair() {
    with_guile(|_| {
        let scm_pair = SCM::from(Pair::new(SCM::from(1), SCM::from(2)));
        let pair: Pair = scm_pair.try_into().unwrap();
        let first_scm:SCM = pair.car().unwrap_scm();
        let second_scm:SCM = pair.cdr().unwrap_scm();
        let first:i32 = first_scm.try_into().unwrap();
        let second:i32 = second_scm.try_into().unwrap();
        assert_eq!(first, 1);
        assert_eq!(second, 2);
    });
}

#[test]
fn can_convert_string_to_symbol() {
    with_guile(|_| {
        let scm_str = SCM::from(String::from("my-symbol"));
        let scm_sym = scm_str.string_to_symbol();
        assert!(scm_sym.is_symbol());
        assert_eq!(scm_sym.to_string(), "my-symbol");
    });
}

#[test]
fn can_convert_symbol_to_keyword() {
    with_guile(|_| {
        let scm_str = SCM::from(String::from("my-symbol"));
        let scm_sym = scm_str.string_to_symbol();
        let scm_key = scm_sym.symbol_to_keyword();
        assert!(scm_key.is_keyword());
        assert_eq!(scm_key.to_string(), "#:my-symbol");
    });
}

#[test]
fn can_make_procedure_0() {
    with_guile(|vm| {
        let gsubr = make_procedure! {
            fn scm_hello_test() -> SCM {
                println!("Hello from Rust procedure!");
                SCM::from(0)
            }
        };
        let result:i32 = vm.apply(&gsubr, &SCMOrPair::Other(SCM::eol())).try_into().unwrap();
        assert_eq!(result, 0);
    });
}

#[test]
fn can_make_procedure_1() {
    with_guile(|vm| {
        scm_fn! {
            fn scm_hello_test(scm1:SCM) -> SCM {
                println!("Hello from Rust procedure!");
                let val:i32 = scm1.try_into().unwrap();
                SCM::from(val - 1)
            }
        }
        let gsubr = make_gsubr("scm_hello_test",1,0,false,scm_hello_test as *const () as *mut c_void);
        let result:i32 = vm.apply(&gsubr,
            &SCMOrPair::Pair(Pair::new(SCM::from(1),SCM::eol())))
            .try_into().unwrap();
        assert_eq!(result, 0);
    });
}

#[test]
fn can_define_procedure_0() {
    with_guile(|vm| {
        define_procedure! {
            fn scm_hello_test() -> SCM {
                println!("Hello from Defined Rust procedure!");
                SCM::from(42)
            }
        };
        let proc = SCM::from_var_name("scm_hello_test");
        let result:i32 = vm.apply(&proc, &SCMOrPair::Other(SCM::eol())).try_into().unwrap();
        assert_eq!(result, 42);
    });
}

#[test]
fn can_make_vector() {
    with_guile(|_| {
        let vec:Vec<i32> = vec![1,2,3,4,5];
        let scm_vector = SCM::from(&vec[..]);
        assert!(scm_vector.is_vector());
    });
}

#[test]
fn can_make_scm_vector() {
    with_guile(|_| {
        let vec:Vec<i32> = vec![1,2,3,4,5];
        let scm_vec:Vec<SCM> = vec.iter().map(|&x| SCM::from(x)).collect();
        let scm_vector = SCM::from(&scm_vec[..]);
        assert!(scm_vector.is_vector());
    });
}

#[test]
fn can_convert_vector() {
    with_guile(|_| {
        let vec:Vec<i32> = vec![1,2,3,4,5];
        let scm_vector = SCM::from(&vec[..]);
        let result:Result<Vec<i32>,String> = scm_vector.try_into();
        assert_eq!(result, Ok(vec));
    });
}

#[test]
fn can_convert_vector_for_scm() {
    with_guile(|_| {
        let vec:Vec<i32> = vec![10,20,30,40,50];
        let vec_scm:Vec<SCM> = vec.iter().map(|&x| SCM::from(x)).collect();
        let scm_vector = SCM::from(&vec[..]);
        let result:Result<Vec<SCM>,String> = scm_vector.try_into();
        assert_eq!(result, Ok(vec_scm));
    });
}

#[test]
fn can_convert_list_to_vec() {
    with_guile(|_| {
        let pair = Pair::from(vec![1,2,3,4,5]);
        let scm_list = SCM::from(pair);
        let result:Result<Vec<i32>,String> = scm_list.try_into();
        assert_eq!(result, Ok(vec![1,2,3,4,5]));
    });
}

#[test]
fn can_make_hash_map() {
    with_guile(|_| {
        let mut map = HashMap::new();
        map.insert(String::from("one"), 1);
        map.insert(String::from("two"), 2);
        let scm_hash_map = SCM::from(map);
        assert!(scm_hash_map.is_hash_table());
    });
}

#[test]
fn can_make_alist() {
    with_guile(|_| {
        let mut map = HashMap::new();
        map.insert(String::from("one"), 1);
        map.insert(String::from("two"), 2);
        let scm_alist = SCM::alist(map);
        assert!(scm_alist.is_list());
    });
}

#[test]
fn can_convert_hash_map() {
    with_guile(|_| {
        let mut map = HashMap::new();
        map.insert(String::from("one"), 1);
        map.insert(String::from("two"), 2);
        let scm_hash_map = SCM::from(map.clone());
        let result:Result<HashMap<String,i32>,String> = scm_hash_map.try_into();
        assert_eq!(result, Ok(map));
    });
}

#[test]
fn can_make_foreign_type() {
    struct MyForeignType {
        _value: i32,
    }

    impl ForeignType for MyForeignType {
        fn type_name(&self) -> String {
            "my-foreign-type".to_string()
        }

        fn slots(&self) -> Vec<String> {
            vec!["value".to_string()]
        }
    }

    with_guile(|_| {
        let foreign = MyForeignType { _value: 42 };
        let scm_foreign = foreign.new_type();
        assert!(scm_foreign.is_struct());
    });
}

#[test]
fn can_make_foreign_object() {
    #[derive(Clone)]
    struct MyForeignType {
        _value: i32,
        scm_type: Option<SCM>
    }

    impl ForeignType for MyForeignType {
        fn type_name(&self) -> String {
            "my-foreign-type".to_string()
        }

        fn slots(&self) -> Vec<String> {
            vec!["value".to_string()]
        }

        fn slot_vals(&self) -> Vec<*mut c_void> {
            vec![&self._value as *const i32 as *mut c_void]
        }

        fn type_of(&self) -> SCM {
            if let Some(scm_type) = &self.scm_type {
                scm_type.clone()
            } else {
                self.new_type()
            }
        }
    }

    impl MyForeignType {
        fn new(value: i32) -> Self {
            let mut foreign = MyForeignType {
                _value: value,
                scm_type: None,
            };
            foreign.scm_type = Some(foreign.type_of());
            foreign
        }
    }

    with_guile(|_| {
        let foreign = MyForeignType::new(42);
        let scm_foreign:SCM = foreign.clone().into();
        assert!(scm_foreign.is_struct());
        assert_eq!(scm_foreign.struct_vtable(), foreign.type_of());
    });
}
