/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WEATHER_ASSET_REPLACE_PARAM_ST::WEATHER_ASSET_REPLACE_PARAM_ST;

/// Type: WEATHER_ASSET_REPLACE_PARAM_ST

pub struct WeatherAssetReplaceParam {
	_data: WEATHER_ASSET_REPLACE_PARAM_ST
}
impl Param for WeatherAssetReplaceParam {
	type Def = WEATHER_ASSET_REPLACE_PARAM_ST;
	const NAME: &'static str = "WeatherAssetReplaceParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WeatherAssetReplaceParam {
	type Target = WEATHER_ASSET_REPLACE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WeatherAssetReplaceParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WeatherAssetReplaceParam::WeatherAssetReplaceParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WeatherAssetReplaceParam>(), 64)
	}
}
