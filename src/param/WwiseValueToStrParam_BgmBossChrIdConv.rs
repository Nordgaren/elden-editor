/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WWISE_VALUE_TO_STR_CONVERT_PARAM_ST.rs");

/// Type: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST

pub type WwiseValueToStrParam_BgmBossChrIdConv = ParamStruct<WWISE_VALUE_TO_STR_CONVERT_PARAM_ST>;
impl Param for ParamStruct<WWISE_VALUE_TO_STR_CONVERT_PARAM_ST> {
	const NAME: &'static str = "WwiseValueToStrParam_BgmBossChrIdConv";
	const TYPE_NAME: &'static str = "WWISE_VALUE_TO_STR_CONVERT_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WwiseValueToStrParam_BgmBossChrIdConv::WwiseValueToStrParam_BgmBossChrIdConv;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WwiseValueToStrParam_BgmBossChrIdConv>(), 36)
	}
}
