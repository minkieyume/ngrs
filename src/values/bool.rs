// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use crate::raw;
use crate::SCM;
use std::ffi::c_int;

impl From<bool> for SCM {
    fn from(value: bool) -> SCM {
        unsafe {
            let scm = if value {
                raw::ngrs_from_bool(1 as c_int)
            } else {
                raw::ngrs_from_bool(0 as c_int)
            };
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for bool {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_bool(scm.0) } != 0 {
            Ok(unsafe { raw::scm_to_bool(scm.0) } != 0)
        } else {
            Err("Wrong Type: SCM is not bool.".to_string())
        }
    }
}
