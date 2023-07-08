/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SE_ACTIVATION_RANGE_PARAM_ST.rs");

/// Type: SE_ACTIVATION_RANGE_PARAM_ST

pub type SeActivationRangeParam = ParamStruct<SE_ACTIVATION_RANGE_PARAM_ST>;
impl Param for ParamStruct<SE_ACTIVATION_RANGE_PARAM_ST> {
    const NAME: &'static str = "SeActivationRangeParam";
    const TYPE_NAME: &'static str = "SE_ACTIVATION_RANGE_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::SeActivationRangeParam::SeActivationRangeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SeActivationRangeParam>(), 4)
    }
}
