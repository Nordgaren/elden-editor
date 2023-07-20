/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MAP_GD_REGION_DRAW_PARAM {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Local light scale overwrite value at the time of IV shooting - IV撮影時のローカルライトスケール上書き値
	/// DESC: Local light scale value at the time of IV shooting (0 or less: no overwrite) [GR] SEQ13338 [Irradiance volume] I want to change the indirect scale of the SFX light source uniformly. - IV撮影時のローカルライトスケール値(0以下：上書きなし) 【GR】SEQ13338 【イラディアンスボリューム】SFX光源のインダイレクトスケールを一律に変更したい
	pub overrideIVLocalLightScale:f32,
}

impl Paramdef for MAP_GD_REGION_DRAW_PARAM {
const NAME: &'static str = "MAP_GD_REGION_DRAW_PARAM";
const VERSION: u16 = 2;
}
impl MAP_GD_REGION_DRAW_PARAM {
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
