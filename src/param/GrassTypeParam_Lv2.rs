/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GRASS_TYPE_PARAM_ST.rs");

/// Type: GRASS_TYPE_PARAM_ST

pub type GrassTypeParam_Lv2 = ParamStruct<GRASS_TYPE_PARAM_ST>;
impl Param for ParamStruct<GRASS_TYPE_PARAM_ST> {
	const NAME: &'static str = "GrassTypeParam_Lv2";
	const TYPE_NAME: &'static str = "GRASS_TYPE_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GrassTypeParam_Lv2::GrassTypeParam_Lv2;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GrassTypeParam_Lv2>(), 276)
	}
}
