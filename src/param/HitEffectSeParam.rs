/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/HIT_EFFECT_SE_PARAM_ST.rs");

/// Type: HIT_EFFECT_SE_PARAM_ST

pub type HitEffectSeParam = ParamStruct<HIT_EFFECT_SE_PARAM_ST>;
impl Param for ParamStruct<HIT_EFFECT_SE_PARAM_ST> {
	const NAME: &'static str = "HitEffectSeParam";
	const TYPE_NAME: &'static str = "HIT_EFFECT_SE_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::HitEffectSeParam::HitEffectSeParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<HitEffectSeParam>(), 640)
	}
}
