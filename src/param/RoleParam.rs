/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ROLE_PARAM_ST.rs");

/// Type: ROLE_PARAM_ST

pub type RoleParam = ParamStruct<ROLE_PARAM_ST>;
impl Param for ParamStruct<ROLE_PARAM_ST> {
	const NAME: &'static str = "RoleParam";
	const TYPE_NAME: &'static str = "ROLE_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::RoleParam::RoleParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<RoleParam>(), 128)
	}
}
