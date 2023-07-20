/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_DECAL_QUALITY_DETAIL {
    /// NAME: Decal valid - デカール有効
    /// DESC: Decal valid - デカール有効
    pub enabled: u8,

    /// NAME: dmy - dmy
    pub dmy: [u8; 3],
}

impl Paramdef for CS_DECAL_QUALITY_DETAIL {
    const NAME: &'static str = "CS_DECAL_QUALITY_DETAIL";
    const VERSION: u16 = 1;
}
