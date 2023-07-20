/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 202
#[repr(C)]
pub struct PERFORMANCE_CHECK_PARAM {
    /// NAME: Report destination_Job type tag - 報告先_職種タグ
    /// DESC: Report destination_Job type tag - 報告先_職種タグ
    pub workTag: u8,

    /// NAME: Report destination_category tag - 報告先_カテゴリタグ
    /// DESC: Report destination_category tag - 報告先_カテゴリタグ
    pub categoryTag: u8,

    /// NAME: Comparison symbol - 比較記号
    /// DESC: Comparison symbol - 比較記号
    pub compareType: u8,

    /// NAME: Reservation 1 - 予約1
    /// DESC: Reservation 1 - 予約1
    pub dummy1: [u8; 1],

    /// NAME: Comparison value - 比較数値
    /// DESC: Comparison value - 比較数値
    pub compareValue: f32,

    /// NAME: Reservation 2 - 予約2
    /// DESC: Reservation 2 - 予約2
    pub dummy2: [u8; 8],

    /// NAME: Report destination_user tag - 報告先_userタグ
    /// DESC: Report to_Performance person tag - 報告先_パフォーマンス人物タグ
    pub userTag: [u16; 16],
}

impl Paramdef for PERFORMANCE_CHECK_PARAM {
    const NAME: &'static str = "PERFORMANCE_CHECK_PARAM";
    const VERSION: u16 = 1;
}
