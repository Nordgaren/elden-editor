/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::HIT_MTRL_PARAM_ST::HIT_MTRL_PARAM_ST;

/// Type: HIT_MTRL_PARAM_ST

pub struct HitMtrlParam {
	_data: HIT_MTRL_PARAM_ST
}
impl Param for HitMtrlParam {
	type Def = HIT_MTRL_PARAM_ST;
	const NAME: &'static str = "HitMtrlParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for HitMtrlParam {
	type Target = HIT_MTRL_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for HitMtrlParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::HitMtrlParam::HitMtrlParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<HitMtrlParam>(), 100)
	}
}
