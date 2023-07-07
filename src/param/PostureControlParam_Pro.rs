/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/POSTURE_CONTROL_PARAM_PRO_ST.rs");

/// Type: POSTURE_CONTROL_PARAM_PRO_ST

pub type PostureControlParam_Pro = ParamStruct<POSTURE_CONTROL_PARAM_PRO_ST>;
impl Param for ParamStruct<POSTURE_CONTROL_PARAM_PRO_ST> {
	const NAME: &'static str = "PostureControlParam_Pro";
	const TYPE_NAME: &'static str = "POSTURE_CONTROL_PARAM_PRO_ST";
	const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PostureControlParam_Pro::PostureControlParam_Pro;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PostureControlParam_Pro>(), 80)
	}
}
