/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WEATHER_ASSET_REPLACE_PARAM_ST.rs");

/// Type: WEATHER_ASSET_REPLACE_PARAM_ST

pub type WeatherAssetReplaceParam = ParamStruct<WEATHER_ASSET_REPLACE_PARAM_ST>;
impl Param for ParamStruct<WEATHER_ASSET_REPLACE_PARAM_ST> {
	const NAME: &'static str = "WeatherAssetReplaceParam";
	const TYPE_NAME: &'static str = "WEATHER_ASSET_REPLACE_PARAM_ST";
	const VERSION: u16 = 1;
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
