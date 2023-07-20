/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct AI_SOUND_PARAM_ST {
    /// NAME: Sound radius [m] - 音半径[m]
    /// DESC: AI sound radius - AI音の半径
    pub radius: f32,

    /// NAME: Extinction time [seconds] - 消滅時間[秒]
    /// DESC: Time for AI sound to remain - AI音が残る時間
    pub lifeFrame: f32,

    /// NAME: Is it affected by special effects? - 特殊効果からの影響を受けるか
    /// DESC: Whether it is affected by the special effect "Sound Radius Magnification" - 特殊効果の”音半径倍率”の影響を受けるかどうか
    pub bSpEffectEnable: u8,

    /// NAME: kinds - 種別
    /// DESC: AI sound type - AI音の種別
    pub r#type: u8,

    /// NAME: Target: ● Hostile - 対象：●敵対
    /// DESC: Target: ● Hostile - 対象：●敵対
    pub Bitfield1: u8,

    /// NAME: Character behavior - キャラの振る舞い
    /// DESC: Character behavior (former: sound type) - キャラの振る舞い(旧：音タイプ
    pub rank: u8,

    /// NAME: Time to forget the sound target (overwrite) [sec] - 音ターゲット忘れる時間（上書き）[sec]
    /// DESC: Time to forget the sound target (overwrite) [sec] - 音ターゲット忘れる時間（上書き）[sec]
    pub forgetTime: f32,

    /// NAME: AI sound priority - AI音優先度
    /// DESC: AI sound priority - AI音優先度
    pub priority: i32,

    /// NAME: Behavior ID - 振る舞いID
    /// DESC: Behavior ID - 振る舞いID
    pub soundBehaviorId: i32,

    /// NAME: AI sound level - AI音レベル
    /// DESC: How hard it is to hear - どれくらい聞き取りづらい音なのか
    pub aiSoundLevel: u8,

    /// NAME: AI state to replan - リプランニングするAI状態
    /// DESC: AI state setting to run replanning when listening to AI sound - AI音を聞いた際にリプランニングを走らせるAI状態の設定
    pub replaningState: u8,

    /// NAME: pad - パッド
    /// DESC: pad - pad
    pub pad1: [u8; 6],
}

impl Paramdef for AI_SOUND_PARAM_ST {
    const NAME: &'static str = "AI_SOUND_PARAM_ST";
    const VERSION: u16 = 1;
}
impl AI_SOUND_PARAM_ST {
    /// Target: ● Hostile - 対象：●敵対
    /// Bitfield1
    pub fn get_opposeTarget(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_opposeTarget(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// Target: ○ Allies - 対象：○味方
    /// Bitfield1
    pub fn get_friendlyTarget(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_friendlyTarget(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Target: myself - 対象：自分
    /// Bitfield1
    pub fn get_selfTarget(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_selfTarget(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// You can't listen while targeting your PC - PC自軍をターゲット中は聞けない
    /// Bitfield1
    pub fn get_disableOnTargetPCompany(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_disableOnTargetPCompany(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
}
