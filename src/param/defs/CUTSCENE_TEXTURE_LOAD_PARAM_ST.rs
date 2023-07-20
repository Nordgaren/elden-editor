/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CUTSCENE_TEXTURE_LOAD_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Texture name 00 - テクスチャ名 00
	/// DESC: Texture name 00 - テクスチャ名 00
	pub texName_00:[u8;16],

	/// NAME: Texture name 01 - テクスチャ名 01
	/// DESC: Texture name 01 - テクスチャ名 01
	pub texName_01:[u8;16],

	/// NAME: Texture name 02 - テクスチャ名 02
	/// DESC: Texture name 02 - テクスチャ名 02
	pub texName_02:[u8;16],

	/// NAME: Texture name 03 - テクスチャ名 03
	/// DESC: Texture name 03 - テクスチャ名 03
	pub texName_03:[u8;16],

	/// NAME: Texture name 04 - テクスチャ名 04
	/// DESC: Texture name 04 - テクスチャ名 04
	pub texName_04:[u8;16],

	/// NAME: Texture name 05 - テクスチャ名 05
	/// DESC: Texture name 05 - テクスチャ名 05
	pub texName_05:[u8;16],

	/// NAME: Texture name 06 - テクスチャ名 06
	/// DESC: Texture name 06 - テクスチャ名 06
	pub texName_06:[u8;16],

	/// NAME: Texture name 07 - テクスチャ名 07
	/// DESC: Texture name 07 - テクスチャ名 07
	pub texName_07:[u8;16],

	/// NAME: Texture name 08 - テクスチャ名 08
	/// DESC: Texture name 08 - テクスチャ名 08
	pub texName_08:[u8;16],

	/// NAME: Texture name 09 - テクスチャ名 09
	/// DESC: Texture name 09 - テクスチャ名 09
	pub texName_09:[u8;16],

	/// NAME: Texture name 10 - テクスチャ名 10
	/// DESC: Texture name 10 - テクスチャ名 10
	pub texName_10:[u8;16],

	/// NAME: Texture name 11 - テクスチャ名 11
	/// DESC: Texture name 11 - テクスチャ名 11
	pub texName_11:[u8;16],

	/// NAME: Texture name 12 - テクスチャ名 12
	/// DESC: Texture name 12 - テクスチャ名 12
	pub texName_12:[u8;16],

	/// NAME: Texture name 13 - テクスチャ名 13
	/// DESC: Texture name 13 - テクスチャ名 13
	pub texName_13:[u8;16],

	/// NAME: Texture name 14 - テクスチャ名 14
	/// DESC: Texture name 14 - テクスチャ名 14
	pub texName_14:[u8;16],

	/// NAME: Texture name 15 - テクスチャ名 15
	/// DESC: Texture name 15 - テクスチャ名 15
	pub texName_15:[u8;16],
}

impl Paramdef for CUTSCENE_TEXTURE_LOAD_PARAM_ST {
const NAME: &'static str = "CUTSCENE_TEXTURE_LOAD_PARAM_ST";
const VERSION: u16 = 1;
}
impl CUTSCENE_TEXTURE_LOAD_PARAM_ST {
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
	/// Parameters marked with a circle are excluded from all packages (because they are for debugging). - ○をつけたパラメータは全パッケージから除外します（デバッグ用なので）
	/// Bitfield1
	pub fn get_disableParam_Debug(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_disableParam_Debug(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// Reserve for package output 1 - パッケージ出力用リザーブ1
	/// Bitfield1
	pub fn get_disableParamReserve1(&self) -> u8 {
		&self.Bitfield1 & 0xFC
	}

	/// Bitfield1 MAX: 63
	pub fn set_disableParamReserve1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 2) & 0xFC;
			let newVal = &self.Bitfield1 & 0x3 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x3
		}
	}
}
