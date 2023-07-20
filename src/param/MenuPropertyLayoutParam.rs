/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MENUPROPERTY_LAYOUT::MENUPROPERTY_LAYOUT;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MENUPROPERTY_LAYOUT

pub struct MenuPropertyLayoutParam {
    _data: MENUPROPERTY_LAYOUT,
}
impl Param for MenuPropertyLayoutParam {
    type Def = MENUPROPERTY_LAYOUT;
    const NAME: &'static str = "MenuPropertyLayoutParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MenuPropertyLayoutParam {
    type Target = MENUPROPERTY_LAYOUT;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MenuPropertyLayoutParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MenuPropertyLayoutParam::MenuPropertyLayoutParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuPropertyLayoutParam>(), 32)
    }
}
