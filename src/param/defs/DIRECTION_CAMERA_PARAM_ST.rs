/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct DIRECTION_CAMERA_PARAM_ST {

	/// NAME: Affected by options - オプションの影響を受けるか
	/// DESC: Is it affected by the production camera ON / OFF option? - 演出カメラON/OFFオプションの影響を受けるか？
	pub Bitfield1:u8,

	/// NAME: pad - パッド
	/// DESC: pad - pad
	pub pad1:[u8;15],
}

impl DIRECTION_CAMERA_PARAM_ST {
	/// Is it affected by the production camera ON / OFF option? - 演出カメラON/OFFオプションの影響を受けるか？
	/// Bitfield1
	pub fn get_isUseOption(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_isUseOption(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// pad - pad
	/// Bitfield1
	pub fn get_pad2(&self) -> u8 {
		&self.Bitfield1 & 0xE
	}

	/// Bitfield1 MAX: 7
	pub fn set_pad2(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xE;
			let newVal = &self.Bitfield1 & 0xF1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xF1
		}
	}
}
