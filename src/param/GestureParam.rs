/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GESTURE_PARAM_ST.rs");

/// Type: GESTURE_PARAM_ST

pub type GestureParam = ParamStruct<GESTURE_PARAM_ST>;
impl Param for ParamStruct<GESTURE_PARAM_ST> {
	const NAME: &'static str = "GestureParam";
	const TYPE_NAME: &'static str = "GESTURE_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GestureParam::GestureParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GestureParam>(), 16)
	}
}
