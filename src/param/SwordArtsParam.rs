/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SWORD_ARTS_PARAM_ST.rs");

/// Type: SWORD_ARTS_PARAM_ST

pub type SwordArtsParam = ParamStruct<SWORD_ARTS_PARAM_ST>;
impl Param for ParamStruct<SWORD_ARTS_PARAM_ST> {
	const NAME: &'static str = "SwordArtsParam";
	const TYPE_NAME: &'static str = "SWORD_ARTS_PARAM_ST";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SwordArtsParam::SwordArtsParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SwordArtsParam>(), 32)
	}
}
