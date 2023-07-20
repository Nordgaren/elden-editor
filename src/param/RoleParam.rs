/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ROLE_PARAM_ST::ROLE_PARAM_ST;

/// Type: ROLE_PARAM_ST

pub struct RoleParam {
	_data: ROLE_PARAM_ST
}
impl Param for RoleParam {
	type Def = ROLE_PARAM_ST;
	const NAME: &'static str = "RoleParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for RoleParam {
	type Target = ROLE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for RoleParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
