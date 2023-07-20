/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MENU_PARAM_COLOR_TABLE_ST::MENU_PARAM_COLOR_TABLE_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MENU_PARAM_COLOR_TABLE_ST

pub struct MenuColorTableParam {
    _data: MENU_PARAM_COLOR_TABLE_ST,
}
impl Param for MenuColorTableParam {
    type Def = MENU_PARAM_COLOR_TABLE_ST;
    const NAME: &'static str = "MenuColorTableParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MenuColorTableParam {
    type Target = MENU_PARAM_COLOR_TABLE_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MenuColorTableParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MenuColorTableParam::MenuColorTableParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuColorTableParam>(), 32)
    }
}
