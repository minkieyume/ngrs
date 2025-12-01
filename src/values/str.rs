use crate::raw;
use crate::SCM;
use std::ffi::{CStr,CString,c_void};

impl From<String> for SCM {
    fn from(value: String) -> SCM {
        let c_str = CString::new(value).expect("Invalid String");
        unsafe {
            let scm = raw::scm_from_utf8_string(c_str.as_ptr());
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for String {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_string(scm.0) } != 0 {
            let c_ptr = unsafe { raw::scm_to_utf8_string(scm.0) };
            if c_ptr.is_null() {
                return Err("Failed to convert SCM to string".to_string());
            } else {
                let rust_string = unsafe { CStr::from_ptr(c_ptr) }
                    .to_str()
                    .map_err(|e| e.to_string())?
                    .to_owned();
                
                unsafe {
                    libc::free(c_ptr as *mut c_void);
                }
                
                Ok(rust_string)
            }
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

impl From<char> for SCM {
    fn from(value: char) -> SCM {
        let scm = unsafe { raw::scm_from_wchar(value as i32) };
        SCM::new(scm)
    }
}

// impl TryFrom<SCM> for char {
//     type Error = String;
    
//     fn try_from(scm: SCM) -> Result<Self, Self::Error> {
//         if unsafe { raw::scm_is_integer(scm.0) } != 0 {
//             let c32:u32 = scm.try_into().unwrap();
//             let character = char::from_u32(c32).ok_or("Invalid char value".to_string())?;
//             Ok(character)
//         } else {
//             Err("Expected Valid Char".to_string())
//         }
//     }
// }
