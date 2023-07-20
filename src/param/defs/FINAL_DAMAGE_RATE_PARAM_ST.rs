/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct FINAL_DAMAGE_RATE_PARAM_ST {

	/// NAME: Physical Rate
	/// DESC: Unk
	pub physrate:f32,

	/// NAME: Magic Rate
	/// DESC: Unk
	pub magrate:f32,

	/// NAME: Fire Rate
	/// DESC: Unk
	pub firerate:f32,

	/// NAME: Lightning Rate
	/// DESC: Unk
	pub thunrate:f32,

	/// NAME: Dark Rate
	/// DESC: Unk
	pub darkrate:f32,

	/// NAME: Stamina Rate
	/// DESC: Unk
	pub staminarate:f32,

	/// NAME: Super Armor Rate
	/// DESC: Unk
	pub sarate:f32,
}

impl Paramdef for FINAL_DAMAGE_RATE_PARAM_ST {
const NAME: &'static str = "FINAL_DAMAGE_RATE_PARAM_ST";
const VERSION: u16 = 1;
}
