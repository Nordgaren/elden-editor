/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/RESIST_CORRECT_PARAM_ST.rs");

/// Type: RESIST_CORRECT_PARAM_ST

pub type ResistCorrectParam = ParamStruct<RESIST_CORRECT_PARAM_ST>;
impl Param for ParamStruct<RESIST_CORRECT_PARAM_ST> {
	const NAME: &'static str = "ResistCorrectParam";
	const TYPE_NAME: &'static str = "RESIST_CORRECT_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ResistCorrectParam::ResistCorrectParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ResistCorrectParam>(), 40)
	}
}
