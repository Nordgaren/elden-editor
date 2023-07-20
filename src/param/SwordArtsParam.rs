/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SWORD_ARTS_PARAM_ST::SWORD_ARTS_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SWORD_ARTS_PARAM_ST

pub struct SwordArtsParam {
    _data: SWORD_ARTS_PARAM_ST,
}
impl Param for SwordArtsParam {
    type Def = SWORD_ARTS_PARAM_ST;
    const NAME: &'static str = "SwordArtsParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SwordArtsParam {
    type Target = SWORD_ARTS_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SwordArtsParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::SwordArtsParam::SwordArtsParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SwordArtsParam>(), 32)
    }
}
