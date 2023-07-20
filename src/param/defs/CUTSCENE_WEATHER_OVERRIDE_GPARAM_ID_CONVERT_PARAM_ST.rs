/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST {

	/// NAME: Cutscene Weather Overwrite Gparam Suffix ID - カットシーン天候上書きGparamサフィックスID
	/// DESC: The ID of the ?? part of s00_00_0000_WeatherOverride_ ??. Gparam - s00_00_0000_WeatherOverride_??.gparamの??の部分のIDです
	pub weatherOverrideGparamId:u32,
}

impl Paramdef for CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST {
const NAME: &'static str = "CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST";
const VERSION: u16 = 1;
}
