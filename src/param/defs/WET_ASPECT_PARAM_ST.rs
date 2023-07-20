/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WET_ASPECT_PARAM_ST {

	/// NAME: Base color value R - ベースカラー 値R
	/// DESC: Base color color R. - ベースカラー色Rです。
	pub baseColorR:u8,

	/// NAME: Base color value G - ベースカラー 値G
	/// DESC: Base color color G. - ベースカラー色Gです。
	pub baseColorG:u8,

	/// NAME: Base color value B - ベースカラー 値B
	/// DESC: Base color color B. - ベースカラー色Bです。
	pub baseColorB:u8,

	/// NAME: Spare 1 - 予備1
	pub reserve_0:[u8;1],

	/// NAME: Base color% - ベースカラー ％
	/// DESC: Base color override rate. - ベースカラーのオーバーライド率です。 
	pub baseColorA:f32,

	/// NAME: Metallic value - メタリック 値
	/// DESC: It's metallic. - メタリックです。
	pub metallic:u8,

	/// NAME: Spare 2 - 予備2
	pub reserve_1:[u8;1],

	/// NAME: Spare 3 - 予備3
	pub reserve_2:[u8;1],

	/// NAME: Spare 4 - 予備4
	pub reserve_3:[u8;1],

	/// NAME: Metallic% - メタリック ％
	/// DESC: Metallic override rate. - メタリックのオーバーライド率です。
	pub metallicRate:f32,

	/// NAME: Shininess% - シャイニネス ％
	/// DESC: Shininess override rate. - シャイニネスのオーバーライド率です。
	pub shininessRate:f32,

	/// NAME: Shininess value - シャイニネス 値
	/// DESC: Shininess. - シャイニネスです。 
	pub shininess:u8,

	/// NAME: Spare 5 - 予備5
	pub reserve_4:[u8;11],
}

impl Paramdef for WET_ASPECT_PARAM_ST {
const NAME: &'static str = "WET_ASPECT_PARAM_ST";
const VERSION: u16 = 1;
}
