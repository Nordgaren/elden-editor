/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/DIRECTION_CAMERA_PARAM_ST.rs");

/// Type: DIRECTION_CAMERA_PARAM_ST

pub type DirectionCameraParam = ParamStruct<DIRECTION_CAMERA_PARAM_ST>;
impl Param for ParamStruct<DIRECTION_CAMERA_PARAM_ST> {
	const NAME: &'static str = "DirectionCameraParam";
	const TYPE_NAME: &'static str = "DIRECTION_CAMERA_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::DirectionCameraParam::DirectionCameraParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<DirectionCameraParam>(), 16)
	}
}
