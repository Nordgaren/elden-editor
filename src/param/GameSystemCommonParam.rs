/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GAME_SYSTEM_COMMON_PARAM_ST.rs");

/// Type: GAME_SYSTEM_COMMON_PARAM_ST

pub type GameSystemCommonParam = ParamStruct<GAME_SYSTEM_COMMON_PARAM_ST>;
impl Param for ParamStruct<GAME_SYSTEM_COMMON_PARAM_ST> {
	const NAME: &'static str = "GameSystemCommonParam";
	const TYPE_NAME: &'static str = "GAME_SYSTEM_COMMON_PARAM_ST";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GameSystemCommonParam::GameSystemCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GameSystemCommonParam>(), 880)
	}
}
