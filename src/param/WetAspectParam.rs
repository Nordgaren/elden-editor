/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WET_ASPECT_PARAM_ST::WET_ASPECT_PARAM_ST;

/// Type: WET_ASPECT_PARAM_ST

pub struct WetAspectParam {
	_data: WET_ASPECT_PARAM_ST
}
impl Param for WetAspectParam {
	type Def = WET_ASPECT_PARAM_ST;
	const NAME: &'static str = "WetAspectParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WetAspectParam {
	type Target = WET_ASPECT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WetAspectParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WetAspectParam::WetAspectParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WetAspectParam>(), 32)
	}
}
