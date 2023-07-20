/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct RUNTIME_BONE_CONTROL_PARAM_ST {
    /// NAME: Character ID - キャラID
    /// DESC: Character ID - キャラID
    pub chrId: u32,

    /// NAME: Control type - 制御タイプ
    /// DESC: Control type - 制御タイプ
    pub ctrlType: u8,

    /// NAME: pad - pad
    pub pad: [u8; 11],

    /// NAME: Applicable joint - 適用関節
    /// DESC: Applicable joint - 適用関節
    pub applyBone: [u8; 32],

    /// NAME: Target joint 1 - ターゲット関節１
    /// DESC: Target joint 1 - ターゲット関節１
    pub targetBone1: [u8; 32],

    /// NAME: Target joint 2 - ターゲット関節２
    /// DESC: Target joint 2 - ターゲット関節２
    pub targetBone2: [u8; 32],
}

impl Paramdef for RUNTIME_BONE_CONTROL_PARAM_ST {
    const NAME: &'static str = "RUNTIME_BONE_CONTROL_PARAM_ST";
    const VERSION: u16 = 1;
}
