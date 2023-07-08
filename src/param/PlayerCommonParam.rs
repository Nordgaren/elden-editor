/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/PLAYER_COMMON_PARAM_ST.rs");

/// Type: PLAYER_COMMON_PARAM_ST

pub type PlayerCommonParam = ParamStruct<PLAYER_COMMON_PARAM_ST>;
impl Param for ParamStruct<PLAYER_COMMON_PARAM_ST> {
	const NAME: &'static str = "PlayerCommonParam";
	const TYPE_NAME: &'static str = "PLAYER_COMMON_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PlayerCommonParam::PlayerCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PlayerCommonParam>(), 256)
	}
}
