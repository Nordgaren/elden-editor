/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GPARAM_REF_SETTINGS_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Referenced map ID - 参照先マップID 
	/// DESC: Specify the referenced map number. Legacy: m10_10_00_00-> 10010000, Open: m60_ ?? _ ?? _ ??-> m60000000 - 参照先のマップ番号を指定します。レガシー：m10_10_00_00 -> 10010000, オープン：m60_??_??_?? -> m60000000
	pub RefTargetMapId:i32,

	/// NAME: Reserve - リザーブ
	/// DESC: Reserve - リザーブ
	pub Reserve:[u8;24],
}

impl GPARAM_REF_SETTINGS_PARAM_ST {
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
