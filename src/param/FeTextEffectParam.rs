/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/FE_TEXT_EFFECT_PARAM_ST.rs");

/// Type: FE_TEXT_EFFECT_PARAM_ST

pub type FeTextEffectParam = ParamStruct<FE_TEXT_EFFECT_PARAM_ST>;
impl Param for ParamStruct<FE_TEXT_EFFECT_PARAM_ST> {
	const NAME: &'static str = "FeTextEffectParam";
	const TYPE_NAME: &'static str = "FE_TEXT_EFFECT_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::FeTextEffectParam::FeTextEffectParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<FeTextEffectParam>(), 32)
	}
}
