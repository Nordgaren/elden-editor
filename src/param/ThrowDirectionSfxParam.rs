/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/THROW_DIRECTION_SFX_PARAM_ST.rs");

/// Type: THROW_DIRECTION_SFX_PARAM_ST

pub type ThrowDirectionSfxParam = ParamStruct<THROW_DIRECTION_SFX_PARAM_ST>;
impl Param for ParamStruct<THROW_DIRECTION_SFX_PARAM_ST> {
    const NAME: &'static str = "ThrowDirectionSfxParam";
    const TYPE_NAME: &'static str = "THROW_DIRECTION_SFX_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::ThrowDirectionSfxParam::ThrowDirectionSfxParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ThrowDirectionSfxParam>(), 144)
    }
}
