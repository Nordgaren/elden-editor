/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MENU_VALUE_TABLE_SPEC::MENU_VALUE_TABLE_SPEC;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MENU_VALUE_TABLE_SPEC

pub struct MenuValueTableParam {
    _data: MENU_VALUE_TABLE_SPEC,
}
impl Param for MenuValueTableParam {
    type Def = MENU_VALUE_TABLE_SPEC;
    const NAME: &'static str = "MenuValueTableParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MenuValueTableParam {
    type Target = MENU_VALUE_TABLE_SPEC;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MenuValueTableParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MenuValueTableParam::MenuValueTableParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuValueTableParam>(), 12)
    }
}
