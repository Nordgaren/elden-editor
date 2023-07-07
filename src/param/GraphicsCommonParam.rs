/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GRAPHICS_COMMON_PARAM_ST.rs");

/// Type: GRAPHICS_COMMON_PARAM_ST

pub type GraphicsCommonParam = ParamStruct<GRAPHICS_COMMON_PARAM_ST>;
impl Param for ParamStruct<GRAPHICS_COMMON_PARAM_ST> {
	const NAME: &'static str = "GraphicsCommonParam";
	const TYPE_NAME: &'static str = "GRAPHICS_COMMON_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GraphicsCommonParam::GraphicsCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GraphicsCommonParam>(), 256)
	}
}
