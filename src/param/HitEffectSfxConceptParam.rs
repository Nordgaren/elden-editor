/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/HIT_EFFECT_SFX_CONCEPT_PARAM_ST.rs");

/// Type: HIT_EFFECT_SFX_CONCEPT_PARAM_ST

pub type HitEffectSfxConceptParam = ParamStruct<HIT_EFFECT_SFX_CONCEPT_PARAM_ST>;
impl Param for ParamStruct<HIT_EFFECT_SFX_CONCEPT_PARAM_ST> {
    const NAME: &'static str = "HitEffectSfxConceptParam";
    const TYPE_NAME: &'static str = "HIT_EFFECT_SFX_CONCEPT_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::HitEffectSfxConceptParam::HitEffectSfxConceptParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<HitEffectSfxConceptParam>(), 80)
    }
}
