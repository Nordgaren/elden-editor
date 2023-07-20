/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GPARAM_GRID_REGION_INFO_PARAM_ST {

	/// NAME: Open local ID for MapGparam - MapGparam用オープン地方ID 
	/// DESC: Open local ID for MapGparam. Used for the XX part of m60_00_00XX.gparamxml - MapGparam用オープンの地方ID。m60_00_00XX.gparamxmlのXXの部分に使用される
	pub GparamGridRegionId:u32,

	/// NAME: Reserve - リザーブ
	/// DESC: Reserve - リザーブ
	pub Reserve:[u8;28],
}

impl Paramdef for GPARAM_GRID_REGION_INFO_PARAM_ST {
const NAME: &'static str = "GPARAM_GRID_REGION_INFO_PARAM_ST";
const VERSION: u16 = 1;
}
