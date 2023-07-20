#![allow(unused)]

use std::borrow::Borrow;
use std::cell::UnsafeCell;
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
use crate::fmg_editor::fmg::{Fmg, FmgEntry};

pub mod structs;
pub mod iterator;
pub mod fmg;

// static  msg_repository_imp: MsgRepositoryImpPtr = MsgRepositoryImpPtr { address: 0 as *mut MsgRepositoryImp };
// pub unsafe fn init() {
//     msg_repository_imp.address = 0 as *mut MsgRepositoryImp;
// }

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

impl FmgEditor {
    pub unsafe fn new(fmg_id: FmgId) -> Self {
        FmgEditor {
            msg_repository_imp:  Default::default(),
            fmg: Fmg::new(fmg_id),
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
        self.msg_repository_imp = MsgRepositoryImpPtr { address: msg_repository_imp_address as *mut MsgRepositoryImp };
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

