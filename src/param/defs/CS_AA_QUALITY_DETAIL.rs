/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_AA_QUALITY_DETAIL {
    /// NAME: AA valid - AA有効
    /// DESC: AA valid - AA有効
    pub enabled: u8,

    /// NAME: Force FXAA2 - 強制的にFXAA2
    /// DESC: Force FXAA2 - 強制的にFXAA2
    pub forceFXAA2: u8,

    /// NAME: dmy - dmy
    pub dmy: [u8; 2],
}

impl Paramdef for CS_AA_QUALITY_DETAIL {
    const NAME: &'static str = "CS_AA_QUALITY_DETAIL";
    const VERSION: u16 = 1;
}
