/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SIGN_PUDDLE_PARAM_ST.rs");

/// Type: SIGN_PUDDLE_PARAM_ST

pub type SignPuddleParam = ParamStruct<SIGN_PUDDLE_PARAM_ST>;
impl Param for ParamStruct<SIGN_PUDDLE_PARAM_ST> {
	const NAME: &'static str = "SignPuddleParam";
	const TYPE_NAME: &'static str = "SIGN_PUDDLE_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SignPuddleParam::SignPuddleParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SignPuddleParam>(), 32)
	}
}
