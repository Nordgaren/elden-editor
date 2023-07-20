/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CHR_MODEL_PARAM_ST::CHR_MODEL_PARAM_ST;

/// Type: CHR_MODEL_PARAM_ST

pub struct ChrModelParam {
	_data: CHR_MODEL_PARAM_ST
}
impl Param for ChrModelParam {
	type Def = CHR_MODEL_PARAM_ST;
	const NAME: &'static str = "ChrModelParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ChrModelParam {
	type Target = CHR_MODEL_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ChrModelParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ChrModelParam::ChrModelParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ChrModelParam>(), 16)
	}
}
