mod dl_string;
pub mod structs;

use crate::param::traits::*;
use std::ffi::{c_char, CStr};
use std::marker::PhantomData;
use std::mem;
use std::mem::size_of;
use std::ptr::addr_of;
use structs::*;
use widestring::U16CStr;

const PARAM_ENTRIES: usize = 186;
const REPOSITORY_ARRAY: usize = 0x80;

pub struct ParamEditor<P: Param> {
    solo_param_repository_address: usize,
    param_res_cap: &'static ParamResCap,
    param_pointer: usize,
    param_header: &'static ParamHeader,
    id_repository_info: &'static IdRepositoryInfo,
    phantom_data: PhantomData<P>,
}

impl<P: Param> ParamEditor<P> {
    pub unsafe fn new() -> ParamEditor<P> {
        ParamEditor {
            solo_param_repository_address: 0,
            param_res_cap: mem::transmute(0usize),
            param_pointer: 0,
            param_header: mem::transmute(0usize),
            id_repository_info: mem::transmute(0usize),
            phantom_data: Default::default(),
        }
    }
    pub unsafe fn from_addr(solo_param_repository_address: usize) -> ParamEditor<P> {
        let mut editor = ParamEditor::new();
        editor.init(solo_param_repository_address);
        editor
    }
    pub unsafe fn from_addr_ref(solo_param_repository_address: &usize) -> ParamEditor<P> {
        let mut editor = ParamEditor::new();
        editor.init(*solo_param_repository_address);
        editor
    }
    pub unsafe fn init(&mut self, solo_param_repository_address: usize) {
        self.solo_param_repository_address = solo_param_repository_address;
        self.param_res_cap = self
            .find_param_res_cap()
            .expect(&format!("Could not find ParamResCap for {}", P::name()));

        self.param_header = self.param_res_cap.param_info.param;
        self.param_pointer = addr_of!(*self.param_header) as usize;

        let param_type = CStr::from_ptr(
            (self.param_pointer + self.param_header.param_type_offset as usize) as *const c_char,
        );
        if param_type.to_bytes() != P::param_type_name().as_bytes() {
            panic!(
                "Param {} def strings name did not match. game: {:?} header: {}",
                P::name(),
                param_type,
                P::param_type_name()
            );
        }

        let param_table = self.get_param_table();
        let entry_size = param_table[1].param_offset - param_table[0].param_offset;
        if entry_size as usize != size_of::<P>() {
            panic!(
                "Param {} new entries size and entry size in game did not match. In Game:{:X} - {:X}\n",
                P::name(),
                entry_size,
                size_of::<P>()
            );
        }

        self.id_repository_info = mem::transmute(self.param_pointer - 0x10);
    }
    #[inline(always)]
    pub unsafe fn get_param_table(&self) -> &'static [ParamTable] {
        std::slice::from_raw_parts(
            &self.param_header.table,
            self.param_header.row_count as usize,
        )
    }
    #[inline(always)]
    pub unsafe fn get_param_slice(&self) -> &'static [P] {
        self.get_param_slice_mut()
    }
    pub unsafe fn get_param_slice_mut(&self) -> &'static mut [P] {
        let pointer = self.param_pointer + self.param_header.data_offset as usize;
        std::slice::from_raw_parts_mut(pointer as *mut P, self.param_header.row_count as usize)
    }
    #[inline(always)]
    pub unsafe fn get_id_repository(&self) -> &'static [IdRepositoryEntry] {
        self.get_id_repository_mut()
    }
    pub unsafe fn get_id_repository_mut(&self) -> &'static mut [IdRepositoryEntry] {
        let pointer =
            self.param_pointer + ((self.id_repository_info.start_offset as usize + 15) & !0xF);
        std::slice::from_raw_parts_mut(
            pointer as *mut IdRepositoryEntry,
            self.id_repository_info.entry_count as usize,
        )
    }
    pub unsafe fn get_param_entry(&self, entry_id: i32) -> Option<&'static P> {
        let param_table = self.get_param_table();
        for entry in param_table {
            if entry.param_id == entry_id {
                return Some(mem::transmute(
                    self.param_pointer + entry.param_offset as usize,
                ));
            }
        }
        None
    }
    pub unsafe fn get_param_entry_mut(&self, entry_id: i32) -> Option<&'static mut P> {
        let param_table = self.get_param_table();
        for entry in param_table {
            if entry.param_id == entry_id {
                return Some(mem::transmute(
                    self.param_pointer + entry.param_offset as usize,
                ));
            }
        }
        None
    }
    unsafe fn find_param_res_cap(&self) -> Option<&'static ParamResCap> {
        let solo_param_entries = std::slice::from_raw_parts(
            (self.solo_param_repository_address + REPOSITORY_ARRAY) as *const RepositoryEntry,
            PARAM_ENTRIES,
        );

        for entry in solo_param_entries {
            if entry.param_loaded {
                if entry.param.param_name == P::name() {
                    return Some(entry.param);
                }
            }
        }
        None
    }
}
