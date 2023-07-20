/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GRASS_LOD_RANGE_PARAM_ST {
    /// NAME: LOD0-distance - LOD0 - 距離
    pub LOD0_range: f32,

    /// NAME: LOD0-play - LOD0 - 遊び
    pub LOD0_play: f32,

    /// NAME: LOD1-distance - LOD１ - 距離
    pub LOD1_range: f32,

    /// NAME: LOD1-Play - LOD１ - 遊び
    pub LOD1_play: f32,

    /// NAME: LOD2-distance - LOD２ - 距離
    pub LOD2_range: f32,

    /// NAME: LOD2-Play - LOD２ - 遊び
    pub LOD2_play: f32,
}

impl Paramdef for GRASS_LOD_RANGE_PARAM_ST {
    const NAME: &'static str = "GRASS_LOD_RANGE_PARAM_ST";
    const VERSION: u16 = 1;
}
