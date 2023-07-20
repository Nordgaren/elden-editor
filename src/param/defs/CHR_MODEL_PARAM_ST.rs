/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CHR_MODEL_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Model used memory type - モデルの使用メモリタイプ
	/// DESC: Model used memory type - モデルの使用メモリタイプ
	pub modelMemoryType:u8,

	/// NAME: Texture usage memory type - テクスチャの使用メモリタイプ
	/// DESC: Texture usage memory type - テクスチャの使用メモリタイプ
	pub texMemoryType:u8,

	/// NAME: Camera fade parameter ID - カメラフェードパラメータID
	/// DESC: Camera fade parameter ID (-1: refer to material, 0: do not disappear, 1 ~: parameter ID) - カメラフェードパラメータID（-1：マテリアルを参照、0：消えない、1～：パラメータID）
	pub cameraDitherFadeId:i16,

	/// NAME: Report animation capacity (MB) - 報告アニメ容量(MB)
	/// DESC: If this value is exceeded, a report will be sent by the reporting system. - この値を超えると報告システムで報告が来る。
	pub reportAnimMemSizeMb:f32,

	/// NAME: Unk
	/// DESC: Unk (Added in 1.06)
	pub unk:u32,
}

impl Paramdef for CHR_MODEL_PARAM_ST {
const NAME: &'static str = "CHR_MODEL_PARAM_ST";
const VERSION: u16 = 2;
}
impl CHR_MODEL_PARAM_ST {
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
