/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GAME_AREA_PARAM_ST.rs");

/// Type: GAME_AREA_PARAM_ST

pub type GameAreaParam = ParamStruct<GAME_AREA_PARAM_ST>;
impl Param for ParamStruct<GAME_AREA_PARAM_ST> {
	const NAME: &'static str = "GameAreaParam";
	const TYPE_NAME: &'static str = "GAME_AREA_PARAM_ST";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GameAreaParam::GameAreaParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GameAreaParam>(), 96)
	}
}
