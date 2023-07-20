/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ATTACK_ELEMENT_CORRECT_PARAM_ST {

	/// NAME: Whether to correct muscle strength (physical) - 筋力補正するか（物理）
	pub Bitfield1:u8,

	/// NAME: Do you correct your faith (magic)? - 信仰補正するか（魔法）
	pub Bitfield2:u8,

	/// NAME: Do you correct your workmanship (lightning)? - 技量補正するか（雷）
	pub Bitfield3:u8,

	/// NAME: Do you correct your luck (darkness)? - 運補正するか（闇）
	pub Bitfield4:u8,

	/// NAME: Overwrite muscle strength correction value (physical) - 筋力補正値上書き（物理）
	pub overwriteStrengthCorrectRate_byPhysics:i16,

	/// NAME: Skill correction value overwrite (physical) - 技量補正値上書き（物理）
	pub overwriteDexterityCorrectRate_byPhysics:i16,

	/// NAME: Overwrite the force correction value (physical) - 理力補正値上書き（物理）
	pub overwriteMagicCorrectRate_byPhysics:i16,

	/// NAME: Faith correction value overwrite (physical) - 信仰補正値上書き（物理）
	pub overwriteFaithCorrectRate_byPhysics:i16,

	/// NAME: Overwrite luck correction value (physical) - 運補正値上書き（物理）
	pub overwriteLuckCorrectRate_byPhysics:i16,

	/// NAME: Overwrite muscle strength correction value (magic) - 筋力補正値上書き（魔法）
	pub overwriteStrengthCorrectRate_byMagic:i16,

	/// NAME: Skill correction value overwrite (magic) - 技量補正値上書き（魔法）
	pub overwriteDexterityCorrectRate_byMagic:i16,

	/// NAME: Overwrite the force correction value (magic) - 理力補正値上書き（魔法）
	pub overwriteMagicCorrectRate_byMagic:i16,

	/// NAME: Faith correction value overwrite (magic) - 信仰補正値上書き（魔法）
	pub overwriteFaithCorrectRate_byMagic:i16,

	/// NAME: Overwrite luck correction value (magic) - 運補正値上書き（魔法）
	pub overwriteLuckCorrectRate_byMagic:i16,

	/// NAME: Overwrite muscle strength correction value (flame) - 筋力補正値上書き（炎）
	pub overwriteStrengthCorrectRate_byFire:i16,

	/// NAME: Skill correction value overwrite (flame) - 技量補正値上書き（炎）
	pub overwriteDexterityCorrectRate_byFire:i16,

	/// NAME: Overwrite the force correction value (flame) - 理力補正値上書き（炎）
	pub overwriteMagicCorrectRate_byFire:i16,

	/// NAME: Faith correction value overwrite (flame) - 信仰補正値上書き（炎）
	pub overwriteFaithCorrectRate_byFire:i16,

	/// NAME: Overwrite luck correction value (flame) - 運補正値上書き（炎）
	pub overwriteLuckCorrectRate_byFire:i16,

	/// NAME: Overwrite muscle strength correction value (lightning) - 筋力補正値上書き（雷）
	pub overwriteStrengthCorrectRate_byThunder:i16,

	/// NAME: Skill correction value overwrite (lightning) - 技量補正値上書き（雷）
	pub overwriteDexterityCorrectRate_byThunder:i16,

	/// NAME: Overwrite the force correction value (lightning) - 理力補正値上書き（雷）
	pub overwriteMagicCorrectRate_byThunder:i16,

	/// NAME: Faith correction value overwrite (thunder) - 信仰補正値上書き（雷）
	pub overwriteFaithCorrectRate_byThunder:i16,

	/// NAME: Overwrite luck correction value (lightning) - 運補正値上書き（雷）
	pub overwriteLuckCorrectRate_byThunder:i16,

	/// NAME: Overwrite muscle strength correction value (darkness) - 筋力補正値上書き（闇）
	pub overwriteStrengthCorrectRate_byDark:i16,

	/// NAME: Skill correction value overwrite (darkness) - 技量補正値上書き（闇）
	pub overwriteDexterityCorrectRate_byDark:i16,

	/// NAME: Overwrite the force correction value (darkness) - 理力補正値上書き（闇）
	pub overwriteMagicCorrectRate_byDark:i16,

	/// NAME: Faith correction value overwrite (darkness) - 信仰補正値上書き（闇）
	pub overwriteFaithCorrectRate_byDark:i16,

	/// NAME: Overwrite luck correction value (darkness) - 運補正値上書き（闇）
	pub overwriteLuckCorrectRate_byDark:i16,

	/// NAME: Strength correction value Impact rate (physical) - 筋力補正値影響率（物理）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceStrengthCorrectRate_byPhysics:i16,

	/// NAME: Skill correction value Impact rate (physical) - 技量補正値影響率（物理）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceDexterityCorrectRate_byPhysics:i16,

	/// NAME: Force correction value Impact rate (physical) - 理力補正値影響率（物理）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceMagicCorrectRate_byPhysics:i16,

	/// NAME: Faith correction value Impact rate (physical) - 信仰補正値影響率（物理）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceFaithCorrectRate_byPhysics:i16,

	/// NAME: Luck correction value influence rate (physical) - 運補正値影響率（物理）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceLuckCorrectRate_byPhysics:i16,

	/// NAME: Strength correction value Impact rate (magic) - 筋力補正値影響率（魔法）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceStrengthCorrectRate_byMagic:i16,

	/// NAME: Skill correction value Impact rate (magic) - 技量補正値影響率（魔法）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceDexterityCorrectRate_byMagic:i16,

	/// NAME: Force correction value Impact rate (magic) - 理力補正値影響率（魔法）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceMagicCorrectRate_byMagic:i16,

	/// NAME: Faith correction value Impact rate (magic) - 信仰補正値影響率（魔法）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceFaithCorrectRate_byMagic:i16,

	/// NAME: Luck correction value influence rate (magic) - 運補正値影響率（魔法）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceLuckCorrectRate_byMagic:i16,

	/// NAME: Strength correction value Impact rate (flame) - 筋力補正値影響率（炎）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceStrengthCorrectRate_byFire:i16,

	/// NAME: Skill correction value Impact rate (flame) - 技量補正値影響率（炎）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceDexterityCorrectRate_byFire:i16,

	/// NAME: Force correction value Impact rate (flame) - 理力補正値影響率（炎）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceMagicCorrectRate_byFire:i16,

	/// NAME: Faith correction value Impact rate (flame) - 信仰補正値影響率（炎）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceFaithCorrectRate_byFire:i16,

	/// NAME: Luck correction value influence rate (flame) - 運補正値影響率（炎）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceLuckCorrectRate_byFire:i16,

	/// NAME: Strength correction value Impact rate (lightning) - 筋力補正値影響率（雷）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceStrengthCorrectRate_byThunder:i16,

	/// NAME: Skill correction value Impact rate (lightning) - 技量補正値影響率（雷）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceDexterityCorrectRate_byThunder:i16,

	/// NAME: Force correction value Impact rate (lightning) - 理力補正値影響率（雷）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceMagicCorrectRate_byThunder:i16,

	/// NAME: Faith correction value Impact rate (lightning) - 信仰補正値影響率（雷）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceFaithCorrectRate_byThunder:i16,

	/// NAME: Luck correction value influence rate (lightning) - 運補正値影響率（雷）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceLuckCorrectRate_byThunder:i16,

	/// NAME: Strength correction value influence rate (darkness) - 筋力補正値影響率（闇）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceStrengthCorrectRate_byDark:i16,

	/// NAME: Skill correction value Impact rate (darkness) - 技量補正値影響率（闇）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceDexterityCorrectRate_byDark:i16,

	/// NAME: Force correction value Impact rate (darkness) - 理力補正値影響率（闇）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceMagicCorrectRate_byDark:i16,

	/// NAME: Faith correction value Impact rate (darkness) - 信仰補正値影響率（闇）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceFaithCorrectRate_byDark:i16,

	/// NAME: Luck correction value influence rate (darkness) - 運補正値影響率（闇）
	/// DESC: The rate of influence of the correction factor. - 補正率の影響割合。
	pub InfluenceLuckCorrectRate_byDark:i16,

	/// NAME: Padding - パディング
	pub pad2:[u8;24],
}

impl Paramdef for ATTACK_ELEMENT_CORRECT_PARAM_ST {
const NAME: &'static str = "ATTACK_ELEMENT_CORRECT_PARAM_ST";
const VERSION: u16 = 2;
}
impl ATTACK_ELEMENT_CORRECT_PARAM_ST {
	/// 
	/// Bitfield1
	pub fn get_isStrengthCorrect_byPhysics(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_isStrengthCorrect_byPhysics(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isDexterityCorrect_byPhysics(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_isDexterityCorrect_byPhysics(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isMagicCorrect_byPhysics(&self) -> bool {
		&self.Bitfield1 & 0x4 != 0
	}

	/// Bitfield1
	pub fn set_isMagicCorrect_byPhysics(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x4
		} else {
			self.Bitfield1 &= 0xFB
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isFaithCorrect_byPhysics(&self) -> bool {
		&self.Bitfield1 & 0x8 != 0
	}

	/// Bitfield1
	pub fn set_isFaithCorrect_byPhysics(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x8
		} else {
			self.Bitfield1 &= 0xF7
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isLuckCorrect_byPhysics(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_isLuckCorrect_byPhysics(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xEF
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isStrengthCorrect_byMagic(&self) -> bool {
		&self.Bitfield1 & 0x20 != 0
	}

	/// Bitfield1
	pub fn set_isStrengthCorrect_byMagic(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x20
		} else {
			self.Bitfield1 &= 0xDF
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isDexterityCorrect_byMagic(&self) -> bool {
		&self.Bitfield1 & 0x40 != 0
	}

	/// Bitfield1
	pub fn set_isDexterityCorrect_byMagic(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40
		} else {
			self.Bitfield1 &= 0xBF
		}
	}
	/// 
	/// Bitfield1
	pub fn get_isMagicCorrect_byMagic(&self) -> bool {
		&self.Bitfield1 & 0x80 != 0
	}

	/// Bitfield1
	pub fn set_isMagicCorrect_byMagic(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80
		} else {
			self.Bitfield1 &= 0x7F
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isFaithCorrect_byMagic(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isFaithCorrect_byMagic(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isLuckCorrect_byMagic(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_isLuckCorrect_byMagic(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isStrengthCorrect_byFire(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_isStrengthCorrect_byFire(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isDexterityCorrect_byFire(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_isDexterityCorrect_byFire(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isMagicCorrect_byFire(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_isMagicCorrect_byFire(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isFaithCorrect_byFire(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_isFaithCorrect_byFire(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isLuckCorrect_byFire(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_isLuckCorrect_byFire(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// 
	/// Bitfield2
	pub fn get_isStrengthCorrect_byThunder(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_isStrengthCorrect_byThunder(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isDexterityCorrect_byThunder(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_isDexterityCorrect_byThunder(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isMagicCorrect_byThunder(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_isMagicCorrect_byThunder(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isFaithCorrect_byThunder(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_isFaithCorrect_byThunder(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isLuckCorrect_byThunder(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_isLuckCorrect_byThunder(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isStrengthCorrect_byDark(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_isStrengthCorrect_byDark(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isDexterityCorrect_byDark(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_isDexterityCorrect_byDark(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isMagicCorrect_byDark(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_isMagicCorrect_byDark(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// 
	/// Bitfield3
	pub fn get_isFaithCorrect_byDark(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_isFaithCorrect_byDark(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// 
	/// Bitfield4
	pub fn get_isLuckCorrect_byDark(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_isLuckCorrect_byDark(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// 
	/// Bitfield4
	pub fn get_pad1(&self) -> u8 {
		&self.Bitfield4 & 0xFE
	}

	/// Bitfield4 MAX: 127
	pub fn set_pad1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield4 & 0x1 | val;
			self.Bitfield4 = newVal
		} else {
			self.Bitfield4 &= 0x1
		}
	}
}
