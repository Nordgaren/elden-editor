/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct POSTURE_CONTROL_PARAM_WEP_LEFT_ST {

	/// NAME: Left arm_front and back - 左腕_前後
	/// DESC: Left arm_front and back - 左腕_前後
	pub a000_leftArmFB:i16,

	/// NAME: Left wrist_front and back - 左手首_前後
	/// DESC: Left wrist_front and back - 左手首_前後
	pub a000_leftWristFB:i16,

	/// NAME: Left wrist_inside and outside - 左手首_内外
	/// DESC: Left wrist_inside and outside - 左手首_内外
	pub a000_leftWristIO:i16,

	/// NAME: Left arm_front and back - 左腕_前後
	/// DESC: Left arm_front and back - 左腕_前後
	pub a002_leftArmFB:i16,

	/// NAME: Left wrist_front and back - 左手首_前後
	/// DESC: Left wrist_front and back - 左手首_前後
	pub a002_leftWristFB:i16,

	/// NAME: Left wrist_inside and outside - 左手首_内外
	/// DESC: Left wrist_inside and outside - 左手首_内外
	pub a002_leftWristIO:i16,

	/// NAME: Left arm_front and back - 左腕_前後
	/// DESC: Left arm_front and back - 左腕_前後
	pub a003_leftArmFB:i16,

	/// NAME: Left wrist_front and back - 左手首_前後
	/// DESC: Left wrist_front and back - 左手首_前後
	pub a003_leftWristFB:i16,

	/// NAME: Left wrist_inside and outside - 左手首_内外
	/// DESC: Left wrist_inside and outside - 左手首_内外
	pub a003_leftWristIO:i16,

	/// NAME: pad - pad
	pub pad:[u8;14],
}

impl Paramdef for POSTURE_CONTROL_PARAM_WEP_LEFT_ST {
const NAME: &'static str = "POSTURE_CONTROL_PARAM_WEP_LEFT_ST";
const VERSION: u16 = 1;
}
