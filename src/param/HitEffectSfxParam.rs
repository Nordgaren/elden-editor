/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/HIT_EFFECT_SFX_PARAM_ST.rs");

/// Type: HIT_EFFECT_SFX_PARAM_ST

pub type HitEffectSfxParam = ParamStruct<HIT_EFFECT_SFX_PARAM_ST>;
impl Param for ParamStruct<HIT_EFFECT_SFX_PARAM_ST> {
	const NAME: &'static str = "HitEffectSfxParam";
	const TYPE_NAME: &'static str = "HIT_EFFECT_SFX_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::HitEffectSfxParam::HitEffectSfxParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<HitEffectSfxParam>(), 80)
	}
}
