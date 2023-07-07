/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/REINFORCE_PARAM_PROTECTOR_ST.rs");

/// Type: REINFORCE_PARAM_PROTECTOR_ST

pub type ReinforceParamProtector = ParamStruct<REINFORCE_PARAM_PROTECTOR_ST>;
impl Param for ParamStruct<REINFORCE_PARAM_PROTECTOR_ST> {
	const NAME: &'static str = "ReinforceParamProtector";
	const TYPE_NAME: &'static str = "REINFORCE_PARAM_PROTECTOR_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ReinforceParamProtector::ReinforceParamProtector;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ReinforceParamProtector>(), 64)
	}
}
