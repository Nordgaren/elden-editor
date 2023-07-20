/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
    /// NAME: Sound No - サウンドNo
    /// DESC: Sound No to play (sound type is fixed to a) - 再生するサウンドNo (サウンドタイプはa固定)
    pub SoundNo: i32,

    /// NAME: Judgment extended distance - 判定拡張距離
    /// DESC: Extended distance of playback judgment area - 再生判定領域の拡張距離
    pub ExpandRange: f32,

    /// NAME: Sound source tracking speed - 音源追従スピード
    /// DESC: Follow-up speed (fixed speed) to the target position of the actual sound source - 実音源の目標位置への追従速度(固定速度)
    pub FollowSpeed: f32,

    /// NAME: Sound source follow-up rate - 音源追従率
    /// DESC: Follow-up speed (difference ratio) to the target position of the actual sound source - 実音源の目標位置への追従速度(差分割合)
    pub FollowRate: f32,
}

impl Paramdef for SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
    const NAME: &'static str = "SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST";
    const VERSION: u16 = 0;
}
