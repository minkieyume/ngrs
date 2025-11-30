// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use crate::raw;
use std::ffi::c_void;
use crate::Runtime;

// 用于在C回调和Rust之间传递数据的结构
// English: A structure used to pass data between C callbacks and Rust
struct WithGuileData<F, R> {
    callback: Option<F>,
    result: Option<R>,
}

/// Use this function to run code within the Guile runtime context.
///
/// Useage:
/// ```
/// use ngrs::with_guile;
/// with_guile(|vm| {
///    vm.shell(vec!["test".to_string()]);
/// });
/// ```
pub fn with_guile<F, R>(f: F) -> R
where
    F: FnOnce(&Runtime) -> R,
{        
    let mut data = WithGuileData {
        callback: Some(f),
        result: None,
    };

    unsafe {
        raw::scm_with_guile(
            Some(with_guile_callback::<F, R>),
            &mut data as *mut _ as *mut c_void,
        );
    }

    // 取出结果
    // English: Extract the result
    data.result.expect("callback should have set result")
}

unsafe extern "C" fn with_guile_callback<F, R>(data: *mut c_void) -> *mut c_void
where
    F: FnOnce(&Runtime) -> R,
{
    // 转回我们的数据结构
    // English: Convert back to our data structure
    let data = unsafe{ &mut *(data as *mut WithGuileData<F, R>) };
    
    // 取出闭包
    // English: Extract the closure
    let callback = data.callback.take().expect("callback already called");
    
    // 取出Runtime引用（复用外层的Runtime）
    // English: Extract the Runtime reference (reuse the outer Runtime)
    let runtime = Runtime {};
    
    // 执行用户闭包
    // English: Execute the user closure
    let result = callback(&runtime);
    
    // 保存结果
    // English: Save the result
    data.result = Some(result);
    
    // C期望返回void*，我们返回NULL
    // English: C expects a void* return, we return NULL
    std::ptr::null_mut()
}

// 用于在C回调和Rust之间传递数据的结构
// English: A structure used to pass data between C callbacks and Rust
struct WithoutGuileData<F, R> {
    callback: Option<F>,
    result: Option<R>,
}

pub fn without_guile<F, R>(f: F) -> R
where
    F: FnOnce() -> R,  // 注意：不需要 &Runtime 参数
{        
    let mut data = WithoutGuileData {
        callback: Some(f),
        result: None,
    };
    unsafe {
        raw::scm_without_guile(
            Some(without_guile_callback::<F, R>),
            &mut data as *mut _ as *mut c_void,
        );
    }
    // 取出结果
    // English: Extract the result
    data.result.expect("callback should have set result")
}

unsafe extern "C" fn without_guile_callback<F, R>(data: *mut c_void) -> *mut c_void
where
    F: FnOnce() -> R,
{
    // 转回我们的数据结构
    // English: Convert back to our data structure
    let data = unsafe { &mut *(data as *mut WithoutGuileData<F, R>) };
    
    // 取出闭包
    // English: Extract the closure
    let callback = data.callback.take().expect("callback already called");
    
    // 执行用户闭包（在 guile 模式之外）
    // English: Execute the user closure (outside of guile mode)
    let result = callback();
    
    // 保存结果
    // English: Save the result
    data.result = Some(result);
    
    // C期望返回void*，我们返回NULL
    // English: C expects a void* return, we return NULL
    std::ptr::null_mut()
}
