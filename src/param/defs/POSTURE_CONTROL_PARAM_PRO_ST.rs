/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct POSTURE_CONTROL_PARAM_PRO_ST {
    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a000_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a000_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a000_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a000_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a002_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a002_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a002_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a002_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a003_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a003_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a003_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a003_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a010_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a010_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a010_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a010_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a012_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a012_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a012_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a012_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a013_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a013_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a013_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a013_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a014_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a014_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a014_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a014_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a015_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a015_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a015_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a015_leftArmFB: i16,

    /// NAME: Right arm_inside and outside - 右腕_内外
    /// DESC: Right arm_inside and outside - 右腕_内外
    pub a016_rightArmIO: i16,

    /// NAME: Right arm_front and back - 右腕_前後
    /// DESC: Right arm_front and back - 右腕_前後
    pub a016_rightArmFB: i16,

    /// NAME: Left arm_inside and outside - 左腕_内外
    /// DESC: Left arm_inside and outside - 左腕_内外
    pub a016_leftArmIO: i16,

    /// NAME: Left arm_front and back - 左腕_前後
    /// DESC: Left arm_front and back - 左腕_前後
    pub a016_leftArmFB: i16,

    /// NAME: pad - pad
    pub pad: [u8; 8],
}

impl Paramdef for POSTURE_CONTROL_PARAM_PRO_ST {
    const NAME: &'static str = "POSTURE_CONTROL_PARAM_PRO_ST";
    const VERSION: u16 = 0;
}
