/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WEATHER_PARAM_ST::WEATHER_PARAM_ST;

/// Type: WEATHER_PARAM_ST

pub struct WeatherParam {
	_data: WEATHER_PARAM_ST
}
impl Param for WeatherParam {
	type Def = WEATHER_PARAM_ST;
	const NAME: &'static str = "WeatherParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WeatherParam {
	type Target = WEATHER_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WeatherParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WeatherParam::WeatherParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WeatherParam>(), 68)
	}
}
