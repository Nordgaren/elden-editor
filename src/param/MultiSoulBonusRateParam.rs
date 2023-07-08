/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MULTI_SOUL_BONUS_RATE_PARAM_ST.rs");

/// Type: MULTI_SOUL_BONUS_RATE_PARAM_ST

pub type MultiSoulBonusRateParam = ParamStruct<MULTI_SOUL_BONUS_RATE_PARAM_ST>;
impl Param for ParamStruct<MULTI_SOUL_BONUS_RATE_PARAM_ST> {
    const NAME: &'static str = "MultiSoulBonusRateParam";
    const TYPE_NAME: &'static str = "MULTI_SOUL_BONUS_RATE_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::MultiSoulBonusRateParam::MultiSoulBonusRateParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MultiSoulBonusRateParam>(), 128)
    }
}
