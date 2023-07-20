/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CAMERA_FADE_PARAM_ST {
    /// NAME: Distance to be transparent (m) - 透明になりきる距離(m)
    /// DESC: Near Fade minimum distance (m): Distance where α = 0 - Nearフェード最小距離(m) : α = 0になる距離
    pub NearMinDist: f32,

    /// NAME: Distance that begins to become transparent (m) - 透明になり始める距離(m)
    /// DESC: Near fade maximum distance (m): Starting distance between α = Middel Alpha - Nearフェード最大距離(m) : α = MiddelAlphaとなる間の開始距離
    pub NearMaxDist: f32,

    /// NAME: Distance to become translucent (m) - 半透明状態になりきる距離(m)
    /// DESC: Minimum distance of Far fade (m): End distance between α = Middle Alpha - Farフェードの最小距離(m) : α = MiddleAlphaとなる間の終了距離
    pub FarMinDist: f32,

    /// NAME: Distance (m) at which it begins to become translucent - 半透明状態になり始める距離(m)
    /// DESC: Maximum Far Fade Distance (m): Distance where α = 1 - Farフェードの最大距離(m) : α = 1になる距離
    pub FarMaxDist: f32,

    /// NAME: Translucent darkness (α value) - 半透明状態の濃さ(α値)
    /// DESC: Intermediate α value - 中間のα値
    pub MiddleAlpha: f32,

    /// NAME: dummy - ダミー
    pub dummy: [u8; 12],
}

impl Paramdef for CAMERA_FADE_PARAM_ST {
    const NAME: &'static str = "CAMERA_FADE_PARAM_ST";
    const VERSION: u16 = 1;
}
