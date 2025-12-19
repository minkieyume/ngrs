// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use crate::{raw, SCMOrPair};
use std::sync::OnceLock;
use std::ffi::{CString, c_char};
use crate::with_guile::with_guile;
use crate::values::SCM;

static GUILE_INIT: OnceLock<()> = OnceLock::new();

pub struct Runtime {}

impl Runtime {
    /// Initialize Guile runtime. Must run on main thread before creating other threads.
    /// It's must for Thread Safety.If you want to use Guile in multiple threads, you must call this function first on the main thread.
    /// 
    /// 必须在创建其它线程之前在主线程运行一次。
    /// 这是为了线程安全。如果你想在多个线程中使用Guile，必须先在主线程调用此函数。
    pub fn initialize() {
        GUILE_INIT.get_or_init(|| {
            with_guile(|_| {});
        });
    }

    /// Warning: This way to init runtime may not support in some platforms.
    /// Because it used fuction `scm_init_guile` for create Guile runtime.
    /// If you want to get best support for multi-platforms, please use `with_guile` instead.
    ///
    /// Usage:
    /// ```
    /// use ngrs::Runtime;
    /// let runtime = Runtime::new();
    /// runtime.shell(vec!["test".to_string()]);
    /// ```
    pub fn new() -> Self {
        unsafe {
            raw::scm_init_guile();
        };
        
        Runtime {}
    }

    pub fn shell(&self, args: Vec<String>) {
        let c_args: Vec<CString> = args
            .into_iter()
            .filter_map(|s| CString::new(s).ok())
            .collect();
    
        let mut ptrs: Vec<*mut c_char> = c_args
            .iter()
            .map(|cs| cs.as_ptr() as *mut c_char)
            .collect();
    
        unsafe {
            raw::scm_shell(c_args.len() as i32, ptrs.as_mut_ptr());
        }
    }

    pub fn eval_string(&self, code: &str) -> SCM {
        let c_code = CString::new(code).expect("Failed to create CString");
        let c_code_ptr = c_code.as_ptr();
        unsafe {
            let raw_scm = raw::scm_c_eval_string(c_code_ptr);
            SCM::new(raw_scm)
        }
    }

    pub fn primitive_load(&self, filename: &str) -> SCM {
        let c_filename = CString::new(filename).expect("Failed to create CString");
        let c_filename_ptr = c_filename.as_ptr();
        unsafe {
            let value = raw::scm_c_primitive_load(c_filename_ptr);
            SCM::new(value)
        }
    }

    pub fn primitive_load_path(&self, filename: &str) -> SCM {
        let c_filename = CString::new(filename).expect("Failed to create CString");
        let c_filename_ptr = c_filename.as_ptr();
        unsafe {
            let value = raw::scm_c_primitive_load_path(c_filename_ptr);
            SCM::new(value)
        }
    }

    pub fn define(&self, name: &str, value: &SCM) {
        let c_name = CString::new(name).expect("Failed to create CString");
        let c_name_ptr = c_name.as_ptr();
        unsafe {
            raw::scm_c_define(c_name_ptr, value.0);
        }
    }

    pub fn apply_scm(&self, proc: &SCM,args: &SCMOrPair) -> SCM {
        unsafe {
            let result = match args {
                SCMOrPair::Other(scm) => {
                    raw::scm_apply_0(proc.0, scm.0)
                },
                SCMOrPair::Pair(pair) => {
                    let arglst: SCM = pair.clone().into();
                    raw::scm_apply_0(proc.0, arglst.0)
                },
            };
            SCM::new(result)
        }
    }

    // pub fn call() -> SCM {
        
    // }

    
}
