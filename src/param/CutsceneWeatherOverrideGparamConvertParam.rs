/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST.rs");

/// Type: CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST

pub type CutsceneWeatherOverrideGparamConvertParam = ParamStruct<CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST>;
impl Param for ParamStruct<CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST> {
	const NAME: &'static str = "CutsceneWeatherOverrideGparamConvertParam";
	const TYPE_NAME: &'static str = "CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CutsceneWeatherOverrideGparamConvertParam::CutsceneWeatherOverrideGparamConvertParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CutsceneWeatherOverrideGparamConvertParam>(), 4)
	}
}
