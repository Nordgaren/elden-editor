/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::HIT_EFFECT_SE_PARAM_ST::HIT_EFFECT_SE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: HIT_EFFECT_SE_PARAM_ST

pub struct HitEffectSeParam {
    _data: HIT_EFFECT_SE_PARAM_ST,
}
impl Param for HitEffectSeParam {
    type Def = HIT_EFFECT_SE_PARAM_ST;
    const NAME: &'static str = "HitEffectSeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for HitEffectSeParam {
    type Target = HIT_EFFECT_SE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for HitEffectSeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::HitEffectSeParam::HitEffectSeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<HitEffectSeParam>(), 640)
    }
}
