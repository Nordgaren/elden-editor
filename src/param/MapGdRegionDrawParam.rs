/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_GD_REGION_DRAW_PARAM.rs");

/// Type: MAP_GD_REGION_DRAW_PARAM

pub type MapGdRegionDrawParam = ParamStruct<MAP_GD_REGION_DRAW_PARAM>;
impl Param for ParamStruct<MAP_GD_REGION_DRAW_PARAM> {
	const NAME: &'static str = "MapGdRegionDrawParam";
	const TYPE_NAME: &'static str = "MAP_GD_REGION_DRAW_PARAM";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapGdRegionDrawParam::MapGdRegionDrawParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapGdRegionDrawParam>(), 8)
	}
}
