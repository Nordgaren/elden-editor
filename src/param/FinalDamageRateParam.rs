/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/FINAL_DAMAGE_RATE_PARAM_ST.rs");

/// Type: FINAL_DAMAGE_RATE_PARAM_ST

pub type FinalDamageRateParam = ParamStruct<FINAL_DAMAGE_RATE_PARAM_ST>;
impl Param for ParamStruct<FINAL_DAMAGE_RATE_PARAM_ST> {
	const NAME: &'static str = "FinalDamageRateParam";
	const TYPE_NAME: &'static str = "FINAL_DAMAGE_RATE_PARAM_ST";
	const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::FinalDamageRateParam::FinalDamageRateParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<FinalDamageRateParam>(), 28)
	}
}
