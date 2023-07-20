/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_PARAM_CUSTOM_WEAPON_ST {

	/// NAME: Weapon base ID - 武器ベースID
	/// DESC: Weapon base ID - 武器ベースID
	pub baseWepId:i32,

	/// NAME: Magic stone ID - 魔石ID
	/// DESC: Magic stone ID - 魔石ID
	pub gemId:i32,

	/// NAME: Enhancement value - 強化値
	/// DESC: Enhancement value - 強化値
	pub reinforceLv:u8,

	/// NAME: pad - pad
	/// DESC: pad - pad
	pub pad:[u8;7],
}

impl Paramdef for EQUIP_PARAM_CUSTOM_WEAPON_ST {
const NAME: &'static str = "EQUIP_PARAM_CUSTOM_WEAPON_ST";
const VERSION: u16 = 1;
}
