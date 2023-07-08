/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_NAME_TEX_PARAM_ST.rs");

/// Type: MAP_NAME_TEX_PARAM_ST

pub type MapNameTexParam = ParamStruct<MAP_NAME_TEX_PARAM_ST>;
impl Param for ParamStruct<MAP_NAME_TEX_PARAM_ST> {
	const NAME: &'static str = "MapNameTexParam";
	const TYPE_NAME: &'static str = "MAP_NAME_TEX_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapNameTexParam::MapNameTexParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapNameTexParam>(), 16)
	}
}
