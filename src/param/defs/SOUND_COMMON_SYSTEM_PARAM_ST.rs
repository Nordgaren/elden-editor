/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 202
#[repr(C)]
pub struct SOUND_COMMON_SYSTEM_PARAM_ST {

	/// NAME: Parameter Key string - パラメータのKey文字列
	/// DESC: Key string of the parameter - パラメータのKey文字列です
	pub ParamKeyStr:[u8;32],

	/// NAME: Parameter Value string - パラメータのValue文字列
	/// DESC: Value string of the parameter - パラメータのValue文字列です
	pub ParamValueStr:[u8;32],
}

impl Paramdef for SOUND_COMMON_SYSTEM_PARAM_ST {
const NAME: &'static str = "SOUND_COMMON_SYSTEM_PARAM_ST";
const VERSION: u16 = 0;
}
