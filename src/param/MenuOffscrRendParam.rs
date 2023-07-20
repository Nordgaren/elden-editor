/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MENU_OFFSCR_REND_PARAM_ST::MENU_OFFSCR_REND_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MENU_OFFSCR_REND_PARAM_ST

pub struct MenuOffscrRendParam {
    _data: MENU_OFFSCR_REND_PARAM_ST,
}
impl Param for MenuOffscrRendParam {
    type Def = MENU_OFFSCR_REND_PARAM_ST;
    const NAME: &'static str = "MenuOffscrRendParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MenuOffscrRendParam {
    type Target = MENU_OFFSCR_REND_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MenuOffscrRendParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MenuOffscrRendParam::MenuOffscrRendParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuOffscrRendParam>(), 64)
    }
}
