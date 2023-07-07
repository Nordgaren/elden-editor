use std::ffi::{c_char, CStr};
use std::mem;
use std::ops::Deref;
use std::ptr::addr_of;
use std::str::Utf8Error;
use std::string::FromUtf16Error;
use widestring::{u16cstr, U16CStr, U16CString};

#[repr(C)]
pub(crate) struct DLWString {
    pub string: [u16; 0x8],
    pub length: usize,
    pub capacity: usize,
}

impl DLWString {
    pub unsafe fn from_u16cstr(s: &U16CStr) -> DLWString {
        let mut string = [0u16; 8];
        let capacity;

        if s.len() >= 8 {
            let addr = s.as_ptr() as usize;
            let mut str_ptr_bytes =
                unsafe { std::slice::from_raw_parts(addr_of!(addr) as *const u8, 8) };
            let string_bytes =
                { std::slice::from_raw_parts_mut(string.as_mut_ptr() as *mut u8, 8) };

            for i in 0..str_ptr_bytes.len() {
                string_bytes[i] = str_ptr_bytes[i]
            }

            capacity = s.len();
        } else {
            let buffer = s.as_slice();
            for i in 0..s.len() {
                string[i] = buffer[i]
            }

            capacity = string.len()
        }

        DLWString {
            string,
            length: s.len(),
            capacity,
        }
    }

    pub unsafe fn get_string_bytes(&self) -> &'static [u16] {
        if self.length >= 8 {
            let ptr = self.string.as_ptr() as *const *const u16;
            return std::slice::from_raw_parts(*ptr, self.length as usize);
        }

        std::slice::from_raw_parts(self.string.as_ptr(), self.length as usize)
    }
}

impl Deref for DLWString {
    type Target = U16CStr;

    fn deref(&self) -> &Self::Target {
        unsafe {
            U16CStr::from_ptr_unchecked(self.get_string_bytes().as_ptr(), self.length as usize)
        }
    }
}

impl AsRef<U16CStr> for DLWString {
    fn as_ref(&self) -> &U16CStr {
        unsafe { self.deref() }
    }
}

impl PartialEq<&str> for DLWString {
    fn eq(&self, other: &&str) -> bool {
        if other.len() != self.length as usize {
            return false;
        }

        self.as_slice()
            .iter()
            .zip(other.encode_utf16())
            .all(|(x, y)| x == &y)
    }
}

impl PartialEq<&U16CStr> for DLWString {
    fn eq(&self, other: &&U16CStr) -> bool {
        self.as_slice() == other.as_slice()
    }
}

#[repr(C)]
pub(crate) struct DLString {
    pub string: [u8; 0x10],
    pub length: usize,
    pub capacity: usize,
}

impl DLString {
    pub unsafe fn from_str(s: &str) -> DLString {
        let mut string = [0u8; 16];
        let capacity;

        if s.len() >= 8 {
            let addr = s.as_ptr() as usize;
            let mut str_ptr_bytes =
                unsafe { std::slice::from_raw_parts(addr_of!(addr) as *const u8, 8) };

            for i in 0..str_ptr_bytes.len() {
                string[i] = str_ptr_bytes[i]
            }

            capacity = s.len();
        } else {
            let buffer = s.as_bytes();
            for i in 0..s.len() {
                string[i] = buffer[i]
            }

            capacity = string.len()
        }

        DLString {
            string,
            length: s.len(),
            capacity: s.len(),
        }
    }

    pub unsafe fn get_string_bytes(&self) -> &'static [u8] {
        if self.length >= 8 {
            let ptr = self.string.as_ptr() as *const *const u8;
            return std::slice::from_raw_parts(*ptr, self.length as usize);
        }

        std::slice::from_raw_parts(self.string.as_ptr(), self.length as usize)
    }
}

impl Deref for DLString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.get_string_bytes().as_ptr() as *const c_char) }
    }
}

impl AsRef<CStr> for DLString {
    fn as_ref(&self) -> &CStr {
        unsafe { self.deref() }
    }
}

impl PartialEq<&U16CStr> for DLString {
    fn eq(&self, other: &&U16CStr) -> bool {
        if other.len() != self.length {
            return false;
        }

        self.to_bytes()
            .iter()
            .zip(other.as_slice().iter())
            .all(|(x, y)| *x as u16 == *y)
    }
}

impl PartialEq<&str> for DLString {
    fn eq(&self, other: &&str) -> bool {
        self.to_bytes() == other.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::param_editor::dl_string::*;
    use std::mem;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<DLWString>(), 0x20);
        assert_eq!(size_of::<DLString>(), 0x20);
    }

    #[test]
    fn w_string_check() {
        let w_string = unsafe { DLWString::from_u16cstr(u16cstr!("Bullet")) };
        assert!(w_string == "Bullet");
        assert!(w_string == u16cstr!("Bullet"));
        let w_string = unsafe { DLWString::from_u16cstr(u16cstr!("EquipParamWeapon")) };
        assert!(w_string == "EquipParamWeapon");
        assert!(w_string == u16cstr!("EquipParamWeapon"));
    }

    #[test]
    fn string_check() {
        let string = unsafe { DLString::from_str("Bullet") };
        assert!(string == "Bullet");
        assert!(string == u16cstr!("Bullet"));
        let string = unsafe { DLString::from_str("EquipParamWeapon") };
        assert!(string == "EquipParamWeapon");
        assert!(string == u16cstr!("EquipParamWeapon"));
    }
}
