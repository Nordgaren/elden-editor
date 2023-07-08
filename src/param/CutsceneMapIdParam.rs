/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CUTSCENE_MAP_ID_PARAM_ST.rs");

/// Type: CUTSCENE_MAP_ID_PARAM_ST

pub type CutsceneMapIdParam = ParamStruct<CUTSCENE_MAP_ID_PARAM_ST>;
impl Param for ParamStruct<CUTSCENE_MAP_ID_PARAM_ST> {
	const NAME: &'static str = "CutsceneMapIdParam";
	const TYPE_NAME: &'static str = "CUTSCENE_MAP_ID_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CutsceneMapIdParam::CutsceneMapIdParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CutsceneMapIdParam>(), 48)
	}
}
