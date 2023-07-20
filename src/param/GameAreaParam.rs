/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::GAME_AREA_PARAM_ST::GAME_AREA_PARAM_ST;

/// Type: GAME_AREA_PARAM_ST

pub struct GameAreaParam {
	_data: GAME_AREA_PARAM_ST
}
impl Param for GameAreaParam {
	type Def = GAME_AREA_PARAM_ST;
	const NAME: &'static str = "GameAreaParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for GameAreaParam {
	type Target = GAME_AREA_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for GameAreaParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GameAreaParam::GameAreaParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GameAreaParam>(), 96)
	}
}
