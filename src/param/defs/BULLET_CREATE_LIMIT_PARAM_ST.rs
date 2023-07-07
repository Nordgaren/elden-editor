/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BULLET_CREATE_LIMIT_PARAM_ST {

	/// NAME: Maximum number of bullets in the group - グループ内上限弾数
	/// DESC: Maximum number of creations in the same group - 同一グループ内での作成上限数
	pub limitNum_byGroup:u8,

	/// NAME: Is it restricted for each owner? - オーナー毎に制限するか
	/// DESC: Is it restricted for each owner? - オーナー毎に制限するか
	pub Bitfield1:u8,

	/// NAME: Padding - パディング
	/// DESC: pad3 - pad3
	pub pad:[u8;30],
}

impl BULLET_CREATE_LIMIT_PARAM_ST {
	/// Is it restricted for each owner? - オーナー毎に制限するか
	/// Bitfield1
	pub fn get_isLimitEachOwner(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_isLimitEachOwner(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// pad2 - pad2
	/// Bitfield1
	pub fn get_pad2(&self) -> u8 {
		&self.Bitfield1 & 0xFE
	}

	/// Bitfield1 MAX: 127
	pub fn set_pad2(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield1 & 0x1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1
		}
	}
}
