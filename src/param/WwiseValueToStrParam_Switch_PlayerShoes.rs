/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WWISE_VALUE_TO_STR_CONVERT_PARAM_ST.rs");

/// Type: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST

pub type WwiseValueToStrParam_Switch_PlayerShoes = ParamStruct<WWISE_VALUE_TO_STR_CONVERT_PARAM_ST>;
impl Param for ParamStruct<WWISE_VALUE_TO_STR_CONVERT_PARAM_ST> {
    const NAME: &'static str = "WwiseValueToStrParam_Switch_PlayerShoes";
    const TYPE_NAME: &'static str = "WWISE_VALUE_TO_STR_CONVERT_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::WwiseValueToStrParam_Switch_PlayerShoes::WwiseValueToStrParam_Switch_PlayerShoes;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WwiseValueToStrParam_Switch_PlayerShoes>(), 36)
    }
}
