/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/WWISE_VALUE_TO_STR_CONVERT_PARAM_ST.rs");

/// Type: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST

pub type WwiseValueToStrParam_Switch_AttackType = ParamStruct<WWISE_VALUE_TO_STR_CONVERT_PARAM_ST>;
impl Param for ParamStruct<WWISE_VALUE_TO_STR_CONVERT_PARAM_ST> {
	const NAME: &'static str = "WwiseValueToStrParam_Switch_AttackType";
	const TYPE_NAME: &'static str = "WWISE_VALUE_TO_STR_CONVERT_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WwiseValueToStrParam_Switch_AttackType::WwiseValueToStrParam_Switch_AttackType;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WwiseValueToStrParam_Switch_AttackType>(), 36)
	}
}
