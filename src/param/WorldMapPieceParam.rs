/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WORLD_MAP_PIECE_PARAM_ST.rs");

/// Type: WORLD_MAP_PIECE_PARAM_ST

pub type WorldMapPieceParam = ParamStruct<WORLD_MAP_PIECE_PARAM_ST>;
impl Param for ParamStruct<WORLD_MAP_PIECE_PARAM_ST> {
	const NAME: &'static str = "WorldMapPieceParam";
	const TYPE_NAME: &'static str = "WORLD_MAP_PIECE_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WorldMapPieceParam::WorldMapPieceParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WorldMapPieceParam>(), 64)
	}
}
