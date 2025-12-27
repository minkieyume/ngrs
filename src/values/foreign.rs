// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use crate::SCM;
use std::ffi::c_void;
// use std::ptr;

use crate::raw;

pub trait ForeignType {
    fn type_name(&self) -> String;
    fn slots(&self) -> Vec<String>;
    fn slot_vals(&self) -> Vec<*mut c_void> {
        // 默认返回空的指针向量
        vec![]
    }
    fn need_finalizer(&self) -> bool { false }
    fn finalizer(&self) -> unsafe extern "C" fn(scm:*mut raw::scm_unused_struct) {
        unsafe extern "C" fn default_finalizer(_scm: *mut raw::scm_unused_struct) {
            // 默认不做任何操作
        }
        default_finalizer
    }

    fn type_of(&self) -> SCM {
        self.new_type()
    }

    /// Convert a ForeignType to a SCM foreign object type
    /// This will create a new foreign object type in Guile with the specified
    /// name and slots.
    /// If the ForeignType indicates that it needs a finalizer, the finalizer
    /// function will be used when creating the foreign object type.
    /// Warning: This function must to process in with_guile context.
    fn new_type(&self) -> SCM {
        let scm_name = SCM::from(self.type_name()).string_to_symbol();
        let symbols:Vec<SCM> = self.slots().into_iter()
            .map(SCM::from)
            .map(|s| s.string_to_symbol())
            .collect();
        let scm_slots = SCM::list(symbols.as_slice());
        let finalizer = if self.need_finalizer() {
            Some(self.finalizer())
        } else {
            None
        };

        let scm_type = unsafe {
            raw::scm_make_foreign_object_type(scm_name.0, scm_slots.0, finalizer)
        };

        SCM::new(scm_type)
    }
}


impl<T:ForeignType> From<T> for SCM {
    fn from(foreign: T) -> Self {
        let scm_type = foreign.type_of();
        let slot_vals = foreign.slot_vals();
        let slot_num = foreign.slots().len();
        let scm_obj = unsafe {
            raw::scm_make_foreign_object_n(scm_type.0, slot_num, slot_vals.as_ptr() as *mut *mut c_void)
        };
        SCM::new(scm_obj)
    }
}
