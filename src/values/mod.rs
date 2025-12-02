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

    pub fn lookup_var(cha:&str) -> Self {
        let c_str = std::ffi::CString::new(cha).expect("Failed to create CString");
        let scm = unsafe { raw::scm_c_lookup(c_str.as_ptr()) };
        SCM::new(scm)
    }

    pub fn var_to_val(&self) -> Self {
        assert!(self.is_variable(), "SCM value is not variable");
        let scm_val = unsafe { raw::scm_variable_ref(self.0) };
        SCM::new(scm_val)
    }

    pub fn from_var_name(cha:&str) -> Self {
        let scm = SCM::lookup_var(cha);
        scm.var_to_val()
    }

    // Type predicates
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

    pub fn is_rational(&self) -> bool {
        unsafe { raw::scm_is_rational(self.0) != 0 }
    }

    pub fn is_exact(&self) -> bool {
        unsafe { raw::scm_is_exact(self.0) != 0 }
    }

    pub fn is_exact_integer(&self) -> bool {
        unsafe { raw::scm_is_exact_integer(self.0) != 0 }
    }

    pub fn is_inexact(&self) -> bool {
        unsafe { raw::scm_is_inexact(self.0) != 0 }
    }

    pub fn inexact_to_exact(&self) -> SCM {
        assert!(self.is_inexact(), "SCM value is not inexact number");
        let scm = unsafe { raw::scm_inexact_to_exact(self.0) };
        SCM::new(scm)
    }

    pub fn exact_to_inexact(&self) -> SCM {
        assert!(self.is_exact(), "SCM value is not exact number");
        let scm = unsafe { raw::scm_exact_to_inexact(self.0) };
        SCM::new(scm)
    }
    
    pub fn is_complex(&self) -> bool {
        unsafe { raw::scm_is_complex(self.0) != 0 }
    }

    pub fn is_string(&self) -> bool {
        unsafe { raw::scm_is_string(self.0) != 0 }
    }

    pub fn is_char(&self) -> bool {
        let char_p = SCM::new(unsafe { raw::scm_char_p(self.0) });
        char_p.is_true()
    }

    pub fn is_symbol(&self) -> bool {
        unsafe { raw::ngrs_is_symbol(self.0) != 0 }
    }

    pub fn is_keyword(&self) -> bool {
        unsafe { raw::scm_is_keyword(self.0) != 0 }
    }

    pub fn is_pair(&self) -> bool {
        unsafe { raw::scm_is_pair(self.0) != 0 }
    }

    pub fn is_null(&self) -> bool {
        unsafe { raw::ngrs_is_null(self.0) != 0 }
    }

    pub fn is_vector(&self) -> bool {
        unsafe { raw::scm_is_vector(self.0) != 0 }
    }

    pub fn is_array(&self) -> bool {
        unsafe { raw::scm_is_array(self.0) != 0 }
    }

    pub fn is_bytevector(&self) -> bool {
        unsafe { raw::scm_is_bytevector(self.0) != 0 }
    }

    pub fn is_procedure(&self) -> bool {
        let procedure_p = SCM::new(unsafe { raw::scm_procedure_p(self.0) });
        procedure_p.is_true()
    }

    pub fn is_thunk(&self) -> bool {
        let thunk_p = SCM::new(unsafe { raw::scm_thunk_p(self.0) });
        thunk_p.is_true()
    }

    pub fn is_variable(&self) -> bool {
        let variable_p = SCM::new(unsafe { raw::scm_variable_p(self.0) });
        variable_p.is_true()
    }
}

impl Drop for SCM {
    fn drop(&mut self) {
        unsafe {
            raw::scm_gc_unprotect_object(self.0);
        }
    }
}

impl PartialEq for SCM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { raw::ngrs_is_eq(self.0, other.0) != 0 }
    }
}
