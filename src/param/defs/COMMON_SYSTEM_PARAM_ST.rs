/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct COMMON_SYSTEM_PARAM_ST {
    /// NAME: At the start of the game Map name ID_for save data - ゲーム開始時マップ名ID_セーブデータ用
    /// DESC: At the start of the game Map name ID_for save data (SEQ15052) - ゲーム開始時マップ名ID_セーブデータ用(SEQ15052)
    pub mapSaveMapNameIdOnGameStart: u32,

    /// NAME: Reserve - リザーブ
    /// DESC: Reserve - リザーブ
    pub reserve0: [u8; 60],
}

impl Paramdef for COMMON_SYSTEM_PARAM_ST {
    const NAME: &'static str = "COMMON_SYSTEM_PARAM_ST";
    const VERSION: u16 = 0;
}
