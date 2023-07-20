/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::GESTURE_PARAM_ST::GESTURE_PARAM_ST;

/// Type: GESTURE_PARAM_ST

pub struct GestureParam {
	_data: GESTURE_PARAM_ST
}
impl Param for GestureParam {
	type Def = GESTURE_PARAM_ST;
	const NAME: &'static str = "GestureParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for GestureParam {
	type Target = GESTURE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for GestureParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GestureParam::GestureParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GestureParam>(), 16)
	}
}
