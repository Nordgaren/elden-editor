use crate::dl_string::DLWString;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::addr_of;
use std::string::FromUtf16Error;

const PARAM_ENTRIES: usize = 186;

#[repr(C)]
pub(super) struct SoloParamRepository {
    pub unknown0x0: [u8; 0x80],
    pub repository_entries: [RepositoryEntry; PARAM_ENTRIES],
}

#[derive(Clone, Copy)]
pub(super) struct SoloParamRepositoryPtr {
    pub address: *mut SoloParamRepository,
}

#[repr(C)]
pub(super) struct RepositoryEntry {
    pub param_loaded: bool,
    pub undefined0x1: [u8; 0x3],
    pub pad0x4: u32,
    pub param: ParamResCapPtr,
    pub undefined0x10: [u8; 0x38],
}

#[repr(C)]
pub struct TableEntry {
    pub param_id: i32,
    pub pad0x4: u32,
    pub param_offset: u32,
    pub pad0xc: u32,
    pub string_offset: u32,
    pub pad0x14: u32,
}

#[repr(C)]
pub struct ParamHeader {
    pub string_offset: u32,
    pub undefined0x4: [u8; 0x6],
    pub row_count: u16,
    pub undefined0xc: [u8; 0x4],
    pub param_type_offset: u64,
    pub undefined0x18: [u8; 0x18],
    pub data_offset: u32,
    pub undefined0x34: [u8; 0xC],
    pub param_table: TableEntry,
}
#[derive(Clone, Copy)]
pub(super) struct ParamHeaderPtr {
    pub address: *mut ParamHeader,
}


#[repr(C)]
pub(super) struct ParamInfo {
    pub undefined0x0: [u8; 0x18],
    pub param_name: DLWString,
    pub undefined0x38: [u8; 0x48],
    pub param: ParamHeaderPtr,
}
#[derive(Clone, Copy)]
pub(super) struct ParamInfoPtr {
    pub address: *mut ParamInfo,
}


#[repr(C)]
pub(super) struct ParamResCap {
    pub undefined0x0: [u8; 0x18],
    pub param_name: DLWString,
    pub undefined0x38: [u8; 0x48],
    pub param_info: ParamInfoPtr,
}
#[derive(Clone, Copy)]
pub(super) struct ParamResCapPtr {
    pub address: *mut ParamResCap,
}


#[repr(C)]
pub(super) struct IdRepositoryInfo {
    pub start_offset: u32,
    pub entry_count: u32,
    pub pad0x8: u64,
}
#[derive(Clone, Copy)]
pub(super) struct IdRepositoryInfoPtr {
    pub address: *mut IdRepositoryInfo,
}

#[repr(C)]
pub struct IdRepositoryEntry {
    pub id: u32,
    pub index: u32,
}

#[cfg(test)]
mod tests {
    use crate::param_editor::structs::*;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<TableEntry>(), 0x18);
        assert_eq!(size_of::<ParamHeader>(), 0x58);
        assert_eq!(size_of::<ParamInfo>(), 0x88);
        assert_eq!(size_of::<ParamResCap>(), 0x88);
        assert_eq!(size_of::<RepositoryEntry>(), 0x48);
        assert_eq!(size_of::<IdRepositoryInfo>(), 0x10);
        assert_eq!(size_of::<IdRepositoryEntry>(), 0x8);
        assert_eq!(size_of::<SoloParamRepository>(), 0x34D0);
    }
}
