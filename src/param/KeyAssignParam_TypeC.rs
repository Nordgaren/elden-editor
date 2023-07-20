/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::KEY_ASSIGN_PARAM_ST::KEY_ASSIGN_PARAM_ST;

/// Type: KEY_ASSIGN_PARAM_ST

pub struct KeyAssignParam_TypeC {
	_data: KEY_ASSIGN_PARAM_ST
}
impl Param for KeyAssignParam_TypeC {
	type Def = KEY_ASSIGN_PARAM_ST;
	const NAME: &'static str = "KeyAssignParam_TypeC";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for KeyAssignParam_TypeC {
	type Target = KEY_ASSIGN_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for KeyAssignParam_TypeC {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::KeyAssignParam_TypeC::KeyAssignParam_TypeC;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<KeyAssignParam_TypeC>(), 32)
	}
}
