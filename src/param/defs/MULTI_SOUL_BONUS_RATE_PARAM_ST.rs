/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MULTI_SOUL_BONUS_RATE_PARAM_ST {
    /// NAME: host - ホスト
    /// DESC: Host reward soul multiplier - ホストの報酬ソウル倍率
    pub host: f32,

    /// NAME: White sign - 白サイン
    /// DESC: Cooperation sign white spirit reward soul multiplier - 協力サインの白霊の報酬ソウル倍率
    pub WhiteGhost_None: f32,

    /// NAME: Gold spirit (sun) - 金霊（太陽）
    /// DESC: Cooperative sign gold spirit reward soul multiplier - 協力サインの金霊の報酬ソウル倍率
    pub WhiteGhost_Umbasa: f32,

    /// NAME: White berserker - 白バーサーカー
    /// DESC: Cooperation sign white Berserker reward soul multiplier - 協力サインの白バーサーカーの報酬ソウル倍率
    pub WhiteGhost_Berserker: f32,

    /// NAME: Red sign - 赤サイン
    /// DESC: Hostile sign red spirit reward soul multiplier - 敵対サインの赤霊の報酬ソウル倍率
    pub BlackGhost_None_Sign: f32,

    /// NAME: Red gold spirit (signature) - 赤金霊（サイン）
    /// DESC: Hostile sign red gold spirit reward soul multiplier - 敵対サインの赤金霊の報酬ソウル倍率
    pub BlackGhost_Umbasa_Sign: f32,

    /// NAME: Red berserker (sign) - 赤バーサーカー（サイン）
    /// DESC: Hostile sign red berserker reward soul multiplier - 敵対サインの赤バーサーカーの報酬ソウル倍率
    pub BlackGhost_Berserker_Sign: f32,

    /// NAME: Invasion - 侵入
    /// DESC: Invasion reward Soul multiplier - 侵入の報酬ソウル倍率
    pub BlackGhost_None_Invade: f32,

    /// NAME: Red Gold Spirit (Invasion) - 赤金霊（侵入）
    /// DESC: Invasion Orb's Red Gold Spirit Reward Soul Multiplier - 侵入オーブの赤金霊の報酬ソウル倍率
    pub BlackGhost_Umbasa_Invade: f32,

    /// NAME: Red berserker (invasion) - 赤バーサーカー（侵入）
    /// DESC: Invasion Orb Red Berserker Reward Soul Multiplier - 侵入オーブの赤バーサーカーの報酬ソウル倍率
    pub BlackGhost_Berserker_Invade: f32,

    /// NAME: Relief guest - 救援ゲスト
    /// DESC: Relief guest reward soul multiplier - 救援ゲストの報酬ソウル倍率
    pub RedHunter1: f32,

    /// NAME: Red Scare Spirit 2 - 赤狩り霊２
    /// DESC: Red Scare Spirit 2 Reward Soul Multiplier - 赤狩り霊２の報酬ソウル倍率
    pub RedHunter2: f32,

    /// NAME: Map Guardian Spirit (Forest) - マップ守護霊(森)
    /// DESC: Map Guardian Spirit (Forest) Reward Soul Magnification - マップ守護霊（森）の報酬ソウル倍率
    pub GuardianOfForest: f32,

    /// NAME: Map Guardian (Anor) - マップ守護霊(アノール)
    /// DESC: Map Guardian Spirit (Anor) Reward Soul Multiplier - マップ守護霊(アノール)の報酬ソウル倍率
    pub GuardianOfAnor: f32,

    /// NAME: Arena - 闘技場
    /// DESC: Arena reward soul multiplier - 闘技場の報酬ソウル倍率
    pub BattleRoyal: f32,

    /// NAME: Yellow robe's old man - 黄衣の翁
    /// DESC: Yellow robe's old man's reward soul multiplier - 黄衣の翁の報酬ソウル倍率
    pub YellowMonk: f32,

    /// NAME: pad - pad
    pub pad1: [u8; 64],
}

impl Paramdef for MULTI_SOUL_BONUS_RATE_PARAM_ST {
    const NAME: &'static str = "MULTI_SOUL_BONUS_RATE_PARAM_ST";
    const VERSION: u16 = 3;
}
