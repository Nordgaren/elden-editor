/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::BEHAVIOR_PARAM_ST::BEHAVIOR_PARAM_ST;

/// Type: BEHAVIOR_PARAM_ST

pub struct BehaviorParam_PC {
	_data: BEHAVIOR_PARAM_ST
}
impl Param for BehaviorParam_PC {
	type Def = BEHAVIOR_PARAM_ST;
	const NAME: &'static str = "BehaviorParam_PC";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for BehaviorParam_PC {
	type Target = BEHAVIOR_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for BehaviorParam_PC {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BehaviorParam_PC::BehaviorParam_PC;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BehaviorParam_PC>(), 32)
	}
}
