/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_REFLECTION_QUALITY_DETAIL {

	/// NAME: Reflective effective - 反射有効
	/// DESC: Reflective effective - 反射有効
	pub enabled:u8,

	/// NAME: Local light enabled - ローカルライト有効
	/// DESC: Local light enabled - ローカルライト有効
	pub localLightEnabled:u8,

	/// NAME: Local light forced enable - ローカルライト強制有効
	/// DESC: Local light forced enable - ローカルライト強制有効
	pub localLightForceEnabled:u8,

	/// NAME: dmy - dmy
	pub dmy:[u8;1],

	/// NAME: Resolution scale - 解像度スケール
	/// DESC: Resolution scale - 解像度スケール
	pub resolutionDivider:u32,

	/// NAME: SSR enabled - SSR有効
	/// DESC: SSR enabled - SSR有効
	pub ssrEnabled:u8,

	/// NAME: Gaussian blur permission - ガウスぼかしの許可
	/// DESC: Gaussian blur permission - ガウスぼかしの許可
	pub ssrGaussianBlurEnabled:u8,

	/// NAME: dmy - dmy
	pub dmy2:[u8;2],

	/// NAME: Calculated distance scale - 計算距離スケール
	/// DESC: Calculated distance scale - 計算距離スケール
	pub ssrDepthRejectThresholdScale:f32,

	/// NAME: Raytrace step factor (multiply by SSR parameter) - レイトレースステップ係数（SSRパラメータに乗算）
	/// DESC: Raytrace step factor (multiply by SSR parameter) - レイトレースステップ係数（SSRパラメータに乗算）
	pub ssrRayTraceStepScale:f32,

	/// NAME: Fade angle bias. High quality when made smaller - フェード角度バイアス。小さくすると高品質
	/// DESC: Fade angle bias. High quality when made smaller - フェード角度バイアス。小さくすると高品質
	pub ssrFadeToViewerBias:f32,

	/// NAME: Fresnel reject bias. High quality when made smaller - フレネルリジェクトバイアス。小さくすると高品質
	/// DESC: Fresnel reject bias. High quality when made smaller - フレネルリジェクトバイアス。小さくすると高品質
	pub ssrFresnelRejectBias:f32,
}

impl Paramdef for CS_REFLECTION_QUALITY_DETAIL {
const NAME: &'static str = "CS_REFLECTION_QUALITY_DETAIL";
const VERSION: u16 = 1;
}
