/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct FE_TEXT_EFFECT_PARAM_ST {

	/// NAME: Resource ID - リソースID
	/// DESC: Instance name of the menu resource. ID of TextEffect_X - メニューリソースのインスタンス名。TextEffect_X のID
	pub resId:i16,

	/// NAME: Padding - パディング
	pub pad1:[u8;2],

	/// NAME: Text ID - テキストID
	/// DESC: Text ID to display (-1: No text). MenuText - 表示するテキストID(-1: テキストなし)。MenuText
	pub textId:i32,

	/// NAME: SE ID - SEのID
	/// DESC: Voice ID to play (-1: No SE) - 再生するVoiceのID(-1: SEなし)
	pub seId:i32,

	/// NAME: Whether to display at the same time as the map name - マップ名と同時に表示するか
	/// DESC: Whether to display at the same time as the map name - マップ名と同時に表示するか
	pub Bitfield1:u8,

	/// NAME: Padding - パディング
	pub pad2:[u8;19],
}

impl Paramdef for FE_TEXT_EFFECT_PARAM_ST {
const NAME: &'static str = "FE_TEXT_EFFECT_PARAM_ST";
const VERSION: u16 = 1;
}
impl FE_TEXT_EFFECT_PARAM_ST {
	/// Whether to display at the same time as the map name - マップ名と同時に表示するか
	/// Bitfield1
	pub fn get_canMixMapName(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_canMixMapName(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// 
	/// Bitfield1
	pub fn get_pad3(&self) -> u8 {
		&self.Bitfield1 & 0xFE
	}

	/// Bitfield1 MAX: 127
	pub fn set_pad3(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield1 & 0x1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1
		}
	}
}
