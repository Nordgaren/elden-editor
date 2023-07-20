/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WEATHER_LOT_TEX_PARAM_ST::WEATHER_LOT_TEX_PARAM_ST;

/// Type: WEATHER_LOT_TEX_PARAM_ST

pub struct WeatherLotTexParam {
	_data: WEATHER_LOT_TEX_PARAM_ST
}
impl Param for WeatherLotTexParam {
	type Def = WEATHER_LOT_TEX_PARAM_ST;
	const NAME: &'static str = "WeatherLotTexParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WeatherLotTexParam {
	type Target = WEATHER_LOT_TEX_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WeatherLotTexParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WeatherLotTexParam::WeatherLotTexParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WeatherLotTexParam>(), 16)
	}
}
