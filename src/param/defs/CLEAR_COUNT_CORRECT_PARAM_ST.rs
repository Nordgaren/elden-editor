/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CLEAR_COUNT_CORRECT_PARAM_ST {

	/// NAME: 《Maximum HP magnification [%]》 - 《最大HP倍率[%]》
	/// DESC: Maximum HP magnification [%] - 最大HP倍率[%]
	pub MaxHpRate:f32,

	/// NAME: 《Maximum MP magnification [%]》 - 《最大MP倍率[%]》
	/// DESC: Maximum MP magnification [%] - 最大MP倍率[%]
	pub MaxMpRate:f32,

	/// NAME: 《Maximum stamina magnification [%]》 - 《最大スタミナ倍率[%]》
	/// DESC: Maximum stamina magnification [%] - 最大スタミナ倍率[%]
	pub MaxStaminaRate:f32,

	/// NAME: 《Physical attack power multiplier》 - 《物理攻撃力倍率》
	/// DESC: Physical attack power multiplier - 物理攻撃力倍率
	pub PhysicsAttackRate:f32,

	/// NAME: 《Slashing attack power multiplier》 - 《斬撃攻撃力倍率》
	/// DESC: Slash attack power multiplier - 斬撃攻撃力倍率
	pub SlashAttackRate:f32,

	/// NAME: 《Batter attack power multiplier》 - 《打撃攻撃力倍率》
	/// DESC: Batter attack power multiplier - 打撃攻撃力倍率
	pub BlowAttackRate:f32,

	/// NAME: 《Puncture attack power multiplier》 - 《刺突攻撃力倍率》
	/// DESC: Puncture attack power multiplier - 刺突攻撃力倍率
	pub ThrustAttackRate:f32,

	/// NAME: 《Non-attribute attack power multiplier》 - 《無属性攻撃力倍率》
	/// DESC: Non-attribute attack power multiplier - 無属性攻撃力倍率
	pub NeturalAttackRate:f32,

	/// NAME: 《Magic attack power multiplier》 - 《魔法攻撃力倍率》
	/// DESC: Magic attack power multiplier - 魔法攻撃力倍率
	pub MagicAttackRate:f32,

	/// NAME: 《Flame attack power multiplier》 - 《炎攻撃力倍率》
	/// DESC: Fire attack power multiplier - 炎攻撃力倍率
	pub FireAttackRate:f32,

	/// NAME: 《Electric shock attack power multiplier》 - 《電撃攻撃力倍率》
	/// DESC: Electric shock attack power multiplier - 電撃攻撃力倍率
	pub ThunderAttackRate:f32,

	/// NAME: 《Dark attack power multiplier》 - 《闇攻撃力倍率》
	/// DESC: Dark attack power multiplier - 闇攻撃力倍率
	pub DarkAttackRate:f32,

	/// NAME: 《Physical defense multiplier》 - 《物理防御力倍率》
	/// DESC: Physical defense power multiplier - 物理防御力倍率
	pub PhysicsDefenseRate:f32,

	/// NAME: 《Magic Defense Multiplier》 - 《魔法防御力倍率》
	/// DESC: Magic defense multiplier - 魔法防御力倍率
	pub MagicDefenseRate:f32,

	/// NAME: 《Flame defense multiplier》 - 《炎防御力倍率》
	/// DESC: Fire defense multiplier - 炎防御力倍率
	pub FireDefenseRate:f32,

	/// NAME: 《Dengeki Defense Magnification》 - 《電撃防御力倍率》
	/// DESC: Electric shock defense power multiplier - 電撃防御力倍率
	pub ThunderDefenseRate:f32,

	/// NAME: 《Darkness Defense Multiplier》 - 《闇防御力倍率》
	/// DESC: Dark defense multiplier - 闇防御力倍率
	pub DarkDefenseRate:f32,

	/// NAME: 《Stamina attack power multiplier》 - 《スタミナ攻撃力倍率》
	/// DESC: Stamina attack power multiplier - スタミナ攻撃力倍率
	pub StaminaAttackRate:f32,

	/// NAME: 《Soul possession rate》 - 《所持ソウル率》
	/// DESC: Possession soul rate - 所持ソウル率
	pub SoulRate:f32,

	/// NAME: 《Poison resistance change rate》 - 《毒耐性変化倍率》
	/// DESC: Poison resistance change rate - 毒耐性変化倍率
	pub PoisionResistRate:f32,

	/// NAME: 《Pesticide resistance change rate》 - 《疫病耐性変化倍率》
	/// DESC: Epidemic resistance change rate - 疫病耐性変化倍率
	pub DiseaseResistRate:f32,

	/// NAME: 《Bleeding resistance change rate》 - 《出血耐性変化倍率》
	/// DESC: Bleeding resistance change rate - 出血耐性変化倍率
	pub BloodResistRate:f32,

	/// NAME: 《Curse resistance change rate》 - 《呪耐性変化倍率》
	/// DESC: Curse resistance change rate - 呪耐性変化倍率
	pub CurseResistRate:f32,

	/// NAME: 《Cold air resistance change rate》 - 《冷気耐性変化倍率》
	/// DESC: Cold resistance change rate - 冷気耐性変化倍率
	pub FreezeResistRate:f32,

	/// NAME: 《Bleeding damage correction factor》 - 《出血ダメージ補正倍率》
	/// DESC: Bleeding damage correction factor - 出血ダメージ補正倍率
	pub BloodDamageRate:f32,

	/// NAME: 《SA damage correction factor》 - 《SAダメージ補正倍率》
	/// DESC: SA damage correction factor - SAダメージ補正倍率
	pub SuperArmorDamageRate:f32,

	/// NAME: 《Cold air damage correction factor》 - 《冷気ダメージ補正倍率》
	/// DESC: Cold damage correction factor - 冷気ダメージ補正倍率
	pub FreezeDamageRate:f32,

	/// NAME: 《Sleep tolerance change rate》 - 《睡眠耐性変化倍率》
	/// DESC: Sleep tolerance change rate - 睡眠耐性変化倍率
	pub SleepResistRate:f32,

	/// NAME: 《Madness resistance change rate》 - 《発狂耐性変化倍率》
	/// DESC: Madness resistance change rate - 発狂耐性変化倍率
	pub MadnessResistRate:f32,

	/// NAME: 《Sleep damage correction factor》 - 《睡眠ダメージ補正倍率》
	/// DESC: Sleep damage correction factor - 睡眠ダメージ補正倍率
	pub SleepDamageRate:f32,

	/// NAME: 《Crazy damage correction factor》 - 《発狂ダメージ補正倍率》
	/// DESC: Mad damage correction factor - 発狂ダメージ補正倍率
	pub MadnessDamageRate:f32,

	/// NAME: pad - pad
	pub pad1:[u8;4],
}

impl Paramdef for CLEAR_COUNT_CORRECT_PARAM_ST {
const NAME: &'static str = "CLEAR_COUNT_CORRECT_PARAM_ST";
const VERSION: u16 = 1;
}
