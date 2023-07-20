/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MULTI_PLAY_CORRECTION_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: 1 cooperating client Special effect ID - 協力クライアント1名特殊効果ID
	/// DESC: 1 cooperating client Special effect ID - 協力クライアント1名特殊効果ID
	pub client1SpEffectId:i32,

	/// NAME: 2 cooperating clients Special effect ID - 協力クライアント2名特殊効果ID
	/// DESC: 2 cooperating clients Special effect ID - 協力クライアント2名特殊効果ID
	pub client2SpEffectId:i32,

	/// NAME: 3 cooperating clients Special effect ID - 協力クライアント3名特殊効果ID
	/// DESC: 3 cooperating clients Special effect ID - 協力クライアント3名特殊効果ID
	pub client3SpEffectId:i32,

	/// NAME: Whether to overwrite when the number of cooperating people fluctuates - 協力人数変動時に上書きするか
	/// DESC: Whether to overwrite when the number of cooperating people fluctuates - 協力人数変動時に上書きするか
	pub bOverrideSpEffect:u8,

	/// NAME: pad - pad
	/// DESC: pad - pad
	pub pad3:[u8;15],
}

impl Paramdef for MULTI_PLAY_CORRECTION_PARAM_ST {
const NAME: &'static str = "MULTI_PLAY_CORRECTION_PARAM_ST";
const VERSION: u16 = 3;
}
impl MULTI_PLAY_CORRECTION_PARAM_ST {
	/// Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	/// Bitfield1
	pub fn get_disableParam_NT(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_disableParam_NT(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Reserve for package output 1 - パッケージ出力用リザーブ1
	/// Bitfield1
	pub fn get_disableParamReserve1(&self) -> u8 {
		&self.Bitfield1 & 0xFE
	}

	/// Bitfield1 MAX: 127
	pub fn set_disableParamReserve1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield1 & 0x1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1
		}
	}
}
