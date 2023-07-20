/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CHARMAKEMENUTOP_PARAM_ST::CHARMAKEMENUTOP_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CHARMAKEMENUTOP_PARAM_ST

pub struct CharMakeMenuTopParam {
    _data: CHARMAKEMENUTOP_PARAM_ST,
}
impl Param for CharMakeMenuTopParam {
    type Def = CHARMAKEMENUTOP_PARAM_ST;
    const NAME: &'static str = "CharMakeMenuTopParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CharMakeMenuTopParam {
    type Target = CHARMAKEMENUTOP_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CharMakeMenuTopParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CharMakeMenuTopParam::CharMakeMenuTopParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CharMakeMenuTopParam>(), 48)
    }
}
