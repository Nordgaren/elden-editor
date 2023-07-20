/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SE_ACTIVATION_RANGE_PARAM_ST {
    /// NAME: Activate distance - アクティベート距離
    /// DESC: Distance to enable placement SE (m) (0 or less: always enabled) - 配置SEを有効化する距離(m) (0以下：常に有効化)
    pub activateRange: f32,
}

impl Paramdef for SE_ACTIVATION_RANGE_PARAM_ST {
    const NAME: &'static str = "SE_ACTIVATION_RANGE_PARAM_ST";
    const VERSION: u16 = 1;
}
