/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_MOTION_BLUR_QUALITY_DETAIL {

	/// NAME: Motion blur enabled - モーションブラー有効
	/// DESC: Motion blur enabled - モーションブラー有効
	pub enabled:u8,

	/// NAME: OMB (object motion blur) enabled - OMB(オブジェクトモーションブラー)有効
	/// DESC: OMB (object motion blur) enabled - OMB(オブジェクトモーションブラー)有効
	pub ombEnabled:u8,

	/// NAME: Decrease the resolution of the velocity buffer used internally - 内部で使用するベロシティバッファの解像度を下げる
	/// DESC: Decrease the resolution of the velocity buffer used internally. Not effective if you are not using precision velocity buffers - 内部で使用するベロシティバッファの解像度を下げる。高精度ベロシティバッファを使っていない場合は効果ない
	pub forceScaleVelocityBuffer:u8,

	/// NAME: Normally processed by the Reconstruction filter, but downgraded to a lighter process - 通常、Reconstructionフィルタで処理されるが、軽い処理にダウングレードする
	/// DESC: Normally processed by the Reconstruction filter, but downgraded to a lighter process - 通常、Reconstructionフィルタで処理されるが、軽い処理にダウングレードする
	pub cheapFilterMode:u8,

	/// NAME: Give an offset to the sample count - サンプルカウントにオフセットを与える
	/// DESC: Give an offset to the sample count * Set to a multiple of 2 - サンプルカウントにオフセットを与える※2の倍数に設定して下さい
	pub sampleCountBias:i32,

	/// NAME: Offset the number of recursive blurs - 再帰ブラー回数にオフセットを与える
	/// DESC: Give an offset to the number of recursive blurs - 再帰ブラー回数にオフセットを与える
	pub recurrenceCountBias:i32,

	/// NAME: Scale value for blur maximum length parameter - ブラー最大長さパラメータに対するスケール値
	/// DESC: Scale value for blur maximum length parameter - ブラー最大長さパラメータに対するスケール値
	pub blurMaxLengthScale:f32,
}

impl Paramdef for CS_MOTION_BLUR_QUALITY_DETAIL {
const NAME: &'static str = "CS_MOTION_BLUR_QUALITY_DETAIL";
const VERSION: u16 = 2;
}
