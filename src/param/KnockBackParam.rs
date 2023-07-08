/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/KNOCKBACK_PARAM_ST.rs");

/// Type: KNOCKBACK_PARAM_ST

pub type KnockBackParam = ParamStruct<KNOCKBACK_PARAM_ST>;
impl Param for ParamStruct<KNOCKBACK_PARAM_ST> {
	const NAME: &'static str = "KnockBackParam";
	const TYPE_NAME: &'static str = "KNOCKBACK_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::KnockBackParam::KnockBackParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<KnockBackParam>(), 128)
	}
}
