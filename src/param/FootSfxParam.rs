/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::FOOT_SFX_PARAM_ST::FOOT_SFX_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: FOOT_SFX_PARAM_ST

pub struct FootSfxParam {
    _data: FOOT_SFX_PARAM_ST,
}
impl Param for FootSfxParam {
    type Def = FOOT_SFX_PARAM_ST;
    const NAME: &'static str = "FootSfxParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for FootSfxParam {
    type Target = FOOT_SFX_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for FootSfxParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::FootSfxParam::FootSfxParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<FootSfxParam>(), 800)
    }
}
