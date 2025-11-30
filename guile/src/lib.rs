use raw_guile;
use std::ffi::{CString, c_void, c_char};

pub struct Runtime {}

pub struct SCM (raw_guile::SCM);

// 用于在C回调和Rust之间传递数据的结构
// English: A structure used to pass data between C callbacks and Rust
struct WithGuileData<'a,F, R> {
    runtime:&'a Runtime,
    callback: Option<F>,
    result: Option<R>,
}

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            raw_guile::scm_init_guile();
        }
        Runtime {}
    }

    pub fn with_guile<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Runtime) -> R,
    {        
        let mut data = WithGuileData {
            runtime: self,
            callback: Some(f),
            result: None,
        };

        unsafe {
            raw_guile::scm_with_guile(
                Some(with_guile_callback::<F, R>),
                &mut data as *mut _ as *mut c_void,
            );
        }

        // 取出结果
        // English: Extract the result
        data.result.expect("callback should have set result")
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
            raw_guile::scm_shell(c_args.len() as i32, ptrs.as_mut_ptr());
        }
    }
    
}

unsafe extern "C" fn with_guile_callback<F, R>(data: *mut c_void) -> *mut c_void
where
    F: FnOnce(&Runtime) -> R,
{
    // 转回我们的数据结构
    // English: Convert back to our data structure
    let data = unsafe{ &mut *(data as *mut WithGuileData<'_,F, R>) };
    
    // 取出闭包
    // English: Extract the closure
    let callback = data.callback.take().expect("callback already called");
    
    // 取出Runtime引用（复用外层的Runtime）
    // English: Extract the Runtime reference (reuse the outer Runtime)
    let runtime = data.runtime;
    
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let runtime = Runtime::new();
        runtime.with_guile(|_| {
            println!("Hello guile from Rust!");
        });
    }
}
