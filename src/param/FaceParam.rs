/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::FACE_PARAM_ST::FACE_PARAM_ST;

/// Type: FACE_PARAM_ST

pub struct FaceParam {
	_data: FACE_PARAM_ST
}
impl Param for FaceParam {
	type Def = FACE_PARAM_ST;
	const NAME: &'static str = "FaceParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for FaceParam {
	type Target = FACE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for FaceParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::FaceParam::FaceParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<FaceParam>(), 240)
	}
}
