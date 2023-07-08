/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CHR_ACTIVATE_CONDITION_PARAM_ST.rs");

/// Type: CHR_ACTIVATE_CONDITION_PARAM_ST

pub type ChrActivateConditionParam = ParamStruct<CHR_ACTIVATE_CONDITION_PARAM_ST>;
impl Param for ParamStruct<CHR_ACTIVATE_CONDITION_PARAM_ST> {
	const NAME: &'static str = "ChrActivateConditionParam";
	const TYPE_NAME: &'static str = "CHR_ACTIVATE_CONDITION_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ChrActivateConditionParam::ChrActivateConditionParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ChrActivateConditionParam>(), 8)
	}
}
