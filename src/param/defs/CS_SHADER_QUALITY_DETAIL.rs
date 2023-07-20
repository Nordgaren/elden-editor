/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_SHADER_QUALITY_DETAIL {

	/// NAME: SSS enabled - SSS有効
	/// DESC: SSS enabled - SSS有効
	pub sssEnabled:u8,

	/// NAME: Tessellation enabled - テッセレーション有効
	/// DESC: Tessellation enabled - テッセレーション有効
	pub tessellationEnabled:u8,

	/// NAME: High precision normal effective - 高精度ノーマル有効
	/// DESC: High precision normal valid (setting the accuracy of the normal stored in G-Buffer) - 高精度ノーマル有効(G-Bufferに格納する法線の精度の設定)
	pub highPrecisionNormalEnabled:u8,

	/// NAME: dmy - dmy
	pub dmy:[u8;1],
}

impl Paramdef for CS_SHADER_QUALITY_DETAIL {
const NAME: &'static str = "CS_SHADER_QUALITY_DETAIL";
const VERSION: u16 = 1;
}
