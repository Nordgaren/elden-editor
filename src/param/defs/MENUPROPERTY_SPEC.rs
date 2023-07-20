/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MENUPROPERTY_SPEC {
    /// NAME: Item name Text ID - 項目名テキストID
    pub CaptionTextID: i32,

    /// NAME: Item icon ID - 項目アイコンID
    pub IconID: i32,

    /// NAME: Required skills - 必要スキル
    pub RequiredPropertyID: u32,

    /// NAME: Superiority or inferiority judgment - 優劣判定
    pub CompareType: i8,

    /// NAME: Padding - パディング
    pub pad2: [u8; 1],

    /// NAME: Format - 書式
    pub FormatType: u16,

    /// NAME: Padding - パディング
    pub pad: [u8; 16],
}

impl Paramdef for MENUPROPERTY_SPEC {
    const NAME: &'static str = "MENUPROPERTY_SPEC";
    const VERSION: u16 = 1;
}
