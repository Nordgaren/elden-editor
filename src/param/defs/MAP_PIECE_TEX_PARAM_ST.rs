/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MAP_PIECE_TEX_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: R - R
	/// DESC: Color information (R) of the map image before conversion. Pixels with matching RGB values are associated with this parameter - 変換前の地図画像のカラー情報（R）。RGB値が一致したピクセルとこのパラメータが紐づく
	pub srcR:u8,

	/// NAME: G - G
	/// DESC: Color information (G) of the map image before conversion. Pixels with matching RGB values are associated with this parameter - 変換前の地図画像のカラー情報（G）。RGB値が一致したピクセルとこのパラメータが紐づく
	pub srcG:u8,

	/// NAME: B - B
	/// DESC: Color information (B) of the map image before conversion. Pixels with matching RGB values are associated with this parameter - 変換前の地図画像のカラー情報（B）。RGB値が一致したピクセルとこのパラメータが紐づく
	pub srcB:u8,

	/// NAME: pad - パッド
	/// DESC: pad. For the time being, leave it open for "image color information (A)" - パッド。一応「画像色情報（A）」用で空けておく
	pub pad1:[u8;1],

	/// NAME: Map name ID_for saving data display - マップ名ID_セーブデータ表示用
	/// DESC: Map name ID for displaying save data [PlaceName] (0: invalid value) - セーブデータ表示用のマップ名ID[PlaceName](0:無効値)
	pub saveMapNameId:i32,

	/// NAME: Multiplayer area ID - マルチプレイエリアID
	/// DESC: Multiplayer area ID (-1: invalid value) - マルチプレイエリアID(-1:無効値)
	pub multiPlayAreaId:i32,
}

impl MAP_PIECE_TEX_PARAM_ST {
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
