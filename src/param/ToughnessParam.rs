/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/TOUGHNESS_PARAM_ST.rs");

/// Type: TOUGHNESS_PARAM_ST

pub type ToughnessParam = ParamStruct<TOUGHNESS_PARAM_ST>;
impl Param for ParamStruct<TOUGHNESS_PARAM_ST> {
    const NAME: &'static str = "ToughnessParam";
    const TYPE_NAME: &'static str = "TOUGHNESS_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::ToughnessParam::ToughnessParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ToughnessParam>(), 32)
    }
}
