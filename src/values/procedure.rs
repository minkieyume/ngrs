use crate::SCM;
use std::ffi::{c_void, CString};

use crate::raw;

pub fn make_gsubr(name: &str,req:i32,opt:i32,rest:bool,fcn:*mut c_void) -> SCM {
    let arg_name = CString::new(name).expect("Failed to create CString");
    unsafe {
        let scm_proc = raw::scm_c_make_gsubr(
            arg_name.as_ptr(),
            req,
            opt,
            if rest {1} else {0},
            fcn,
        );
        SCM::new(scm_proc)
    }
}

#[macro_export]
macro_rules! scm_fn {
    // 匹配：函数名, 参数名, 函数体
    (fn $name:ident() -> SCM $body:block) => {
        pub extern "C" fn $name() -> $crate::raw::SCM {
            
            let result: $crate::SCM = $body;
            
            result.0
        }
    };
    (fn $name:ident($arg1:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM) -> $crate::raw::SCM {
            
            let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                        
            // 2. 执行用户代码
            let result: $crate::SCM = $body;
            
            // 3. 转换回 raw::SCM
            result.0
        }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM, $arg5:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM,
            __scm_internal_scm_raw_arg5: $crate::raw::SCM) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                let $arg5 = $crate::SCM::new(__scm_internal_scm_raw_arg5);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
}
