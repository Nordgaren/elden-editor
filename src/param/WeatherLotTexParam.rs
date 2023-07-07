/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WEATHER_LOT_TEX_PARAM_ST.rs");

/// Type: WEATHER_LOT_TEX_PARAM_ST

pub type WeatherLotTexParam = ParamStruct<WEATHER_LOT_TEX_PARAM_ST>;
impl Param for ParamStruct<WEATHER_LOT_TEX_PARAM_ST> {
	const NAME: &'static str = "WeatherLotTexParam";
	const TYPE_NAME: &'static str = "WEATHER_LOT_TEX_PARAM_ST";
	const VERSION: u16 = 1;
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
