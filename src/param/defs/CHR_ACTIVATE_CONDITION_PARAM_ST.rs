/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CHR_ACTIVATE_CONDITION_PARAM_ST {

	/// NAME: Appearance condition _ sunny - 出現条件_晴れ
	/// DESC: Will it appear when the weather is "sunny"? - 天候が「晴れ」のときに出現するか
	pub Bitfield1:u8,

	/// NAME: Appearance condition_Snow - 出現条件_雪
	/// DESC: Will it appear when the weather is "snow"? - 天候が「雪」のときに出現するか
	pub Bitfield2:u8,

	/// NAME: Appearance start in game time_hour - 出現開始インゲーム時間_時
	/// DESC: Appearance start in game time_hour - 出現開始インゲーム時間_時
	pub timeStartHour:u8,

	/// NAME: Appearance start in-game time_minutes - 出現開始インゲーム時間_分
	/// DESC: Appearance start in-game time_minutes - 出現開始インゲーム時間_分
	pub timeStartMin:u8,

	/// NAME: Appearance end in game time_hour - 出現終了インゲーム時間_時
	/// DESC: Appearance end in game time_hour - 出現終了インゲーム時間_時
	pub timeEndHour:u8,

	/// NAME: End of appearance In-game time_minutes - 出現終了インゲーム時間_分
	/// DESC: End of appearance In-game time_minutes - 出現終了インゲーム時間_分
	pub timeEndMin:u8,

	/// NAME: pad - pad
	pub pad2:[u8;2],
}

impl CHR_ACTIVATE_CONDITION_PARAM_ST {
	/// Will it appear when the weather is "sunny"? - 天候が「晴れ」のときに出現するか
	/// Bitfield1
	pub fn get_weatherSunny(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_weatherSunny(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Will it appear when the weather is "clear"? - 天候が「快晴」のときに出現するか
	/// Bitfield1
	pub fn get_weatherClearSky(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_weatherClearSky(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// Will it appear when the weather is "lightly cloudy"? - 天候が「薄曇り」のときに出現するか
	/// Bitfield1
	pub fn get_weatherWeakCloudy(&self) -> bool {
		&self.Bitfield1 & 0x4 != 0
	}

	/// Bitfield1
	pub fn set_weatherWeakCloudy(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x4
		} else {
			self.Bitfield1 &= 0xFB
		}
	}
	/// Will it appear when the weather is "cloudy"? - 天候が「曇り」のときに出現するか
	/// Bitfield1
	pub fn get_weatherCloudy(&self) -> bool {
		&self.Bitfield1 & 0x8 != 0
	}

	/// Bitfield1
	pub fn set_weatherCloudy(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x8
		} else {
			self.Bitfield1 &= 0xF7
		}
	}
	/// Will it appear when the weather is "rainy"? - 天候が「雨」のときに出現するか
	/// Bitfield1
	pub fn get_weatherRain(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_weatherRain(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xEF
		}
	}
	/// Will it appear when the weather is "heavy rain"? - 天候が「豪雨」のときに出現するか
	/// Bitfield1
	pub fn get_weatherHeavyRain(&self) -> bool {
		&self.Bitfield1 & 0x20 != 0
	}

	/// Bitfield1
	pub fn set_weatherHeavyRain(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x20
		} else {
			self.Bitfield1 &= 0xDF
		}
	}
	/// Will it appear when the weather is "stormy"? - 天候が「嵐」のときに出現するか
	/// Bitfield1
	pub fn get_weatherStorm(&self) -> bool {
		&self.Bitfield1 & 0x40 != 0
	}

	/// Bitfield1
	pub fn set_weatherStorm(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40
		} else {
			self.Bitfield1 &= 0xBF
		}
	}
	/// Will it appear when the weather is "storm (for combat with the descendants of the guardian)"? - 天候が「嵐（守護者の末裔との戦闘用）」のときに出現するか
	/// Bitfield1
	pub fn get_weatherStormForBattle(&self) -> bool {
		&self.Bitfield1 & 0x80 != 0
	}

	/// Bitfield1
	pub fn set_weatherStormForBattle(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80
		} else {
			self.Bitfield1 &= 0x7F
		}
	}
	/// Will it appear when the weather is "snow"? - 天候が「雪」のときに出現するか
	/// Bitfield2
	pub fn get_weatherSnow(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_weatherSnow(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Will it appear when the weather is "heavy snow"? - 天候が「大雪」のときに出現するか
	/// Bitfield2
	pub fn get_weatherHeavySnow(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_weatherHeavySnow(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Will it appear when the weather is "fog"? - 天候が「霧」のときに出現するか
	/// Bitfield2
	pub fn get_weatherFog(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_weatherFog(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Will it appear when the weather is "dense fog"? - 天候が「濃霧」のときに出現するか
	/// Bitfield2
	pub fn get_weatherHeavyFog(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_weatherHeavyFog(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Does it appear when the weather is "dense fog (rain)"? - 天候が「濃霧（雨）」のときに出現するか
	/// Bitfield2
	pub fn get_weatherHeavyFogRain(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_weatherHeavyFogRain(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Will it appear when the weather is a "sandstorm"? - 天候が「砂嵐」のときに出現するか
	/// Bitfield2
	pub fn get_weatherSandStorm(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_weatherSandStorm(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// 
	/// Bitfield2
	pub fn get_pad1(&self) -> u8 {
		&self.Bitfield2 & 0xC0
	}

	/// Bitfield2 MAX: 3
	pub fn set_pad1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 6) & 0xC0;
			let newVal = &self.Bitfield2 & 0x3F | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0x3F
		}
	}
}
