/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CLEAR_COUNT_CORRECT_PARAM_ST.rs");

/// Type: CLEAR_COUNT_CORRECT_PARAM_ST

pub type ClearCountCorrectParam = ParamStruct<CLEAR_COUNT_CORRECT_PARAM_ST>;
impl Param for ParamStruct<CLEAR_COUNT_CORRECT_PARAM_ST> {
    const NAME: &'static str = "ClearCountCorrectParam";
    const TYPE_NAME: &'static str = "CLEAR_COUNT_CORRECT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::ClearCountCorrectParam::ClearCountCorrectParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ClearCountCorrectParam>(), 128)
    }
}
