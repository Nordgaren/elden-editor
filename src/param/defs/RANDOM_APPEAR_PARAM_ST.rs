/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct RANDOM_APPEAR_PARAM_ST {

	/// NAME: Lottery slot ID 0 - 抽選スロットID0
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield1:u8,

	/// NAME: Lottery slot ID 8 - 抽選スロットID8
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield2:u8,

	/// NAME: Lottery slot ID 16 - 抽選スロットID16
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield3:u8,

	/// NAME: Lottery slot ID 24 - 抽選スロットID24
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield4:u8,

	/// NAME: Lottery slot ID 32 - 抽選スロットID32
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield5:u8,

	/// NAME: Lottery slot ID 40 - 抽選スロットID40
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield6:u8,

	/// NAME: Lottery slot ID 48 - 抽選スロットID48
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield7:u8,

	/// NAME: Lottery slot ID 56 - 抽選スロットID56
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield8:u8,

	/// NAME: Lottery slot ID 64 - 抽選スロットID64
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield9:u8,

	/// NAME: Lottery slot ID 72 - 抽選スロットID72
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield10:u8,

	/// NAME: Lottery slot ID 80 - 抽選スロットID80
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield11:u8,

	/// NAME: Lottery slot ID 88 - 抽選スロットID88
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield12:u8,

	/// NAME: Lottery slot ID 96 - 抽選スロットID96
	/// DESC: Will it appear if the slot is won? - スロットが当選した場合に出現するか
	pub Bitfield13:u8,
}

impl RANDOM_APPEAR_PARAM_ST {
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot0(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_slot0(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot1(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_slot1(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot2(&self) -> bool {
		&self.Bitfield1 & 0x4 != 0
	}

	/// Bitfield1
	pub fn set_slot2(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x4
		} else {
			self.Bitfield1 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot3(&self) -> bool {
		&self.Bitfield1 & 0x8 != 0
	}

	/// Bitfield1
	pub fn set_slot3(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x8
		} else {
			self.Bitfield1 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot4(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_slot4(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot5(&self) -> bool {
		&self.Bitfield1 & 0x20 != 0
	}

	/// Bitfield1
	pub fn set_slot5(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x20
		} else {
			self.Bitfield1 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot6(&self) -> bool {
		&self.Bitfield1 & 0x40 != 0
	}

	/// Bitfield1
	pub fn set_slot6(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40
		} else {
			self.Bitfield1 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield1
	pub fn get_slot7(&self) -> bool {
		&self.Bitfield1 & 0x80 != 0
	}

	/// Bitfield1
	pub fn set_slot7(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80
		} else {
			self.Bitfield1 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot8(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_slot8(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot9(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_slot9(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot10(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_slot10(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot11(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_slot11(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot12(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_slot12(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot13(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_slot13(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot14(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_slot14(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield2
	pub fn get_slot15(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_slot15(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot16(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_slot16(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot17(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_slot17(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot18(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_slot18(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot19(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_slot19(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot20(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_slot20(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot21(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_slot21(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot22(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_slot22(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield3
	pub fn get_slot23(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_slot23(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot24(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_slot24(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot25(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_slot25(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot26(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_slot26(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot27(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_slot27(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot28(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_slot28(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot29(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_slot29(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot30(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_slot30(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield4
	pub fn get_slot31(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_slot31(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot32(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_slot32(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot33(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_slot33(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot34(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_slot34(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot35(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_slot35(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot36(&self) -> bool {
		&self.Bitfield5 & 0x10 != 0
	}

	/// Bitfield5
	pub fn set_slot36(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x10
		} else {
			self.Bitfield5 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot37(&self) -> bool {
		&self.Bitfield5 & 0x20 != 0
	}

	/// Bitfield5
	pub fn set_slot37(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x20
		} else {
			self.Bitfield5 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot38(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_slot38(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield5
	pub fn get_slot39(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_slot39(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot40(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_slot40(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot41(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_slot41(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot42(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_slot42(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot43(&self) -> bool {
		&self.Bitfield6 & 0x8 != 0
	}

	/// Bitfield6
	pub fn set_slot43(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x8
		} else {
			self.Bitfield6 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot44(&self) -> bool {
		&self.Bitfield6 & 0x10 != 0
	}

	/// Bitfield6
	pub fn set_slot44(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x10
		} else {
			self.Bitfield6 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot45(&self) -> bool {
		&self.Bitfield6 & 0x20 != 0
	}

	/// Bitfield6
	pub fn set_slot45(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x20
		} else {
			self.Bitfield6 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot46(&self) -> bool {
		&self.Bitfield6 & 0x40 != 0
	}

	/// Bitfield6
	pub fn set_slot46(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x40
		} else {
			self.Bitfield6 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield6
	pub fn get_slot47(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_slot47(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot48(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_slot48(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot49(&self) -> bool {
		&self.Bitfield7 & 0x2 != 0
	}

	/// Bitfield7
	pub fn set_slot49(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x2
		} else {
			self.Bitfield7 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot50(&self) -> bool {
		&self.Bitfield7 & 0x4 != 0
	}

	/// Bitfield7
	pub fn set_slot50(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x4
		} else {
			self.Bitfield7 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot51(&self) -> bool {
		&self.Bitfield7 & 0x8 != 0
	}

	/// Bitfield7
	pub fn set_slot51(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x8
		} else {
			self.Bitfield7 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot52(&self) -> bool {
		&self.Bitfield7 & 0x10 != 0
	}

	/// Bitfield7
	pub fn set_slot52(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x10
		} else {
			self.Bitfield7 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot53(&self) -> bool {
		&self.Bitfield7 & 0x20 != 0
	}

	/// Bitfield7
	pub fn set_slot53(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x20
		} else {
			self.Bitfield7 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot54(&self) -> bool {
		&self.Bitfield7 & 0x40 != 0
	}

	/// Bitfield7
	pub fn set_slot54(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x40
		} else {
			self.Bitfield7 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield7
	pub fn get_slot55(&self) -> bool {
		&self.Bitfield7 & 0x80 != 0
	}

	/// Bitfield7
	pub fn set_slot55(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x80
		} else {
			self.Bitfield7 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot56(&self) -> bool {
		&self.Bitfield8 & 0x1 != 0
	}

	/// Bitfield8
	pub fn set_slot56(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x1
		} else {
			self.Bitfield8 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot57(&self) -> bool {
		&self.Bitfield8 & 0x2 != 0
	}

	/// Bitfield8
	pub fn set_slot57(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x2
		} else {
			self.Bitfield8 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot58(&self) -> bool {
		&self.Bitfield8 & 0x4 != 0
	}

	/// Bitfield8
	pub fn set_slot58(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x4
		} else {
			self.Bitfield8 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot59(&self) -> bool {
		&self.Bitfield8 & 0x8 != 0
	}

	/// Bitfield8
	pub fn set_slot59(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x8
		} else {
			self.Bitfield8 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot60(&self) -> bool {
		&self.Bitfield8 & 0x10 != 0
	}

	/// Bitfield8
	pub fn set_slot60(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x10
		} else {
			self.Bitfield8 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot61(&self) -> bool {
		&self.Bitfield8 & 0x20 != 0
	}

	/// Bitfield8
	pub fn set_slot61(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x20
		} else {
			self.Bitfield8 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot62(&self) -> bool {
		&self.Bitfield8 & 0x40 != 0
	}

	/// Bitfield8
	pub fn set_slot62(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x40
		} else {
			self.Bitfield8 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield8
	pub fn get_slot63(&self) -> bool {
		&self.Bitfield8 & 0x80 != 0
	}

	/// Bitfield8
	pub fn set_slot63(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x80
		} else {
			self.Bitfield8 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot64(&self) -> bool {
		&self.Bitfield9 & 0x1 != 0
	}

	/// Bitfield9
	pub fn set_slot64(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x1
		} else {
			self.Bitfield9 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot65(&self) -> bool {
		&self.Bitfield9 & 0x2 != 0
	}

	/// Bitfield9
	pub fn set_slot65(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x2
		} else {
			self.Bitfield9 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot66(&self) -> bool {
		&self.Bitfield9 & 0x4 != 0
	}

	/// Bitfield9
	pub fn set_slot66(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x4
		} else {
			self.Bitfield9 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot67(&self) -> bool {
		&self.Bitfield9 & 0x8 != 0
	}

	/// Bitfield9
	pub fn set_slot67(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x8
		} else {
			self.Bitfield9 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot68(&self) -> bool {
		&self.Bitfield9 & 0x10 != 0
	}

	/// Bitfield9
	pub fn set_slot68(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x10
		} else {
			self.Bitfield9 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot69(&self) -> bool {
		&self.Bitfield9 & 0x20 != 0
	}

	/// Bitfield9
	pub fn set_slot69(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x20
		} else {
			self.Bitfield9 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot70(&self) -> bool {
		&self.Bitfield9 & 0x40 != 0
	}

	/// Bitfield9
	pub fn set_slot70(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x40
		} else {
			self.Bitfield9 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield9
	pub fn get_slot71(&self) -> bool {
		&self.Bitfield9 & 0x80 != 0
	}

	/// Bitfield9
	pub fn set_slot71(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x80
		} else {
			self.Bitfield9 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot72(&self) -> bool {
		&self.Bitfield10 & 0x1 != 0
	}

	/// Bitfield10
	pub fn set_slot72(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x1
		} else {
			self.Bitfield10 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot73(&self) -> bool {
		&self.Bitfield10 & 0x2 != 0
	}

	/// Bitfield10
	pub fn set_slot73(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x2
		} else {
			self.Bitfield10 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot74(&self) -> bool {
		&self.Bitfield10 & 0x4 != 0
	}

	/// Bitfield10
	pub fn set_slot74(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x4
		} else {
			self.Bitfield10 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot75(&self) -> bool {
		&self.Bitfield10 & 0x8 != 0
	}

	/// Bitfield10
	pub fn set_slot75(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x8
		} else {
			self.Bitfield10 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot76(&self) -> bool {
		&self.Bitfield10 & 0x10 != 0
	}

	/// Bitfield10
	pub fn set_slot76(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x10
		} else {
			self.Bitfield10 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot77(&self) -> bool {
		&self.Bitfield10 & 0x20 != 0
	}

	/// Bitfield10
	pub fn set_slot77(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x20
		} else {
			self.Bitfield10 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot78(&self) -> bool {
		&self.Bitfield10 & 0x40 != 0
	}

	/// Bitfield10
	pub fn set_slot78(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x40
		} else {
			self.Bitfield10 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield10
	pub fn get_slot79(&self) -> bool {
		&self.Bitfield10 & 0x80 != 0
	}

	/// Bitfield10
	pub fn set_slot79(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x80
		} else {
			self.Bitfield10 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot80(&self) -> bool {
		&self.Bitfield11 & 0x1 != 0
	}

	/// Bitfield11
	pub fn set_slot80(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x1
		} else {
			self.Bitfield11 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot81(&self) -> bool {
		&self.Bitfield11 & 0x2 != 0
	}

	/// Bitfield11
	pub fn set_slot81(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x2
		} else {
			self.Bitfield11 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot82(&self) -> bool {
		&self.Bitfield11 & 0x4 != 0
	}

	/// Bitfield11
	pub fn set_slot82(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x4
		} else {
			self.Bitfield11 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot83(&self) -> bool {
		&self.Bitfield11 & 0x8 != 0
	}

	/// Bitfield11
	pub fn set_slot83(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x8
		} else {
			self.Bitfield11 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot84(&self) -> bool {
		&self.Bitfield11 & 0x10 != 0
	}

	/// Bitfield11
	pub fn set_slot84(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x10
		} else {
			self.Bitfield11 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot85(&self) -> bool {
		&self.Bitfield11 & 0x20 != 0
	}

	/// Bitfield11
	pub fn set_slot85(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x20
		} else {
			self.Bitfield11 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot86(&self) -> bool {
		&self.Bitfield11 & 0x40 != 0
	}

	/// Bitfield11
	pub fn set_slot86(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x40
		} else {
			self.Bitfield11 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield11
	pub fn get_slot87(&self) -> bool {
		&self.Bitfield11 & 0x80 != 0
	}

	/// Bitfield11
	pub fn set_slot87(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x80
		} else {
			self.Bitfield11 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot88(&self) -> bool {
		&self.Bitfield12 & 0x1 != 0
	}

	/// Bitfield12
	pub fn set_slot88(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x1
		} else {
			self.Bitfield12 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot89(&self) -> bool {
		&self.Bitfield12 & 0x2 != 0
	}

	/// Bitfield12
	pub fn set_slot89(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x2
		} else {
			self.Bitfield12 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot90(&self) -> bool {
		&self.Bitfield12 & 0x4 != 0
	}

	/// Bitfield12
	pub fn set_slot90(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x4
		} else {
			self.Bitfield12 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot91(&self) -> bool {
		&self.Bitfield12 & 0x8 != 0
	}

	/// Bitfield12
	pub fn set_slot91(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x8
		} else {
			self.Bitfield12 &= 0xF7
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot92(&self) -> bool {
		&self.Bitfield12 & 0x10 != 0
	}

	/// Bitfield12
	pub fn set_slot92(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x10
		} else {
			self.Bitfield12 &= 0xEF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot93(&self) -> bool {
		&self.Bitfield12 & 0x20 != 0
	}

	/// Bitfield12
	pub fn set_slot93(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x20
		} else {
			self.Bitfield12 &= 0xDF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot94(&self) -> bool {
		&self.Bitfield12 & 0x40 != 0
	}

	/// Bitfield12
	pub fn set_slot94(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x40
		} else {
			self.Bitfield12 &= 0xBF
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield12
	pub fn get_slot95(&self) -> bool {
		&self.Bitfield12 & 0x80 != 0
	}

	/// Bitfield12
	pub fn set_slot95(&mut self, state: bool) {
		if state {
			self.Bitfield12 |= 0x80
		} else {
			self.Bitfield12 &= 0x7F
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield13
	pub fn get_slot96(&self) -> bool {
		&self.Bitfield13 & 0x1 != 0
	}

	/// Bitfield13
	pub fn set_slot96(&mut self, state: bool) {
		if state {
			self.Bitfield13 |= 0x1
		} else {
			self.Bitfield13 &= 0xFE
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield13
	pub fn get_slot97(&self) -> bool {
		&self.Bitfield13 & 0x2 != 0
	}

	/// Bitfield13
	pub fn set_slot97(&mut self, state: bool) {
		if state {
			self.Bitfield13 |= 0x2
		} else {
			self.Bitfield13 &= 0xFD
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield13
	pub fn get_slot98(&self) -> bool {
		&self.Bitfield13 & 0x4 != 0
	}

	/// Bitfield13
	pub fn set_slot98(&mut self, state: bool) {
		if state {
			self.Bitfield13 |= 0x4
		} else {
			self.Bitfield13 &= 0xFB
		}
	}
	/// Will it appear if the slot is won? - スロットが当選した場合に出現するか
	/// Bitfield13
	pub fn get_slot99(&self) -> bool {
		&self.Bitfield13 & 0x8 != 0
	}

	/// Bitfield13
	pub fn set_slot99(&mut self, state: bool) {
		if state {
			self.Bitfield13 |= 0x8
		} else {
			self.Bitfield13 &= 0xF7
		}
	}
	/// 
	/// Bitfield13
	pub fn get_pad(&self) -> u8 {
		&self.Bitfield13 & 0xF0
	}

	/// Bitfield13 MAX: 15
	pub fn set_pad(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 4) & 0xF0;
			let newVal = &self.Bitfield13 & 0xF | val;
			self.Bitfield13 = newVal
		} else {
			self.Bitfield13 &= 0xF
		}
	}
}
