/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ESTUS_FLASK_RECOVERY_PARAM_ST.rs");

/// Type: ESTUS_FLASK_RECOVERY_PARAM_ST

pub type MPEstusFlaskRecoveryParam = ParamStruct<ESTUS_FLASK_RECOVERY_PARAM_ST>;
impl Param for ParamStruct<ESTUS_FLASK_RECOVERY_PARAM_ST> {
	const NAME: &'static str = "MPEstusFlaskRecoveryParam";
	const TYPE_NAME: &'static str = "ESTUS_FLASK_RECOVERY_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MPEstusFlaskRecoveryParam::MPEstusFlaskRecoveryParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MPEstusFlaskRecoveryParam>(), 32)
	}
}
