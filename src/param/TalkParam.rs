/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/TALK_PARAM_ST.rs");

/// Type: TALK_PARAM_ST

pub type TalkParam = ParamStruct<TALK_PARAM_ST>;
impl Param for ParamStruct<TALK_PARAM_ST> {
	const NAME: &'static str = "TalkParam";
	const TYPE_NAME: &'static str = "TALK_PARAM_ST";
	const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::TalkParam::TalkParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<TalkParam>(), 96)
	}
}
