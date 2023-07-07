/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/DECAL_PARAM_ST.rs");

/// Type: DECAL_PARAM_ST

pub type DecalParam = ParamStruct<DECAL_PARAM_ST>;
impl Param for ParamStruct<DECAL_PARAM_ST> {
	const NAME: &'static str = "DecalParam";
	const TYPE_NAME: &'static str = "DECAL_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::DecalParam::DecalParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<DecalParam>(), 248)
	}
}
