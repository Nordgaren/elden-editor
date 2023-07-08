/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SP_EFFECT_PARAM_ST.rs");

/// Type: SP_EFFECT_PARAM_ST

pub type SpEffectParam = ParamStruct<SP_EFFECT_PARAM_ST>;
impl Param for ParamStruct<SP_EFFECT_PARAM_ST> {
    const NAME: &'static str = "SpEffectParam";
    const TYPE_NAME: &'static str = "SP_EFFECT_PARAM_ST";
    const VERSION: u16 = 4;
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
