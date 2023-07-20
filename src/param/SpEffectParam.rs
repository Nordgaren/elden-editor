/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SP_EFFECT_PARAM_ST::SP_EFFECT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SP_EFFECT_PARAM_ST

pub struct SpEffectParam {
    _data: SP_EFFECT_PARAM_ST,
}
impl Param for SpEffectParam {
    type Def = SP_EFFECT_PARAM_ST;
    const NAME: &'static str = "SpEffectParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SpEffectParam {
    type Target = SP_EFFECT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SpEffectParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::SpEffectParam::SpEffectParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SpEffectParam>(), 912)
    }
}
