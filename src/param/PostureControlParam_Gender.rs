/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/POSTURE_CONTROL_PARAM_GENDER_ST.rs");

/// Type: POSTURE_CONTROL_PARAM_GENDER_ST

pub type PostureControlParam_Gender = ParamStruct<POSTURE_CONTROL_PARAM_GENDER_ST>;
impl Param for ParamStruct<POSTURE_CONTROL_PARAM_GENDER_ST> {
	const NAME: &'static str = "PostureControlParam_Gender";
	const TYPE_NAME: &'static str = "POSTURE_CONTROL_PARAM_GENDER_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PostureControlParam_Gender::PostureControlParam_Gender;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PostureControlParam_Gender>(), 64)
	}
}
