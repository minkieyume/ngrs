// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
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
}

impl Drop for SCM {
    fn drop(&mut self) {
        unsafe {
            raw::scm_gc_unprotect_object(self.0);
        }
    }
}
