/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ESTUS_FLASK_RECOVERY_PARAM_ST::ESTUS_FLASK_RECOVERY_PARAM_ST;

/// Type: ESTUS_FLASK_RECOVERY_PARAM_ST

pub struct MPEstusFlaskRecoveryParam {
	_data: ESTUS_FLASK_RECOVERY_PARAM_ST
}
impl Param for MPEstusFlaskRecoveryParam {
	type Def = ESTUS_FLASK_RECOVERY_PARAM_ST;
	const NAME: &'static str = "MPEstusFlaskRecoveryParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MPEstusFlaskRecoveryParam {
	type Target = ESTUS_FLASK_RECOVERY_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MPEstusFlaskRecoveryParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
