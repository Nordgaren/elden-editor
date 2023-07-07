/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SP_EFFECT_VFX_PARAM_ST.rs");

/// Type: SP_EFFECT_VFX_PARAM_ST

pub type SpEffectVfxParam = ParamStruct<SP_EFFECT_VFX_PARAM_ST>;
impl Param for ParamStruct<SP_EFFECT_VFX_PARAM_ST> {
	const NAME: &'static str = "SpEffectVfxParam";
	const TYPE_NAME: &'static str = "SP_EFFECT_VFX_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SpEffectVfxParam::SpEffectVfxParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SpEffectVfxParam>(), 164)
	}
}
