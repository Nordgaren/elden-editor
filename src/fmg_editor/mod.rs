#![allow(unused)]

use std::borrow::Borrow;
use crate::fmg_editor::structs::{FmgId, MsgRepositoryCategory, MsgRepositoryCategoryPtr, MsgRepositoryGroup, MsgRepositoryImp, MsgRepositoryImpPtr};
use crate::util;
use std::mem;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::ptr::addr_of;
use widestring::{U16CStr};
use windows::Win32::System::Memory::{
    VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
};

pub mod structs;

pub struct FmgEditor {
    msg_repository_imp: MsgRepositoryImpPtr,
    fmg: Fmg,
    changed_entries: Vec<FmgEntry>,
}

impl<T> AsRef<T> for FmgEditor
    where
        T: ?Sized,
        <FmgEditor as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl Deref for FmgEditor {
    type Target = Fmg;

    fn deref(&self) -> &Self::Target {
        &self.fmg
    }
}

impl DerefMut for FmgEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fmg
    }
}
#[derive(Copy, Clone)]
pub struct Fmg {
    fmg_id: FmgId,
    category: MsgRepositoryCategoryPtr,
}

struct FmgEntry {
    id: i32,
    string: Vec<u16>,
}

impl Fmg {
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

impl FmgEditor {
    pub unsafe fn new(fmg_id: FmgId) -> Self {
        FmgEditor {
            msg_repository_imp: mem::transmute(0usize),
            fmg: Fmg {
                fmg_id,
                category: Default::default(),
            },
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

impl IntoIterator for &FmgEditor {
    type Item = (i32, &'static mut U16CStr);
    type IntoIter = FmgIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.fmg.into_iter()
    }
}

impl IntoIterator for Fmg {
    type Item = (i32, &'static mut U16CStr);
    type IntoIter = FmgIterator;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let group_slice = self.get_group_slice();
            FmgIterator {
                fmg: self,
                group_slice,
                index: 0,
            }
        }
    }
}


pub struct FmgIterator {
    fmg: Fmg,
    group_slice: &'static [MsgRepositoryGroup],
    index: usize,
}

impl Iterator for FmgIterator {
    type Item = (i32, &'static mut U16CStr);

    fn next(&mut self) -> Option<Self::Item> {
        let mut group_slice = &self.group_slice[0];
        let mut entry = self.index as i32 + group_slice.first_id;
        if entry > group_slice.last_id {
            if self.group_slice.len() == 1 {
                return None;
            }
            self.group_slice = &self.group_slice[1..];
            self.index = 0;
            group_slice = &self.group_slice[0];
            entry = self.group_slice[0].first_id
        }
        self.index += 1;

        unsafe {
            Some((entry, self.fmg.get_entry_from_group(entry, group_slice)))
        }
    }
}
