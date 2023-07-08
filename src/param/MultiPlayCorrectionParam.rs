/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MULTI_PLAY_CORRECTION_PARAM_ST.rs");

/// Type: MULTI_PLAY_CORRECTION_PARAM_ST

pub type MultiPlayCorrectionParam = ParamStruct<MULTI_PLAY_CORRECTION_PARAM_ST>;
impl Param for ParamStruct<MULTI_PLAY_CORRECTION_PARAM_ST> {
	const NAME: &'static str = "MultiPlayCorrectionParam";
	const TYPE_NAME: &'static str = "MULTI_PLAY_CORRECTION_PARAM_ST";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MultiPlayCorrectionParam::MultiPlayCorrectionParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MultiPlayCorrectionParam>(), 32)
	}
}
