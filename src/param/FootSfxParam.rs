/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/FOOT_SFX_PARAM_ST.rs");

/// Type: FOOT_SFX_PARAM_ST

pub type FootSfxParam = ParamStruct<FOOT_SFX_PARAM_ST>;
impl Param for ParamStruct<FOOT_SFX_PARAM_ST> {
    const NAME: &'static str = "FootSfxParam";
    const TYPE_NAME: &'static str = "FOOT_SFX_PARAM_ST";
    const VERSION: u16 = 2;
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
