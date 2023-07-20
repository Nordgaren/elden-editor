use std::mem::size_of;
use std::ptr::addr_of;
use crate::fmg_editor::structs::{FmgId, MsgRepositoryCategory, MsgRepositoryCategoryPtr, MsgRepositoryGroup};
use widestring::U16CStr;

#[derive(Copy, Clone)]
pub struct Fmg {
    pub(super) fmg_id: FmgId,
    pub(super) category: MsgRepositoryCategoryPtr,
}


pub(super) struct FmgEntry {
    pub(super) id: i32,
    pub(super) string: Vec<u16>,
}

impl Fmg {
    pub(crate) fn new(fmg_id: FmgId) -> Fmg {
        Fmg { fmg_id, category: Default::default() }
    }
    #[inline(always)]
    pub unsafe fn get_group_slice(&self) -> &'static [MsgRepositoryGroup] {
        self.get_group_slice_mut()
    }
    pub unsafe fn get_group_slice_mut(&self) -> &'static mut [MsgRepositoryGroup] {
        std::slice::from_raw_parts_mut(
            (addr_of!(*self.category) as usize + size_of::<MsgRepositoryCategory>())
                as *mut MsgRepositoryGroup,
            self.category.group_count as usize,
        )
    }
    #[inline(always)]
    pub unsafe fn get_offset_slice(&self) -> &'static [usize] {
        self.get_offset_slice_mut()
    }
    pub unsafe fn get_offset_slice_mut(&self) -> &'static mut [usize] {
        std::slice::from_raw_parts_mut(
            self.category.string_offsets,
            self.category.string_count as usize,
        )
    }
    #[inline(always)]
    pub unsafe fn get_entry(&self, entry: i32) -> &'static U16CStr {
        self.get_entry_mut(entry)
    }
    pub unsafe fn get_entry_mut(&self, entry: i32) -> &'static mut U16CStr {
        let groups = self.get_group_slice();
        for group in groups {
            if entry <= group.last_id && entry >= group.first_id {
                return self.get_entry_from_group(entry, group);
            }
        }

        panic!("Attempted to find entry {}", entry)
    }
    pub unsafe fn get_entry_from_group(&self, entry: i32, group: &MsgRepositoryGroup) -> &'static mut U16CStr {
        let i = entry - group.first_id;
        let offset_slice = self.get_offset_slice();
        let offset = offset_slice[group.index as usize + i as usize];
        return U16CStr::from_ptr_str_mut(
            (addr_of!(*self.category) as usize + offset) as *mut u16,
        );
    }
    pub(crate) unsafe fn get_offset(&mut self, entry: i32) -> usize {
        let groups = self.get_group_slice();
        for group in groups {
            if entry <= group.last_id && entry >= group.first_id {
                let i = entry - group.first_id;
                return self.get_offset_slice_mut()[group.index as usize + i as usize];
            }
        }

        0
    }
    pub(crate) unsafe fn set_offset(&self, entry: i32, offset: usize) -> bool {
        let groups = self.get_group_slice();

        for group in groups {
            if entry <= group.last_id && entry >= group.first_id {
                let i = entry - group.first_id;
                self.get_offset_slice_mut()[group.index as usize + i as usize] = offset;
                return true;
            }
        }

        false
    }
}
