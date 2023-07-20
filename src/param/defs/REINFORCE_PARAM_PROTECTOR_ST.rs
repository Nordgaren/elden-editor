/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct REINFORCE_PARAM_PROTECTOR_ST {
    /// NAME: Physical defense - 物理防御力
    /// DESC: Physical defense correction value - 物理防御力の補正値
    pub physicsDefRate: f32,

    /// NAME: Magic defense - 魔法防御力
    /// DESC: Magic defense correction value - 魔法防御力の補正値
    pub magicDefRate: f32,

    /// NAME: Fire defense - 炎防御力
    /// DESC: Fire defense correction value - 炎防御力の補正値
    pub fireDefRate: f32,

    /// NAME: Electric shock defense - 電撃防御力
    /// DESC: Electric shock defense correction value - 電撃防御力の補正値
    pub thunderDefRate: f32,

    /// NAME: Slash defense - 斬撃防御力
    /// DESC: Slash defense correction value - 斬撃防御力の補正値
    pub slashDefRate: f32,

    /// NAME: Strike defense - 打撃防御力
    /// DESC: Impact defense correction value - 打撃防御力の補正値
    pub blowDefRate: f32,

    /// NAME: Puncture defense - 刺突防御力
    /// DESC: Correction value of piercing defense power - 刺突防御力の補正値
    pub thrustDefRate: f32,

    /// NAME: Poison resistance - 毒耐性
    /// DESC: Poison resistance correction value - 毒耐性の補正値
    pub resistPoisonRate: f32,

    /// NAME: Epidemic resistance - 疫病耐性
    /// DESC: Epidemic resistance correction value - 疫病耐性の補正値
    pub resistDiseaseRate: f32,

    /// NAME: Bleeding resistance - 出血耐性
    /// DESC: Bleeding resistance correction value - 出血耐性の補正値
    pub resistBloodRate: f32,

    /// NAME: Curse resistance - 呪耐性
    /// DESC: Curse resistance correction value - 呪耐性の補正値
    pub resistCurseRate: f32,

    /// NAME: Resident special effect ID1 - 常駐特殊効果ID1
    /// DESC: Addition correction value for resident special effect ID1 - 常駐特殊効果ID1の加算補正値
    pub residentSpEffectId1: u8,

    /// NAME: Resident special effect ID2 - 常駐特殊効果ID2
    /// DESC: Addition correction value for resident special effect ID2 - 常駐特殊効果ID2の加算補正値
    pub residentSpEffectId2: u8,

    /// NAME: Resident special effect ID3 - 常駐特殊効果ID3
    /// DESC: Addition correction value for resident special effect ID3 - 常駐特殊効果ID3の加算補正値
    pub residentSpEffectId3: u8,

    /// NAME: Material ID addition value - 素材ID加算値
    /// DESC: Addition correction value of material parameter ID - 素材パラメータIDの加算補正値
    pub materialSetId: u8,

    /// NAME: Dark defense - 闇防御力
    /// DESC: Dark defense correction value - 闇防御力の補正値
    pub darkDefRate: f32,

    /// NAME: Cold resistance - 冷気耐性
    /// DESC: Cold resistance correction value - 冷気耐性の補正値
    pub resistFreezeRate: f32,

    /// NAME: Sleep tolerance - 睡眠耐性
    /// DESC: Correction value for sleep tolerance - 睡眠耐性の補正値
    pub resistSleepRate: f32,

    /// NAME: Madness resistance - 発狂耐性
    /// DESC: Madness resistance correction value - 発狂耐性の補正値
    pub resistMadnessRate: f32,
}

impl Paramdef for REINFORCE_PARAM_PROTECTOR_ST {
    const NAME: &'static str = "REINFORCE_PARAM_PROTECTOR_ST";
    const VERSION: u16 = 1;
}
