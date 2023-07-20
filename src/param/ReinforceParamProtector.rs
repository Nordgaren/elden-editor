/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::REINFORCE_PARAM_PROTECTOR_ST::REINFORCE_PARAM_PROTECTOR_ST;

/// Type: REINFORCE_PARAM_PROTECTOR_ST

pub struct ReinforceParamProtector {
	_data: REINFORCE_PARAM_PROTECTOR_ST
}
impl Param for ReinforceParamProtector {
	type Def = REINFORCE_PARAM_PROTECTOR_ST;
	const NAME: &'static str = "ReinforceParamProtector";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ReinforceParamProtector {
	type Target = REINFORCE_PARAM_PROTECTOR_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ReinforceParamProtector {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ReinforceParamProtector::ReinforceParamProtector;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ReinforceParamProtector>(), 64)
	}
}
