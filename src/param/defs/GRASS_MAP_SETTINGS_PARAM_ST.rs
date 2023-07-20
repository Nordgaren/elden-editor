/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 106
#[repr(C)]
pub struct GRASS_MAP_SETTINGS_PARAM_ST {
    /// NAME: Grass type 0 - 草の種類０
    pub grassType0: u32,

    /// NAME: Grass type 1 - 草の種類１
    pub grassType1: u32,

    /// NAME: Grass type 2 - 草の種類２
    pub grassType2: u32,
}

impl Paramdef for GRASS_MAP_SETTINGS_PARAM_ST {
    const NAME: &'static str = "GRASS_MAP_SETTINGS_PARAM_ST";
    const VERSION: u16 = 1;
}
