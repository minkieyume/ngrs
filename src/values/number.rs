// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2024 MinkieYume <minkieyume@yumieko.com>
use crate::raw;
use crate::SCM;
use num_complex::Complex;

// ========== i8 ==========
impl From<i8> for SCM {
    fn from(value: i8) -> SCM {
        unsafe {
            let scm = raw::scm_from_int8(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for i8 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_signed_integer(scm.0,i8::MIN as i64,i8::MAX as i64) } != 0 {
            Ok(unsafe { raw::scm_to_int8(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== u8 ==========
impl From<u8> for SCM {
    fn from(value: u8) -> SCM {
        unsafe {
            let scm = raw::scm_from_uint8(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for u8 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_unsigned_integer(scm.0,u8::MIN as u64,u8::MAX as u64) } != 0 {
            Ok(unsafe { raw::scm_to_uint8(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== i16 ==========
impl From<i16> for SCM {
    fn from(value: i16) -> SCM {
        unsafe {
            let scm = raw::scm_from_int16(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for i16 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_signed_integer(scm.0,i16::MIN as i64,i16::MAX as i64) } != 0 {
            Ok(unsafe { raw::scm_to_int16(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== u16 ==========
impl From<u16> for SCM {
    fn from(value: u16) -> SCM {
        unsafe {
            let scm = raw::scm_from_uint16(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for u16 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_unsigned_integer(scm.0,u16::MIN as u64,u16::MAX as u64) } != 0 {
            Ok(unsafe { raw::scm_to_uint16(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== i32 ==========
impl From<i32> for SCM {
    fn from(value: i32) -> SCM {
        unsafe {
            let scm = raw::scm_from_int32(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for i32 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_signed_integer(scm.0,i32::MIN as i64,i32::MAX as i64) } != 0 {
            Ok(unsafe { raw::scm_to_int32(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== u32 ==========
impl From<u32> for SCM {
    fn from(value: u32) -> SCM {
        unsafe {
            let scm = raw::scm_from_uint32(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for u32 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_unsigned_integer(scm.0,u32::MIN as u64,u32::MAX as u64) } != 0 {
            Ok(unsafe { raw::scm_to_uint32(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== i64 ==========
impl From<i64> for SCM {
    fn from(value: i64) -> SCM {
        unsafe {
            let scm = raw::scm_from_int64(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for i64 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_signed_integer(scm.0,i64::MIN as i64,i64::MAX as i64) } != 0 {
            Ok(unsafe { raw::scm_to_int64(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== u64 ==========
impl From<u64> for SCM {
    fn from(value: u64) -> SCM {
        unsafe {
            let scm = raw::scm_from_uint64(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for u64 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_unsigned_integer(scm.0,u64::MIN as u64,u64::MAX as u64) } != 0 {
            Ok(unsafe { raw::scm_to_uint64(scm.0) })
        } else {
            Err("Expected Valid integer".to_string())
        }
    }
}

// ========== f64 ==========
impl From<f64> for SCM {
    fn from(value: f64) -> SCM {
        unsafe {
            let scm = raw::scm_from_double(value);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for f64 {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_real(scm.0) } != 0 {
            Ok(unsafe { raw::scm_to_double(scm.0) })
        } else {
            Err("Expected Valid Float".to_string())
        }
    }
}

// ========== complex ==========
impl From<Complex<f64>> for SCM {
    fn from(value: Complex<f64>) -> SCM {
        let real:SCM = value.re.into();
        let imag:SCM = value.im.into();
        unsafe {
            let scm = raw::scm_make_rectangular(real.0, imag.0);
            SCM::new(scm)
        }
    }
}

impl TryFrom<SCM> for Complex<f64> {
    type Error = String;
    
    fn try_from(scm: SCM) -> Result<Self, Self::Error> {
        if unsafe { raw::scm_is_complex(scm.0) } != 0 {
            let real = unsafe { raw::scm_c_real_part(scm.0) };
            let imag = unsafe { raw::scm_c_imag_part(scm.0) };
            Ok(Complex::new(real,imag))
        } else {
            Err("Expected Valid Complex Number".to_string())
        }
    }
}

