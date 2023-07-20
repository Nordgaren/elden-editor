/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::NETWORK_AREA_PARAM_ST::NETWORK_AREA_PARAM_ST;

/// Type: NETWORK_AREA_PARAM_ST

pub struct NetworkAreaParam {
	_data: NETWORK_AREA_PARAM_ST
}
impl Param for NetworkAreaParam {
	type Def = NETWORK_AREA_PARAM_ST;
	const NAME: &'static str = "NetworkAreaParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for NetworkAreaParam {
	type Target = NETWORK_AREA_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for NetworkAreaParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::NetworkAreaParam::NetworkAreaParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<NetworkAreaParam>(), 28)
	}
}
