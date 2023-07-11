use std::ffi::{c_char, CStr};
use std::ops::Deref;
use widestring::{u16cstr, U16CStr};

#[repr(C)]
pub struct AllocatedDLWString {
    pub allocator: usize,
    pub string: DLWString,
}

#[repr(C)]
pub union DLWStringUnion {
    pub ptr: *const u16,
    pub buffer: [u16; 0x8],
}

#[repr(C)]
pub struct DLWString {
    pub string: DLWStringUnion,
    pub length: usize,
    pub capacity: usize,
}

impl DLWString {
    pub unsafe fn from_u16cstr(s: &U16CStr) -> DLWString {
        if s.len() >= 8 {
            return DLWString {
                string: DLWStringUnion { ptr: s.as_ptr() },
                length: s.len(),
                capacity: s.len(),
            };
        }
        let mut string = DLWStringUnion { buffer: [0; 0x8] };
        let buffer = s.as_slice();
        for i in 0..s.len() {
            string.buffer[i] = buffer[i]
        }
        DLWString {
            capacity: string.buffer.len(),
            string,
            length: s.len(),
        }
    }

    pub unsafe fn get_string_bytes(&self) -> &'static [u16] {
        if self.string.ptr.is_null() {
            return &[0; 1];
        }

        std::slice::from_raw_parts(self.get_string_ptr(), self.length)
    }
    pub unsafe fn get_string_ptr(&self) -> *const u16 {
        if self.length >= 8 {
            return self.string.ptr;
        }

        self.string.buffer.as_ptr()
    }
}

impl Deref for DLWString {
    type Target = U16CStr;

    fn deref(&self) -> &Self::Target {
        unsafe {
            U16CStr::from_ptr_str(self.get_string_ptr())
        }
    }
}

impl AsRef<U16CStr> for DLWString {
    fn as_ref(&self) -> &U16CStr {
        self.deref()
    }
}

impl PartialEq<&str> for DLWString {
    fn eq(&self, other: &&str) -> bool {
        if other.len() != self.length {
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
pub union DLStringUnion {
    pub ptr: *const c_char,
    pub buffer: [c_char; 0x10],
}

#[repr(C)]
pub struct DLString {
    pub string: DLStringUnion,
    pub length: usize,
    pub capacity: usize,
}

impl DLString {
    pub unsafe fn from_str(s: &str) -> DLString {
        if s.len() >= 8 {
            return DLString {
                string: DLStringUnion { ptr: s.as_ptr() as *const c_char },
                length: s.len(),
                capacity: s.len(),
            };
        }

        let mut string = DLStringUnion { buffer: [0; 0x10] };
        let buffer = s.as_bytes();
        for i in 0..s.len() {
            string.buffer[i] = buffer[i] as i8
        }
        DLString {
            capacity: string.buffer.len(),
            string,
            length: s.len(),
        }
    }

    pub unsafe fn get_string_bytes(&self) -> &'static [c_char] {
        if self.string.ptr.is_null() {
            return &[0; 1];
        }

        std::slice::from_raw_parts(self.get_string_ptr(), self.length)
    }
    pub unsafe fn get_string_ptr(&self) -> *const c_char {
        if self.length >= 0x10 {
            return self.string.ptr;
        }

        self.string.buffer.as_ptr()
    }
}

impl Deref for DLString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.get_string_ptr()) }
    }
}

impl AsRef<CStr> for DLString {
    fn as_ref(&self) -> &CStr {
        self.deref()
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
    use crate::dl_string::*;
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
