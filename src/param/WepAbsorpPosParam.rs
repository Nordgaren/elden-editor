/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WEP_ABSORP_POS_PARAM_ST.rs");

/// Type: WEP_ABSORP_POS_PARAM_ST

pub type WepAbsorpPosParam = ParamStruct<WEP_ABSORP_POS_PARAM_ST>;
impl Param for ParamStruct<WEP_ABSORP_POS_PARAM_ST> {
	const NAME: &'static str = "WepAbsorpPosParam";
	const TYPE_NAME: &'static str = "WEP_ABSORP_POS_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WepAbsorpPosParam::WepAbsorpPosParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WepAbsorpPosParam>(), 96)
	}
}
