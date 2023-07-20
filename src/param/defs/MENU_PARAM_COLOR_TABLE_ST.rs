/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MENU_PARAM_COLOR_TABLE_ST {
    /// NAME: Interpolation method - 補間方法
    /// DESC: Interpolation method - 補間方法
    pub lerpMode: u8,

    /// NAME: pad - パッド
    pub pad1: [u8; 3],

    /// NAME: Hue - 色相
    /// DESC: Hue. Treat as a fixed value in interpolation - 色相。補間では固定値として扱う
    pub h: u16,

    /// NAME: pad - パッド
    pub pad2: [u8; 2],

    /// NAME: saturation - 彩度
    /// DESC: Saturation 1. Treated as the first point of interpolation - 彩度1。補間の1点目として扱われる
    pub s1: f32,

    /// NAME: brightness - 明度
    /// DESC: Brightness 1. Treated as the first point of interpolation - 明度1。補間の1点目として扱われる
    pub v1: f32,

    /// NAME: saturation - 彩度
    /// DESC: Saturation 2. Treated as the second point of interpolation - 彩度2。補間の2点目として扱われる
    pub s2: f32,

    /// NAME: brightness - 明度
    /// DESC: Brightness 2. Treated as the second point of interpolation - 明度2。補間の2点目として扱われる
    pub v2: f32,

    /// NAME: saturation - 彩度
    /// DESC: Saturation 3. Treated as the third point of interpolation - 彩度3。補間の3点目として扱われる
    pub s3: f32,

    /// NAME: brightness - 明度
    /// DESC: Brightness 3. Treated as the third point of interpolation - 明度3。補間の3点目として扱われる
    pub v3: f32,
}

impl Paramdef for MENU_PARAM_COLOR_TABLE_ST {
    const NAME: &'static str = "MENU_PARAM_COLOR_TABLE_ST";
    const VERSION: u16 = 2;
}
