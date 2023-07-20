/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::LOCK_CAM_PARAM_ST::LOCK_CAM_PARAM_ST;

/// Type: LOCK_CAM_PARAM_ST

pub struct LockCamParam {
	_data: LOCK_CAM_PARAM_ST
}
impl Param for LockCamParam {
	type Def = LOCK_CAM_PARAM_ST;
	const NAME: &'static str = "LockCamParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for LockCamParam {
	type Target = LOCK_CAM_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for LockCamParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::LockCamParam::LockCamParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<LockCamParam>(), 128)
	}
}
