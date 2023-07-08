/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_DEFAULT_INFO_PARAM_ST.rs");

/// Type: MAP_DEFAULT_INFO_PARAM_ST

pub type MapDefaultInfoParam = ParamStruct<MAP_DEFAULT_INFO_PARAM_ST>;
impl Param for ParamStruct<MAP_DEFAULT_INFO_PARAM_ST> {
	const NAME: &'static str = "MapDefaultInfoParam";
	const TYPE_NAME: &'static str = "MAP_DEFAULT_INFO_PARAM_ST";
	const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapDefaultInfoParam::MapDefaultInfoParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapDefaultInfoParam>(), 64)
	}
}
