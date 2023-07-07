/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct HIT_MTRL_PARAM_ST {

	/// NAME: Sound radius magnification - 音半径倍率
	/// DESC: Normal when 1x. When set to 0, the radius of sound becomes 0 (SE and SFX are unrelated game-like parameters). - 1倍のときは普通。0にすると音半径が0になる（SEとSFXは無関係のゲーム的なパラメータ）
	pub aiVolumeRate:f32,

	/// NAME: Special effect 0 applied when stepping on hit material - ヒットマテリアルを踏んだ時にかかる特殊効果0
	/// DESC: When the character steps on the hit material, the set special effect 0 is exhibited. - キャラがヒットマテリアルを踏んだ時に、設定した特殊効果0が発揮される
	pub spEffectIdOnHit0:i32,

	/// NAME: Special effect applied when stepping on the hit material 1 - ヒットマテリアルを踏んだ時にかかる特殊効果1
	/// DESC: When the character steps on the hit material, the set special effect 1 is exhibited. - キャラがヒットマテリアルを踏んだ時に、設定した特殊効果1が発揮される
	pub spEffectIdOnHit1:i32,

	/// NAME: Foot effect height type - フットエフェクトの高さタイプ
	/// DESC: Height to generate foot effect - フットエフェクトを発生させる高さ
	pub Bitfield1:u8,

	/// NAME: Material hardness type - 材質の固さタイプ
	/// DESC: The hardness of the material. Used for soft contact processing of rigid bodies. - 材質の固さ。剛体のソフトコンタクト処理に使用。
	pub hardnessType:u8,

	/// NAME: pad - pad
	/// DESC: pad - pad
	pub pad2:[u8;6],

	/// NAME: Special effect applied when stepping on the hit material 0 2nd lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　2周目
	/// DESC: Special effect applied when stepping on the hit material 0 2nd lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　2周目
	pub spEffectIdOnHit0_ClearCount_2:i32,

	/// NAME: Special effect applied when stepping on the hit material 0 3rd lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　3周目
	/// DESC: Special effect applied when stepping on the hit material 0 3rd lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　3周目
	pub spEffectIdOnHit0_ClearCount_3:i32,

	/// NAME: Special effect applied when stepping on the hit material 0 4th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　4周目
	/// DESC: Special effect applied when stepping on the hit material 0 4th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　4周目
	pub spEffectIdOnHit0_ClearCount_4:i32,

	/// NAME: Special effect applied when stepping on the hit material 0 5th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　5周目
	/// DESC: Special effect applied when stepping on the hit material 0 5th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　5周目
	pub spEffectIdOnHit0_ClearCount_5:i32,

	/// NAME: Special effect applied when stepping on the hit material 0 6th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　6周目
	/// DESC: Special effect applied when stepping on the hit material 0 6th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　6周目
	pub spEffectIdOnHit0_ClearCount_6:i32,

	/// NAME: Special effect applied when stepping on the hit material 0 7th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　7周目
	/// DESC: Special effect applied when stepping on the hit material 0 7th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　7周目
	pub spEffectIdOnHit0_ClearCount_7:i32,

	/// NAME: Special effect applied when stepping on the hit material 0 8th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　8周目
	/// DESC: Special effect applied when stepping on the hit material 0 8th lap - ヒットマテリアルを踏んだ時にかかる特殊効果0　8周目
	pub spEffectIdOnHit0_ClearCount_8:i32,

	/// NAME: Special effect applied when stepping on the hit material 1st lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　2周目
	/// DESC: Special effect applied when stepping on the hit material 1st lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　2周目
	pub spEffectIdOnHit1_ClearCount_2:i32,

	/// NAME: Special effect applied when stepping on the hit material 1 3rd lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　3周目
	/// DESC: Special effect applied when stepping on the hit material 1 3rd lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　3周目
	pub spEffectIdOnHit1_ClearCount_3:i32,

	/// NAME: Special effect when stepping on the hit material 1 4th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　4周目
	/// DESC: Special effect applied when stepping on the hit material 1 4th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　4周目
	pub spEffectIdOnHit1_ClearCount_4:i32,

	/// NAME: Special effect applied when stepping on the hit material 1 5th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　5周目
	/// DESC: Special effect applied when stepping on the hit material 1 5th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　5周目
	pub spEffectIdOnHit1_ClearCount_5:i32,

	/// NAME: Special effect when stepping on the hit material 1 6th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　6周目
	/// DESC: Special effect when stepping on the hit material 1 6th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　6周目
	pub spEffectIdOnHit1_ClearCount_6:i32,

	/// NAME: Special effect applied when stepping on the hit material 1 7th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　7周目
	/// DESC: Special effect applied when stepping on the hit material 1 7th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　7周目
	pub spEffectIdOnHit1_ClearCount_7:i32,

	/// NAME: Special effect when stepping on the hit material 1 8th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　8周目
	/// DESC: Special effect applied when stepping on the hit material 1 8th lap - ヒットマテリアルを踏んだ時にかかる特殊効果1　8周目
	pub spEffectIdOnHit1_ClearCount_8:i32,

	/// NAME: Hit material replacement (rain) - ヒットマテリアル差し替え(雨)
	/// DESC: Hit material change destination ID due to weather (rain) (-1: No change) - 天候(雨)によるヒットマテリアル変更先ID(-1：変更を行なわない)
	pub replaceMateiralId_Rain:i16,

	/// NAME: pad - pad
	/// DESC: pad - pad
	pub pad4:[u8;2],

	/// NAME: Wet special effect ID_00 - 濡れ特殊効果ID_00
	/// DESC: Special effect for wetting 00 - 濡れ用特殊効果00
	pub spEffectId_forWet00:i32,

	/// NAME: Wet special effect ID_01 - 濡れ特殊効果ID_01
	/// DESC: Special effects for wetting 01 - 濡れ用特殊効果01
	pub spEffectId_forWet01:i32,

	/// NAME: Wet special effect ID_02 - 濡れ特殊効果ID_02
	/// DESC: Special effects for wetting 02 - 濡れ用特殊効果02
	pub spEffectId_forWet02:i32,

	/// NAME: Wet special effect ID_03 - 濡れ特殊効果ID_03
	/// DESC: Special effects for wetting 03 - 濡れ用特殊効果03
	pub spEffectId_forWet03:i32,

	/// NAME: Wet special effect ID_04 - 濡れ特殊効果ID_04
	/// DESC: Special effects for wetting 04 - 濡れ用特殊効果04
	pub spEffectId_forWet04:i32,
}

impl HIT_MTRL_PARAM_ST {
	/// Height to generate foot effect - フットエフェクトを発生させる高さ
	/// Bitfield1
	pub fn get_footEffectHeightType(&self) -> u8 {
		&self.Bitfield1 & 0x6
	}

	/// Bitfield1 MAX: 3
	pub fn set_footEffectHeightType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0x6;
			let newVal = &self.Bitfield1 & 0xF9 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xF9
		}
	}	/// Direction of foot effect - フットエフェクトの発生向き
	/// Bitfield1
	pub fn get_footEffectDirType(&self) -> u8 {
		&self.Bitfield1 & 0xC
	}

	/// Bitfield1 MAX: 3
	pub fn set_footEffectDirType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 2) & 0xC;
			let newVal = &self.Bitfield1 & 0xF3 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xF3
		}
	}	/// For floating items such as the surface of the water - 水面などアイテムを浮かせるとき用
	/// Bitfield1
	pub fn get_floorHeightType(&self) -> u8 {
		&self.Bitfield1 & 0x30
	}

	/// Bitfield1 MAX: 3
	pub fn set_floorHeightType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 4) & 0x30;
			let newVal = &self.Bitfield1 & 0xCF | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xCF
		}
	}	/// Set to 1 for floors that are not subject to fall damage - 落下ダメージを受けない床の場合に 1 を設定する
	/// Bitfield1
	pub fn get_disableFallDamage(&self) -> bool {
		&self.Bitfield1 & 0x40 != 0
	}

	/// Bitfield1
	pub fn set_disableFallDamage(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40
		} else {
			self.Bitfield1 &= 0xBF
		}
	}
	/// Is it a hard material for sound echo? (0: soft, 1: hard) - サウンド反響用 硬い材質か？(0:やわらかい,1:かたい)
	/// Bitfield1
	pub fn get_isHardnessForSoundReverb(&self) -> bool {
		&self.Bitfield1 & 0x80 != 0
	}

	/// Bitfield1
	pub fn set_isHardnessForSoundReverb(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80
		} else {
			self.Bitfield1 &= 0x7F
		}
	}

}
