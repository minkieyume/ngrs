// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
pub mod number;
pub mod str;
pub mod bool;

use crate::raw;

#[derive(Debug, Clone)]
pub struct SCM (raw::SCM);

impl SCM {
    pub fn new(scm: raw::SCM) -> Self {
        unsafe {
            raw::scm_gc_protect_object(scm);
        }
        SCM(scm)
    }
    
    pub fn is_bool(&self) -> bool {
        unsafe { raw::scm_is_bool(self.0) != 0 }
    }
    
    pub fn is_true(&self) -> bool {
        unsafe { raw::ngrs_is_true(self.0) != 0 }
    }
    
    pub fn is_false(&self) -> bool {
        unsafe { raw::ngrs_is_false(self.0) != 0 }
    }
    
    pub fn is_number(&self) -> bool {
        unsafe { raw::scm_is_number(self.0) != 0 }
    }
    
    pub fn is_integer(&self) -> bool {
        unsafe { raw::scm_is_integer(self.0) != 0 }
    }
    
    pub fn is_real(&self) -> bool {
        unsafe { raw::scm_is_real(self.0) != 0 }
    }
    
    pub fn is_complex(&self) -> bool {
        unsafe { raw::scm_is_complex(self.0) != 0 }
    }
}

impl Drop for SCM {
    fn drop(&mut self) {
        unsafe {
            raw::scm_gc_unprotect_object(self.0);
        }
    }
}
