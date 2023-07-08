/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MULTI_ESTUS_FLASK_BONUS_PARAM_ST.rs");

/// Type: MULTI_ESTUS_FLASK_BONUS_PARAM_ST

pub type MultiHPEstusFlaskBonusParam = ParamStruct<MULTI_ESTUS_FLASK_BONUS_PARAM_ST>;
impl Param for ParamStruct<MULTI_ESTUS_FLASK_BONUS_PARAM_ST> {
    const NAME: &'static str = "MultiHPEstusFlaskBonusParam";
    const TYPE_NAME: &'static str = "MULTI_ESTUS_FLASK_BONUS_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::MultiHPEstusFlaskBonusParam::MultiHPEstusFlaskBonusParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MultiHPEstusFlaskBonusParam>(), 64)
    }
}
