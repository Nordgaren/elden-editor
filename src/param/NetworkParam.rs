/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NETWORK_PARAM_ST.rs");

/// Type: NETWORK_PARAM_ST

pub type NetworkParam = ParamStruct<NETWORK_PARAM_ST>;
impl Param for ParamStruct<NETWORK_PARAM_ST> {
	const NAME: &'static str = "NetworkParam";
	const TYPE_NAME: &'static str = "NETWORK_PARAM_ST";
	const VERSION: u16 = 10;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::NetworkParam::NetworkParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<NetworkParam>(), 632)
	}
}
