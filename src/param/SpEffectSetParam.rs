/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SP_EFFECT_SET_PARAM_ST.rs");

/// Type: SP_EFFECT_SET_PARAM_ST

pub type SpEffectSetParam = ParamStruct<SP_EFFECT_SET_PARAM_ST>;
impl Param for ParamStruct<SP_EFFECT_SET_PARAM_ST> {
    const NAME: &'static str = "SpEffectSetParam";
    const TYPE_NAME: &'static str = "SP_EFFECT_SET_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::SpEffectSetParam::SpEffectSetParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SpEffectSetParam>(), 16)
    }
}
