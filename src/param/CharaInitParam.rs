/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CHARACTER_INIT_PARAM::CHARACTER_INIT_PARAM;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CHARACTER_INIT_PARAM

pub struct CharaInitParam {
    _data: CHARACTER_INIT_PARAM,
}
impl Param for CharaInitParam {
    type Def = CHARACTER_INIT_PARAM;
    const NAME: &'static str = "CharaInitParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CharaInitParam {
    type Target = CHARACTER_INIT_PARAM;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CharaInitParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CharaInitParam::CharaInitParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CharaInitParam>(), 320)
    }
}
