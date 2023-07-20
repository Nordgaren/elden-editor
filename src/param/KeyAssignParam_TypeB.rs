/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::KEY_ASSIGN_PARAM_ST::KEY_ASSIGN_PARAM_ST;

/// Type: KEY_ASSIGN_PARAM_ST

pub struct KeyAssignParam_TypeB {
	_data: KEY_ASSIGN_PARAM_ST
}
impl Param for KeyAssignParam_TypeB {
	type Def = KEY_ASSIGN_PARAM_ST;
	const NAME: &'static str = "KeyAssignParam_TypeB";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for KeyAssignParam_TypeB {
	type Target = KEY_ASSIGN_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for KeyAssignParam_TypeB {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::KeyAssignParam_TypeB::KeyAssignParam_TypeB;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<KeyAssignParam_TypeB>(), 32)
	}
}
