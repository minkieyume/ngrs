// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use raw_guile;

#[derive(Debug, Clone)]
pub struct SCM (raw_guile::SCM);

impl SCM {
    pub fn new(scm: raw_guile::SCM) -> Self {
        unsafe {
            raw_guile::scm_gc_protect_object(scm);
        }
        SCM(scm)
    }
}

impl Drop for SCM {
    fn drop(&mut self) {
        unsafe {
            raw_guile::scm_gc_unprotect_object(self.0);
        }
    }
}
