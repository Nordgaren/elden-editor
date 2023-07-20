/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MENUPROPERTY_SPEC::MENUPROPERTY_SPEC;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MENUPROPERTY_SPEC

pub struct MenuPropertySpecParam {
    _data: MENUPROPERTY_SPEC,
}
impl Param for MenuPropertySpecParam {
    type Def = MENUPROPERTY_SPEC;
    const NAME: &'static str = "MenuPropertySpecParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MenuPropertySpecParam {
    type Target = MENUPROPERTY_SPEC;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MenuPropertySpecParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MenuPropertySpecParam::MenuPropertySpecParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuPropertySpecParam>(), 32)
    }
}
