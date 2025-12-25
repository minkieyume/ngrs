// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
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
        if scm.is_string() {
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
        let scm = unsafe { raw::scm_integer_to_char(SCM::from(value as u32).0) };
        SCM::new(scm)
    }
}

impl TryFrom<SCM> for char {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if scm.is_char() {
            let scm_int = SCM::new(unsafe { raw::scm_char_to_integer(scm.0) });
            let code_point = u32::try_from(scm_int)?;
            char::from_u32(code_point)
                .ok_or_else(|| "Invalid Unicode code point".to_string())
        } else {
            Err("Expected Valid Char".to_string())
        }
    }
}

impl ToString for SCM {
    fn to_string(&self) -> String {
        let printer = SCM::from_var_name("write");
        let scm_str = SCM::new(unsafe { raw::scm_object_to_string(self.0,printer.0) });
        scm_str.try_into().unwrap_or_default()
    }
}
