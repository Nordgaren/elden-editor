#[allow(unused)]
pub mod structs;

use crate::param::traits::*;
use std::ffi::{c_char, CStr};
use std::marker::PhantomData;
use std::mem;
use std::mem::size_of;
use std::ops::DerefMut;
use std::ptr::addr_of;
use structs::*;
use crate::param::traits;

pub struct ParamEditor<P: Param> {
    solo_param_repository: &'static SoloParamRepository,
    param_res_cap: &'static ParamResCap,
    param_header: &'static ParamHeader,
    id_repository_info: &'static IdRepositoryInfo,
    phantom_data: PhantomData<P>,
}

impl<P: Param> ParamEditor<P> {
    pub unsafe fn new() -> ParamEditor<P> {
        ParamEditor {
            solo_param_repository: mem::transmute(0usize),
            param_res_cap: mem::transmute(0usize),
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
        self.solo_param_repository = mem::transmute(solo_param_repository_address);
        self.param_res_cap = self
            .find_param_res_cap()
            .expect(&format!("Could not find ParamResCap for {}", P::name()));

        self.param_header = self.param_res_cap.param_info.param;

        let param_type = CStr::from_ptr(
            (addr_of!(*self.param_header) as usize + self.param_header.param_type_offset as usize)
                as *const c_char,
        );
        if param_type.to_bytes() != P::Def::param_type_name().as_bytes() {
            panic!(
                "Param {} def strings name did not match. game: {:?} header: {}",
                P::name(),
                param_type,
                P::Def::param_type_name()
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

        self.id_repository_info = mem::transmute(addr_of!(*self.param_header) as usize - 0x10);
    }
    #[inline(always)]
    pub unsafe fn get_param_table(&self) -> &'static [TableEntry] {
        std::slice::from_raw_parts(
            &self.param_header.param_table,
            self.param_header.row_count as usize,
        )
    }
    #[inline(always)]
    pub unsafe fn get_param_slice(&self) -> &'static [P::Def] {
        self.get_param_slice_mut()
    }
    pub unsafe fn get_param_slice_mut(&self) -> &'static mut [P::Def] {
        let pointer =
            addr_of!(*self.param_header) as usize + self.param_header.data_offset as usize;
        std::slice::from_raw_parts_mut(pointer as *mut P::Def, self.param_header.row_count as usize)
    }
    #[inline(always)]
    pub unsafe fn get_id_repository(&self) -> &'static [IdRepositoryEntry] {
        self.get_id_repository_mut()
    }
    pub unsafe fn get_id_repository_mut(&self) -> &'static mut [IdRepositoryEntry] {
        let pointer = addr_of!(*self.param_header) as usize
            + ((self.id_repository_info.start_offset as usize + 15) & !0xF);
        std::slice::from_raw_parts_mut(
            pointer as *mut IdRepositoryEntry,
            self.id_repository_info.entry_count as usize,
        )
    }
    pub unsafe fn get_param_row(&self, entry_id: i32) -> Option<&'static P::Def> {
        let param_table = self.get_param_table();
        for entry in param_table {
            if entry.param_id == entry_id {
                return Some(mem::transmute(
                    addr_of!(*self.param_header) as usize + entry.param_offset as usize,
                ));
            }
        }
        None
    }
    pub unsafe fn get_param_row_mut(&self, entry_id: i32) -> Option<&'static mut P::Def> {
        let param_table = self.get_param_table();
        for entry in param_table {
            if entry.param_id == entry_id {
                return Some(mem::transmute(
                    addr_of!(*self.param_header) as usize + entry.param_offset as usize,
                ));
            }
        }
        None
    }
    pub unsafe fn get_row_from_table(&self, table: &TableEntry) -> &'static P::Def {
        self.get_row_from_table_mut(table)
    }
    pub unsafe fn get_row_from_table_mut(&self, table: &TableEntry) -> &'static mut P::Def {
        mem::transmute(addr_of!(*self.param_header) as usize + table.param_offset as usize)
    }
    unsafe fn find_param_res_cap(&self) -> Option<&'static ParamResCap> {
        let solo_param_entries = &self.solo_param_repository.repository_entries;

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

impl<P: Param> Clone for ParamEditor<P> {
    fn clone(&self) -> Self {
        ParamEditor {
            solo_param_repository: self.solo_param_repository,
            param_res_cap: self.param_res_cap,
            param_header: self.param_header,
            id_repository_info: self.id_repository_info,
            phantom_data: Default::default(),
        }
    }
}

impl<P: Param> Copy for ParamEditor<P> {}

impl<P: Param + 'static> IntoIterator for ParamEditor<P> {
    type Item = (i32, &'static mut P::Def);
    type IntoIter = ParamIterator<P>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            ParamIterator {
                param: self,
                table: self.get_param_table(),
                index: 0,
            }
        }
    }
}

impl<P: Param> ParamEditor<P> {
    pub fn init_from_file() {

    }
}
pub struct ParamIterator<P: Param> {
    param: ParamEditor<P>,
    table: &'static [TableEntry],
    index: usize,
}

impl<P: Param + 'static> Iterator for ParamIterator<P> {
    type Item = (i32, &'static mut P::Def);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.index >= self.table.len() {
                return None;
            }
            let t = (self.table[self.index].param_id, self.param.get_row_from_table_mut(&self.table[self.index]));
            self.index += 1;
            Some(t)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::param_editor;

    #[test]
    fn lol() {
    }
}