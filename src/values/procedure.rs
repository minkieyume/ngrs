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

pub fn define_gsubr(name: &str,req:i32,opt:i32,rest:bool,fcn:*mut c_void) {
    let arg_name = CString::new(name).expect("Failed to create CString");
    unsafe {
        raw::scm_c_define_gsubr(
            arg_name.as_ptr(),
            req,
            opt,
            if rest {1} else {0},
            fcn,
        );
    }
}

#[macro_export]
macro_rules! make_procedure {
    ( $name:expr, $req:expr, $opt:expr, $rest:expr, $fcn:expr ) => {
        {
            $crate::procedure::make_gsubr($name, $req, $opt, $rest, $fcn as *const () as *mut c_void)
        }
    };
    
    (fn $name:ident() -> SCM $body:block) => {
        {
            scm_fn! { fn $name() -> SCM $body }
            make_procedure!(stringify!($name), 0, 0, false, $name)
        }
    };
}

#[macro_export]
macro_rules! define_procedure {
    ( $name:expr, $req:expr, $opt:expr, $rest:expr, $fcn:expr ) => {
        {
            $crate::procedure::define_gsubr($name, $req, $opt, $rest, $fcn as *const () as *mut c_void)
        }
    };
    
    (fn $name:ident() -> SCM $body:block) => {
        {
            scm_fn! { fn $name() -> SCM $body }
            define_procedure!(stringify!($name), 0, 0, false, $name)
        }
    };
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
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM, $arg5:ident:SCM,$arg6:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM,
            __scm_internal_scm_raw_arg5: $crate::raw::SCM,
            __scm_internal_scm_raw_arg6: $crate::raw::SCM,) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                let $arg5 = $crate::SCM::new(__scm_internal_scm_raw_arg5);
                let $arg6 = $crate::SCM::new(__scm_internal_scm_raw_arg6);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM, $arg5:ident:SCM,$arg6:ident:SCM,$arg7:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM,
            __scm_internal_scm_raw_arg5: $crate::raw::SCM,
            __scm_internal_scm_raw_arg6: $crate::raw::SCM,
            __scm_internal_scm_raw_arg7: $crate::raw::SCM,) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                let $arg5 = $crate::SCM::new(__scm_internal_scm_raw_arg5);
                let $arg6 = $crate::SCM::new(__scm_internal_scm_raw_arg6);
                let $arg7 = $crate::SCM::new(__scm_internal_scm_raw_arg7);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM, $arg5:ident:SCM,$arg6:ident:SCM,$arg7:ident:SCM,$arg8:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM,
            __scm_internal_scm_raw_arg5: $crate::raw::SCM,
            __scm_internal_scm_raw_arg6: $crate::raw::SCM,
            __scm_internal_scm_raw_arg7: $crate::raw::SCM,
            __scm_internal_scm_raw_arg8: $crate::raw::SCM,) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                let $arg5 = $crate::SCM::new(__scm_internal_scm_raw_arg5);
                let $arg6 = $crate::SCM::new(__scm_internal_scm_raw_arg6);
                let $arg7 = $crate::SCM::new(__scm_internal_scm_raw_arg7);
                let $arg8 = $crate::SCM::new(__scm_internal_scm_raw_arg8);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM, $arg5:ident:SCM,$arg6:ident:SCM,$arg7:ident:SCM,$arg8:ident:SCM,$arg9:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM,
            __scm_internal_scm_raw_arg5: $crate::raw::SCM,
            __scm_internal_scm_raw_arg6: $crate::raw::SCM,
            __scm_internal_scm_raw_arg7: $crate::raw::SCM,
            __scm_internal_scm_raw_arg8: $crate::raw::SCM,
            __scm_internal_scm_raw_arg9: $crate::raw::SCM,) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                let $arg5 = $crate::SCM::new(__scm_internal_scm_raw_arg5);
                let $arg6 = $crate::SCM::new(__scm_internal_scm_raw_arg6);
                let $arg7 = $crate::SCM::new(__scm_internal_scm_raw_arg7);
                let $arg8 = $crate::SCM::new(__scm_internal_scm_raw_arg8);
                let $arg9 = $crate::SCM::new(__scm_internal_scm_raw_arg9);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
    (fn $name:ident($arg1:ident:SCM, $arg2:ident:SCM, $arg3:ident:SCM, $arg4:ident:SCM, $arg5:ident:SCM,$arg6:ident:SCM,$arg7:ident:SCM,$arg8:ident:SCM,$arg9:ident:SCM) -> SCM $body:block) => {
        pub extern "C" fn $name(__scm_internal_scm_raw_arg1: $crate::raw::SCM,
            __scm_internal_scm_raw_arg2: $crate::raw::SCM,
            __scm_internal_scm_raw_arg3: $crate::raw::SCM,
            __scm_internal_scm_raw_arg4: $crate::raw::SCM,
            __scm_internal_scm_raw_arg5: $crate::raw::SCM,
            __scm_internal_scm_raw_arg6: $crate::raw::SCM,
            __scm_internal_scm_raw_arg7: $crate::raw::SCM,
            __scm_internal_scm_raw_arg8: $crate::raw::SCM,
            __scm_internal_scm_raw_arg9: $crate::raw::SCM,
            __scm_internal_scm_raw_arg10: $crate::raw::SCM,) -> $crate::raw::SCM {
            
                let $arg1 = $crate::SCM::new(__scm_internal_scm_raw_arg1);
                let $arg2 = $crate::SCM::new(__scm_internal_scm_raw_arg2);
                let $arg3 = $crate::SCM::new(__scm_internal_scm_raw_arg3);
                let $arg4 = $crate::SCM::new(__scm_internal_scm_raw_arg4);
                let $arg5 = $crate::SCM::new(__scm_internal_scm_raw_arg5);
                let $arg6 = $crate::SCM::new(__scm_internal_scm_raw_arg6);
                let $arg7 = $crate::SCM::new(__scm_internal_scm_raw_arg7);
                let $arg8 = $crate::SCM::new(__scm_internal_scm_raw_arg8);
                let $arg9 = $crate::SCM::new(__scm_internal_scm_raw_arg9);
                let $arg10 = $crate::SCM::new(__scm_internal_scm_raw_arg10);
                        
                // 2. 执行用户代码
                let result: $crate::SCM = $body;
            
                // 3. 转换回 raw::SCM
                result.0
            }
    };
}
