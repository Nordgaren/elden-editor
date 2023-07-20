/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct REINFORCE_PARAM_WEAPON_ST {

	/// NAME: Physical attack power basic value - 物理攻撃力基本値
	/// DESC: Physical attack power correction value - 物理攻撃力の補正値
	pub physicsAtkRate:f32,

	/// NAME: Magic attack power basic value - 魔法攻撃力基本値
	/// DESC: Magic attack power correction value - 魔法攻撃力の補正値
	pub magicAtkRate:f32,

	/// NAME: Fire attack power basic value - 炎攻撃力基本値
	/// DESC: Fire attack power correction value - 炎攻撃力の補正値
	pub fireAtkRate:f32,

	/// NAME: Electric shock attack power basic value - 電撃攻撃力基本値
	/// DESC: Correction value of electric shock attack power - 電撃攻撃力の補正値
	pub thunderAtkRate:f32,

	/// NAME: Stamina attack power - スタミナ攻撃力
	/// DESC: Stamina attack power correction value - スタミナ攻撃力の補正値
	pub staminaAtkRate:f32,

	/// NAME: SA weapon attack power - SA武器攻撃力
	/// DESC: Super Armor Weapon Attack Color Correction Value - スーパーアーマー武器攻撃色の補正値
	pub saWeaponAtkRate:f32,

	/// NAME: SA durability value - SA耐久値
	/// DESC: SA endurance correction value - SA耐久力の補正値
	pub saDurabilityRate:f32,

	/// NAME: Strength correction - 筋力補正
	/// DESC: Correction value of muscle strength correction - 筋力補正の補正値
	pub correctStrengthRate:f32,

	/// NAME: Agility correction - 俊敏補正
	/// DESC: Correction value of agility correction - 俊敏補正の補正値
	pub correctAgilityRate:f32,

	/// NAME: Magic correction - 魔力補正
	/// DESC: Correction value of magic power correction - 魔力補正の補正値
	pub correctMagicRate:f32,

	/// NAME: Faith correction - 信仰補正
	/// DESC: Correction value of faith correction - 信仰補正の補正値
	pub correctFaithRate:f32,

	/// NAME: Physical attack cut rate when guarding - ガード時物理攻撃カット率
	/// DESC: Correction value of physical attack cut rate when guarding - ガード時物理攻撃カット率の補正値
	pub physicsGuardCutRate:f32,

	/// NAME: Magic attack cut rate when guarding - ガード時魔法攻撃カット率
	/// DESC: Correction value of magic attack cut rate when guarding - ガード時魔法攻撃カット率の補正値
	pub magicGuardCutRate:f32,

	/// NAME: Flame attack cut rate when guarding - ガード時炎攻撃カット率
	/// DESC: Correction value of flame attack cut rate when guarding - ガード時炎攻撃カット率の補正値
	pub fireGuardCutRate:f32,

	/// NAME: Electric shock attack cut rate when guarding - ガード時電撃攻撃カット率
	/// DESC: Correction value of electric shock attack cut rate when guarding - ガード時電撃攻撃カット率の補正値
	pub thunderGuardCutRate:f32,

	/// NAME: Guard time poison attack cut rate - ガード時毒攻撃カット率
	/// DESC: Correction value of poison attack cut rate when guarding - ガード時毒攻撃カット率の補正値
	pub poisonGuardResistRate:f32,

	/// NAME: Epidemic attack cut rate when guarding - ガード時疫病攻撃カット率
	/// DESC: Correction value of plague attack cut rate when guarding - ガード時疫病攻撃カット率の補正値
	pub diseaseGuardResistRate:f32,

	/// NAME: Bleeding attack cut rate when guarding - ガード時出血攻撃カット率
	/// DESC: Correction value for bleeding attack cut rate when guarding - ガード時出血攻撃カット率の補正値
	pub bloodGuardResistRate:f32,

	/// NAME: Curse attack cut rate when guarding - ガード時呪攻撃カット率
	/// DESC: Correction value of curse attack cut rate when guarding - ガード時呪い攻撃カット率の補正値
	pub curseGuardResistRate:f32,

	/// NAME: Stamina defense when guarding - ガード時スタミナ防御力
	/// DESC: Correction value of stamina defense power when guarding - ガード時スタミナ防御力の補正値
	pub staminaGuardDefRate:f32,

	/// NAME: Special effect ID1 - 特殊効果ID1
	/// DESC: Addition correction value for special effect ID1 - 特殊効果ID1の加算補正値
	pub spEffectId1:u8,

	/// NAME: Special effect ID2 - 特殊効果ID2
	/// DESC: Addition correction value for special effect ID2 - 特殊効果ID2の加算補正値
	pub spEffectId2:u8,

	/// NAME: Special effect ID3 - 特殊効果ID3
	/// DESC: Addition correction value for special effect ID3 - 特殊効果ID3の加算補正値
	pub spEffectId3:u8,

	/// NAME: Resident special effect ID1 - 常駐特殊効果ID1
	/// DESC: Addition correction value for resident special effect ID1 - 常駐特殊効果ID1の加算補正値
	pub residentSpEffectId1:u8,

	/// NAME: Resident special effect ID2 - 常駐特殊効果ID2
	/// DESC: Addition correction value for resident special effect ID2 - 常駐特殊効果ID2の加算補正値
	pub residentSpEffectId2:u8,

	/// NAME: Resident special effect ID3 - 常駐特殊効果ID3
	/// DESC: Addition correction value for resident special effect ID3 - 常駐特殊効果ID3の加算補正値
	pub residentSpEffectId3:u8,

	/// NAME: Material ID addition value - 素材ID加算値
	/// DESC: Addition correction value of material parameter ID - 素材パラメータIDの加算補正値
	pub materialSetId:u8,

	/// NAME: Level value for maximum enhanced weapon level - 最大強化武器レベル用レベル値
	/// DESC: Level value for maximum enhanced weapon level - 最大強化武器レベル用レベル値
	pub maxReinforceLevel:u8,

	/// NAME: Dark attack power basic value - 闇攻撃力基本値
	/// DESC: Dark attack power correction value - 闇攻撃力の補正値
	pub darkAtkRate:f32,

	/// NAME: Dark attack cut rate when guarding - ガード時闇攻撃カット率
	/// DESC: Correction value of darkness attack cut rate when guarding - ガード時闇攻撃カット率の補正値
	pub darkGuardCutRate:f32,

	/// NAME: Luck correction - 運補正
	/// DESC: Correction value of luck correction - 運補正の補正値
	pub correctLuckRate:f32,

	/// NAME: Cold attack cut rate when guarding - ガード時冷気攻撃カット率
	/// DESC: Correction value of cold air attack cut rate when guarding - ガード時冷気攻撃カット率の補正値
	pub freezeGuardDefRate:f32,

	/// NAME: Enhanced price correction value - 強化価格補正値
	/// DESC: Correction value to multiply the enhancement price of the weapon parameter - 武器パラメータの強化価格に乗算する補正値
	pub reinforcePriceRate:f32,

	/// NAME: Evolution price correction value - 進化価格補正値
	/// DESC: Correction value to multiply the evolution price of the weapon parameter - 武器パラメータの進化価格に乗算する補正値
	pub baseChangePriceRate:f32,

	/// NAME: Mountable magic stone rank - 装着可能魔石ランク
	/// DESC: Mountable magic stone rank - 装着可能魔石ランク
	pub enableGemRank:i8,

	/// NAME: pad - pad
	pub pad2:[u8;3],

	/// NAME: Sleep attack cut rate when guarding - ガード時睡眠攻撃カット率
	/// DESC: Correction value of sleep attack cut rate when guarding - ガード時睡眠攻撃カット率の補正値
	pub sleepGuardDefRate:f32,

	/// NAME: Mad attack cut rate when guarding - ガード時発狂攻撃カット率
	/// DESC: Correction value of mad attack cut rate when guarding - ガード時発狂攻撃カット率の補正値
	pub madnessGuardDefRate:f32,

	/// NAME: Additional attack power multiplier - 加算攻撃力倍率
	/// DESC: Additional attack power multiplier - 加算攻撃力倍率
	pub baseAtkRate:f32,
}

impl Paramdef for REINFORCE_PARAM_WEAPON_ST {
const NAME: &'static str = "REINFORCE_PARAM_WEAPON_ST";
const VERSION: u16 = 1;
}
