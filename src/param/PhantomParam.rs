/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/PHANTOM_PARAM_ST.rs");

/// Type: PHANTOM_PARAM_ST

pub type PhantomParam = ParamStruct<PHANTOM_PARAM_ST>;
impl Param for ParamStruct<PHANTOM_PARAM_ST> {
	const NAME: &'static str = "PhantomParam";
	const TYPE_NAME: &'static str = "PHANTOM_PARAM_ST";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PhantomParam::PhantomParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PhantomParam>(), 56)
	}
}
