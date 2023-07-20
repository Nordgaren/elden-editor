/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WORLD_MAP_LEGACY_CONV_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Conversion source map ID: Area number - 変換元マップID：エリア番号
	/// DESC: Conversion source map ID: Area number - 変換元マップID：エリア番号
	pub srcAreaNo:u8,

	/// NAME: Source map ID: Grid X - 変換元マップID：グリッドX
	/// DESC: Source map ID: Grid X - 変換元マップID：グリッドX
	pub srcGridXNo:u8,

	/// NAME: Source map ID: Grid Z - 変換元マップID：グリッドZ
	/// DESC: Source map ID: Grid Z - 変換元マップID：グリッドZ
	pub srcGridZNo:u8,

	/// NAME: Padding 1 - パディング１
	/// DESC: Padding 1 - パディング１
	pub pad1:[u8;1],

	/// NAME: Source map reference coordinates X - 変換元マップ基準座標X
	/// DESC: Source map reference coordinates X - 変換元マップ基準座標X
	pub srcPosX:f32,

	/// NAME: Source map reference coordinates Y - 変換元マップ基準座標Y
	/// DESC: Source map reference coordinates Y - 変換元マップ基準座標Y
	pub srcPosY:f32,

	/// NAME: Source map reference coordinates Z - 変換元マップ基準座標Z
	/// DESC: Source map reference coordinates Z - 変換元マップ基準座標Z
	pub srcPosZ:f32,

	/// NAME: Conversion destination map ID: Area number - 変換先マップID：エリア番号
	/// DESC: Conversion destination map ID: Area number - 変換先マップID：エリア番号
	pub dstAreaNo:u8,

	/// NAME: Destination map ID: Grid X - 変換先マップID：グリッドX
	/// DESC: Destination map ID: Grid X - 変換先マップID：グリッドX
	pub dstGridXNo:u8,

	/// NAME: Destination map ID: Grid Z - 変換先マップID：グリッドZ
	/// DESC: Destination map ID: Grid Z - 変換先マップID：グリッドZ
	pub dstGridZNo:u8,

	/// NAME: Padding 2 - パディング２
	/// DESC: Padding 2 - パディング２
	pub pad2:[u8;1],

	/// NAME: Destination map reference coordinates X - 変換先マップ基準座標X
	/// DESC: Destination map reference coordinates X - 変換先マップ基準座標X
	pub dstPosX:f32,

	/// NAME: Destination map reference coordinates Y - 変換先マップ基準座標Y
	/// DESC: Destination map reference coordinates Y - 変換先マップ基準座標Y
	pub dstPosY:f32,

	/// NAME: Destination map reference coordinates Z - 変換先マップ基準座標Z
	/// DESC: Destination map reference coordinates Z - 変換先マップ基準座標Z
	pub dstPosZ:f32,

	/// NAME: Is it a reference connection point? - 基準となる接続点か
	/// DESC: Is it a reference connection point? One reference connection point is always set for one conversion source map ID. - 基準となる接続点か。１つの変換元マップIDには必ず一つは基準となる接続点が設定される
	pub Bitfield2:u8,

	/// NAME: Padding 4 - パディング４
	/// DESC: Padding 4 - パディング４
	pub pad4:[u8;11],
}

impl Paramdef for WORLD_MAP_LEGACY_CONV_PARAM_ST {
const NAME: &'static str = "WORLD_MAP_LEGACY_CONV_PARAM_ST";
const VERSION: u16 = 2;
}
impl WORLD_MAP_LEGACY_CONV_PARAM_ST {
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
	}	/// Is it a reference connection point? One reference connection point is always set for one conversion source map ID. - 基準となる接続点か。１つの変換元マップIDには必ず一つは基準となる接続点が設定される
	/// Bitfield2
	pub fn get_isBasePoint(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isBasePoint(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Padding 3 - パディング３
	/// Bitfield2
	pub fn get_pad3(&self) -> u8 {
		&self.Bitfield2 & 0xFE
	}

	/// Bitfield2 MAX: 127
	pub fn set_pad3(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield2 & 0x1 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0x1
		}
	}
}
