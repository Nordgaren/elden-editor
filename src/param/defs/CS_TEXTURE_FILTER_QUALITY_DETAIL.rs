/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_TEXTURE_FILTER_QUALITY_DETAIL {

	/// NAME: filter - フィルター
	/// DESC: filter - フィルター
	pub filter:u8,

	/// NAME: dmy - dmy
	pub dmy:[u8;3],

	/// NAME: Aniso level - アニソレベル
	/// DESC: Aniso level - アニソレベル
	pub maxAnisoLevel:u32,
}

impl Paramdef for CS_TEXTURE_FILTER_QUALITY_DETAIL {
const NAME: &'static str = "CS_TEXTURE_FILTER_QUALITY_DETAIL";
const VERSION: u16 = 1;
}
