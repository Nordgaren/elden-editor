/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SE_MATERIAL_CONVERT_PARAM_ST {
    /// NAME: SE material ID - SE材質ID
    /// DESC: Conversion from SFX material ID (3 digits) to SE material ID (2 digits) - SFX材質ID（３桁）からSE材質ID（２桁）への変換
    pub seMaterialId: u8,

    /// NAME: Padding - パディング
    /// DESC: Padding - パディング
    pub pad: [u8; 3],
}

impl Paramdef for SE_MATERIAL_CONVERT_PARAM_ST {
    const NAME: &'static str = "SE_MATERIAL_CONVERT_PARAM_ST";
    const VERSION: u16 = 1;
}
