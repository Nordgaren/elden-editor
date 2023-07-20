/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_SHADOW_QUALITY_DETAIL {

	/// NAME: Shadow is enabled - シャドウが有効
	/// DESC: Shadow is enabled - シャドウが有効
	pub enabled:u8,

	/// NAME: Maximum filter quality allowed - 許可される最大のフィルタ品質
	/// DESC: Maximum filter quality allowed - 許可される最大のフィルタ品質
	pub maxFilterLevel:u8,

	/// NAME: dmy - dmy
	pub dmy:[u8;2],

	/// NAME: Scaler with set shadow map resolution - 設定されたシャドウマップ解像度のスケーラ
	/// DESC: Scaler with set shadow map resolution - 設定されたシャドウマップ解像度のスケーラ
	pub textureSizeScaler:u32,

	/// NAME: Divide the set shadow map resolution - 設定されたシャドウマップ解像度を除算
	/// DESC: Divide the set shadow map resolution - 設定されたシャドウマップ解像度を除算
	pub textureSizeDivider:u32,

	/// NAME: Minimum resolution - 解像度最小
	/// DESC: Clamp resolution - 解像度をクランプ
	pub textureMinSize:u32,

	/// NAME: Maximum resolution - 解像度最大
	/// DESC: Clamp the resolution. It will be the resolution judgment for each cascade - 解像度をクランプ。カスケード毎の解像度判定になります
	pub textureMaxSize:u32,

	/// NAME: Blur count bias - ブラーカウントバイアス
	/// DESC: Blur count bias (set count bias, unchanged at 0) - ブラーカウントバイアス(設定されたカウントのバイアス。0で変更なし)
	pub blurCountBias:i32,
}

impl Paramdef for CS_SHADOW_QUALITY_DETAIL {
const NAME: &'static str = "CS_SHADOW_QUALITY_DETAIL";
const VERSION: u16 = 2;
}
