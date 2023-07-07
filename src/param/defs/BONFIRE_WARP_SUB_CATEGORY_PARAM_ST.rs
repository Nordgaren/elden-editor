/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BONFIRE_WARP_SUB_CATEGORY_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Text ID - テキストID
	/// DESC: Subcategory display name Text ID [MenuText] - サブカテゴリの表示名テキストID[MenuText]
	pub textId:i32,

	/// NAME: Kagaribi Warp Tab ID - 篝火ワープタブID
	/// DESC: Kagaribi Warp Tab ID. ID of the tab to which this subcategory belongs - 篝火ワープタブID。このサブカテゴリが所属するタブのID
	pub tabId:u16,

	/// NAME: Bonfire Warp Tab Sort ID - 篝火ワープタブソートID
	/// DESC: Bonfire Warp Tab Sort ID. Display order of subcategories in tabs - 篝火ワープタブソートID。タブの中サブカテゴリの表示順
	pub sortId:u16,

	/// NAME: pad - pad
	pub pad:[u8;4],
}

impl BONFIRE_WARP_SUB_CATEGORY_PARAM_ST {
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
