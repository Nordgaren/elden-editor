/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_LIGHTING_QUALITY_DETAIL {

	/// NAME: Local light effective distance coefficient - ローカルライト有効距離係数
	/// DESC: Local light effective distance coefficient (smaller, it disappears at a short distance) - ローカルライト有効距離係数(小さくすると、近い距離で消える)
	pub localLightDistFactor:f32,

	/// NAME: Local light shadow enabled - ローカルライトシャドウ有効
	/// DESC: Local light shadow enabled - ローカルライトシャドウ有効
	pub localLightShadowEnabled:u8,

	/// NAME: Forward pass writing enabled - フォワードパスライティング有効
	/// DESC: Forward pass writing enabled - フォワードパスライティング有効
	pub forwardPassLightingEnabled:u8,

	/// NAME: Local light shadow spec level - ローカルライトシャドウスペックレベル
	/// DESC: Local light shadow spec level. The larger the value, the more light sources will be shadowed. - ローカルライトシャドウスペックレベル。大きいほど、より多くの光源にシャドウが設定される
	pub localLightShadowSpecLevelMax:u8,

	/// NAME: dmy - dmy
	pub dmy:[u8;1],
}

