/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ENEMY_COMMON_PARAM_ST {
    /// NAME: reserve - 予約
    /// DESC: (dummy8) - (dummy8)
    pub reserved0: [u8; 8],

    /// NAME: Time to try to approach the sound target - 音ターゲットに対して接近を試みる時間
    pub soundTargetTryApproachTime: i32,

    /// NAME: Time to try to approach the search target - 索敵ターゲットに対して接近を試みる時間
    pub searchTargetTryApproachTime: i32,

    /// NAME: Time to try to approach the memory target - 記憶ターゲットに対して接近を試みる時間
    pub memoryTargetTryApproachTime: i32,

    /// NAME: reserve - 予約
    /// DESC: (dummy8) - (dummy8)
    pub reserved5: [u8; 40],

    /// NAME: Phantom shader ID used for directing the appearance and disappearance of enemies placed in a specific time zone - 特定時間帯配置のエネミーの出現・消滅に演出に使うファントムシェーダID
    /// DESC: Fade the phantom shader with the appearance / disappearance effect, - 出現・消滅の演出でファントムシェーダをフェイドする、
    pub activateChrByTime_PhantomId: i32,

    /// NAME: Distance to generate an interrupt, which turns out that the enemy is likely to be cut off when going to the end of the path - パス終端まで行くと敵が見切れそうなことがわかったインタラプト、を発生させる距離
    /// DESC: At the time of Unreach pass, if the end and the target are within this distance, an interrupt is generated. - Unreachパス時、終端と対象がこの距離以内ならインタラプトを発生させる
    pub findUnfavorableFailedPointDist: f32,

    /// NAME: The height that causes an interrupt, which turns out that the enemy is likely to be cut off when going to the end of the path - パス終端まで行くと敵が見切れそうなことがわかったインタラプト、を発生させる高さ
    /// DESC: At the time of Unreach pass, if the end and the target are more than this distance, an interrupt is generated. - Unreachパス時、終端と対象がこの距離以上ならインタラプトを発生させる
    pub findUnfavorableFailedPointHeight: f32,

    /// NAME: reserve - 予約
    /// DESC: (dummy8) - (dummy8)
    pub reserved18: [u8; 184],
}

impl Paramdef for ENEMY_COMMON_PARAM_ST {
    const NAME: &'static str = "ENEMY_COMMON_PARAM_ST";
    const VERSION: u16 = 1;
}
