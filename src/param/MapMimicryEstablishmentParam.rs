/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MAP_MIMICRY_ESTABLISHMENT_PARAM_ST::MAP_MIMICRY_ESTABLISHMENT_PARAM_ST;

/// Type: MAP_MIMICRY_ESTABLISHMENT_PARAM_ST

pub struct MapMimicryEstablishmentParam {
	_data: MAP_MIMICRY_ESTABLISHMENT_PARAM_ST
}
impl Param for MapMimicryEstablishmentParam {
	type Def = MAP_MIMICRY_ESTABLISHMENT_PARAM_ST;
	const NAME: &'static str = "MapMimicryEstablishmentParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MapMimicryEstablishmentParam {
	type Target = MAP_MIMICRY_ESTABLISHMENT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MapMimicryEstablishmentParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapMimicryEstablishmentParam::MapMimicryEstablishmentParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapMimicryEstablishmentParam>(), 64)
	}
}
