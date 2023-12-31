/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_DOF_QUALITY_DETAIL {
    /// NAME: DOF permission - DOF許可
    /// DESC: DOF permission - DOF許可
    pub enabled: u8,

    /// NAME: dmy - dmy
    pub dmy: [u8; 3],

    /// NAME: Change HiResolutionBlur settings - HiResolutionBlur の設定を変更する
    /// DESC: Change the HiResolutionBlur setting (-1: forced off, 0: as-is, 1: forced on) - HiResolutionBlur の設定を変更する(-1:強制オフ、0:そのまま、1:強制オン)
    pub forceHiResoBlur: i32,

    /// NAME: Maximum blur level - 最大ブラーレベル
    /// DESC: Maximum blur level. 2: Maximum, 1: Level to one paragraph, 0: Further reduce accuracy - 最大ブラーレベル。2:最大、1:レベルを一段落とす、0:さらに精度を落とす
    pub maxBlurLevel: i32,
}

impl Paramdef for CS_DOF_QUALITY_DETAIL {
    const NAME: &'static str = "CS_DOF_QUALITY_DETAIL";
    const VERSION: u16 = 1;
}
