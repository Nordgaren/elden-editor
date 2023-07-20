/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::RIDE_PARAM_ST::RIDE_PARAM_ST;

/// Type: RIDE_PARAM_ST

pub struct RideParam {
	_data: RIDE_PARAM_ST
}
impl Param for RideParam {
	type Def = RIDE_PARAM_ST;
	const NAME: &'static str = "RideParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for RideParam {
	type Target = RIDE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for RideParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::RideParam::RideParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<RideParam>(), 64)
	}
}
