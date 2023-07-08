/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SPEEDTREE_MODEL_PARAM_ST.rs");

/// Type: SPEEDTREE_MODEL_PARAM_ST

pub type SpeedtreeParam = ParamStruct<SPEEDTREE_MODEL_PARAM_ST>;
impl Param for ParamStruct<SPEEDTREE_MODEL_PARAM_ST> {
	const NAME: &'static str = "SpeedtreeParam";
	const TYPE_NAME: &'static str = "SPEEDTREE_MODEL_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SpeedtreeParam::SpeedtreeParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SpeedtreeParam>(), 40)
	}
}
