use crate::fmg_editor::fmg::FmgId;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub(super) struct MsgRepositoryImp {
    vtable: usize,
    categories: *mut *mut *mut MsgRepositoryCategory,
    version_count: u32,
    category_capacity: u32,
    unk0x18: usize,
    unk0x20: usize,
    unk0x28: usize,
    unk0x30: usize,
    unk0x38: u32,
    unk0x3c: u32,
    unk0x40: u32,
    unk0x44: u32,
}

pub(super) struct MsgRepositoryImpPtr {
    pub(super) address: *mut MsgRepositoryImp,
}

impl MsgRepositoryImp {
    pub(super) unsafe fn get_version_array(&self) -> &'static [*mut *const MsgRepositoryCategory] {
        std::slice::from_raw_parts(self.categories as _, self.version_count as usize)
    }
    pub(super) unsafe fn get_version_array_mut(
        &self,
    ) -> &'static mut [*mut *mut MsgRepositoryCategory] {
        std::slice::from_raw_parts_mut(self.categories, self.version_count as usize)
    }
    pub(super) unsafe fn get_category_array(
        &self,
        version: usize,
    ) -> &'static [*const MsgRepositoryCategory] {
        std::slice::from_raw_parts(
            self.get_version_array()[version],
            self.category_capacity as usize,
        )
    }
    pub(super) unsafe fn get_category_array_mut(
        &self,
        version: usize,
    ) -> &'static mut [*mut MsgRepositoryCategory] {
        std::slice::from_raw_parts_mut(
            self.get_version_array_mut()[version],
            self.category_capacity as usize,
        )
    }
    pub unsafe fn get_category(&self, version: usize, fmg: FmgId) -> MsgRepositoryCategoryPtr {
        self.get_category_array(version)[fmg as usize].into()
    }
    pub unsafe fn get_category_mut(&self, version: usize, fmg: FmgId) -> MsgRepositoryCategoryPtr {
        self.get_category_array_mut(version)[fmg as usize].into()
    }
}

#[repr(C)]
pub struct MsgRepositoryCategory {
    pub undefined0x0: u8,
    pub big_endian: bool,
    pub version: u8,
    pub undefined0x3: u8,
    pub file_size: u32,
    pub undefined0x8: u8,
    pub undefined0x9: u8,
    pub undefined0xa: u8,
    pub undefined0xb: u8,
    pub group_count: u32,
    pub string_count: u32,
    pub is_0xff: u32,
    pub string_offsets: *mut usize,
    pub undefined0x20: u8,
    pub undefined0x21: u8,
    pub undefined0x22: u8,
    pub undefined0x23: u8,
    pub undefined0x24: u8,
    pub undefined0x25: u8,
    pub undefined0x26: u8,
    pub undefined0x27: u8,
}
#[derive(Copy, Clone)]
pub(super) struct MsgRepositoryCategoryPtr {
    pub(super) address: *mut MsgRepositoryCategory,
}

#[repr(C)]
pub struct MsgRepositoryGroup {
    pub index: i32,
    pub first_id: i32,
    pub last_id: i32,
    pub pad0xc: i32,
}

#[cfg(test)]
mod tests {
    use crate::fmg_editor::structs::*;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MsgRepositoryImp>(), 0x48);
        assert_eq!(size_of::<MsgRepositoryCategory>(), 0x28);
        assert_eq!(size_of::<MsgRepositoryGroup>(), 0x10);
    }
}
