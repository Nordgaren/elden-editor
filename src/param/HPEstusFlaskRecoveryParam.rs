/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ESTUS_FLASK_RECOVERY_PARAM_ST::ESTUS_FLASK_RECOVERY_PARAM_ST;

/// Type: ESTUS_FLASK_RECOVERY_PARAM_ST

pub struct HPEstusFlaskRecoveryParam {
	_data: ESTUS_FLASK_RECOVERY_PARAM_ST
}
impl Param for HPEstusFlaskRecoveryParam {
	type Def = ESTUS_FLASK_RECOVERY_PARAM_ST;
	const NAME: &'static str = "HPEstusFlaskRecoveryParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for HPEstusFlaskRecoveryParam {
	type Target = ESTUS_FLASK_RECOVERY_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for HPEstusFlaskRecoveryParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::HPEstusFlaskRecoveryParam::HPEstusFlaskRecoveryParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<HPEstusFlaskRecoveryParam>(), 32)
	}
}
