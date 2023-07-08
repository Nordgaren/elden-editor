#![allow(unused)]
use crate::fmg_editor::structs::{
    FmgId, MsgRepositoryCategory, MsgRepositoryGroup, MsgRepositoryImp,
};
use crate::util;
use std::mem;
use std::mem::size_of;
use std::ptr::addr_of;
use widestring::{U16CStr};
use windows::Win32::System::Memory::{
    VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
};

pub mod structs;

pub struct FmgEditor {
    msg_repository_imp: &'static MsgRepositoryImp,
    fmg_id: FmgId,
    category: &'static mut MsgRepositoryCategory,
    changed_entries: Vec<FmgEntry>,
}

struct FmgEntry {
    id: i32,
    string: Vec<u16>,
}

impl FmgEditor {
    pub unsafe fn new(fmg_id: FmgId) -> Self {
        FmgEditor {
            msg_repository_imp: mem::transmute(0usize),
            fmg_id,
            category: mem::transmute(0usize),
            changed_entries: vec![],
        }
    }
    pub unsafe fn from_addr(fmg_id: FmgId, msg_repository_imp_address: usize) -> Self {
        let mut editor = FmgEditor::new(fmg_id);
        editor.init(msg_repository_imp_address);
        editor
    }
    pub unsafe fn from_addr_ref(fmg_id: FmgId, msg_repository_imp_address: &usize) -> Self {
        let mut editor = FmgEditor::new(fmg_id);
        editor.init(*msg_repository_imp_address);
        editor
    }
    pub unsafe fn init(&mut self, msg_repository_imp_address: usize) {
        self.msg_repository_imp = mem::transmute(msg_repository_imp_address);
        self.category = self.msg_repository_imp.get_category_mut(0, self.fmg_id);
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
            self.category.string_offset,
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
                let i = entry - group.first_id;
                let offset = self.get_offset_slice()[group.index as usize + i as usize];
                return U16CStr::from_ptr_str_mut(
                    (addr_of!(*self.category) as usize + offset) as *mut u16,
                );
            }
        }

        panic!("Attempted to find entry {}", entry)
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
    pub fn set_entry(&mut self, id: i32, mut string: Vec<u16>) {
        if string.last().unwrap() != &0 {
            string.push(0);
        }
        self.changed_entries.push(FmgEntry { id, string })
    }
    pub unsafe fn patch_entries(&mut self) {
        for i in (0..self.changed_entries.len()).rev() {
            let entry = &self.changed_entries[i];
            let string = self.get_entry_mut(entry.id);
            if entry.string.len() <= string.as_slice_with_nul().len() {
                util::copy_slice(
                    &entry.string[..entry.string.len()],
                    std::slice::from_raw_parts_mut(
                        string.as_mut_ptr(),
                        string.as_slice_with_nul().len(),
                    ),
                );
                self.changed_entries.remove(i);
            }
        }

        let mut sum: usize = self.changed_entries.iter().map(|e| e.string.len()).sum();
        sum += 1;
        sum *= 2;

        let new_mem = VirtualAlloc(
            None,
            self.category.file_size as usize + sum,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        let new_slice = std::slice::from_raw_parts_mut(
            new_mem as *mut u8,
            self.category.file_size as usize + sum,
        );
        let old_slice = std::slice::from_raw_parts(
            addr_of!(*self.category) as usize as *mut u8,
            self.category.file_size as usize,
        );

        util::copy_slice(old_slice, new_slice);

        let mut end_offset = self.category.file_size as usize;

        for entry in &self.changed_entries {
            let end_slice = &mut new_slice[end_offset..];
            self.set_offset(entry.id, end_offset);

            let string = std::slice::from_raw_parts(
                entry.string.as_ptr() as *const u8,
                entry.string.len() * 2,
            );
            util::copy_slice(string, end_slice);

            end_offset += string.len();
        }

        self.category = mem::transmute(new_mem);

        self.msg_repository_imp.get_category_array_mut(0)[self.fmg_id as usize] =
            mem::transmute(addr_of!(*self.category) as usize);

        self.category.file_size += sum as u32;
        self.changed_entries.clear();
    }
}
