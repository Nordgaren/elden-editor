use widestring::U16CStr;
use crate::fmg_editor::{Fmg, FmgEditor};
use crate::fmg_editor::structs::MsgRepositoryGroup;

impl IntoIterator for &FmgEditor {
    type Item = (i32, &'static U16CStr);
    type IntoIter = FmgIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.fmg.into_iter()
    }
}

impl IntoIterator for Fmg {
    type Item = (i32, &'static U16CStr);
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
    type Item = (i32, &'static U16CStr);

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
