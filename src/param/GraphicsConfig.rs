/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CS_GRAPHICS_CONFIG_PARAM_ST.rs");

/// Type: CS_GRAPHICS_CONFIG_PARAM_ST

pub type GraphicsConfig = ParamStruct<CS_GRAPHICS_CONFIG_PARAM_ST>;
impl Param for ParamStruct<CS_GRAPHICS_CONFIG_PARAM_ST> {
	const NAME: &'static str = "GraphicsConfig";
	const TYPE_NAME: &'static str = "CS_GRAPHICS_CONFIG_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GraphicsConfig::GraphicsConfig;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GraphicsConfig>(), 16)
	}
}
