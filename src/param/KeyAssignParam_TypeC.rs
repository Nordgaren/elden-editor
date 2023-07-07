/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/KEY_ASSIGN_PARAM_ST.rs");

/// Type: KEY_ASSIGN_PARAM_ST

pub type KeyAssignParam_TypeC = ParamStruct<KEY_ASSIGN_PARAM_ST>;
impl Param for ParamStruct<KEY_ASSIGN_PARAM_ST> {
	const NAME: &'static str = "KeyAssignParam_TypeC";
	const TYPE_NAME: &'static str = "KEY_ASSIGN_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::KeyAssignParam_TypeC::KeyAssignParam_TypeC;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<KeyAssignParam_TypeC>(), 32)
	}
}
