/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {

	/// NAME: Activation level (start) - 発動レベル(開始)
	/// DESC: Drawing distance scale activation level (start) - 描画距離スケール発動レベル(開始)
	pub DrawDist_LvBegin:u8,

	/// NAME: Activation level (end) - 発動レベル(終了)
	/// DESC: Drawing distance scale activation level (completed) - 描画距離スケール発動レベル(修了)
	pub DrawDist_LvEnd:u8,

	/// NAME: Reserve - 予備
	/// DESC: Reserve - 予備
	pub reserve0:[u8;2],

	/// NAME: Scale (start) - スケール（開始）
	/// DESC: Drawing distance scale (start) - 描画距離スケール（開始）
	pub DrawDist_ScaleBegin:f32,

	/// NAME: Scale (end) - スケール（終了）
	/// DESC: Drawing distance scale (completed) - 描画距離スケール（修了）
	pub DrawDist_ScaleEnd:f32,

	/// NAME: Activation level (start) - 発動レベル(開始)
	/// DESC: Shadow drawing distance scale activation level (start) - 影描画距離スケール発動レベル(開始)
	pub ShadwDrawDist_LvBegin:u8,

	/// NAME: Activation level (end) - 発動レベル(終了)
	/// DESC: Shadow drawing distance scale activation level (completed) - 影描画距離スケール発動レベル(修了)
	pub ShadwDrawDist_LvEnd:u8,

	/// NAME: Reserve - 予備
	/// DESC: Reserve - 予備
	pub reserve1:[u8;2],

	/// NAME: Scale (start) - スケール（開始）
	/// DESC: Shadow drawing distance scale (start) - 影描画距離スケール（開始）
	pub ShadwDrawDist_ScaleBegin:f32,

	/// NAME: Scale (end) - スケール（終了）
	/// DESC: Shadow drawing distance scale (completed) - 影描画距離スケール（修了）
	pub ShadwDrawDist_ScaleEnd:f32,

	/// NAME: Reserve - 予備
	/// DESC: Reserve - 予備
	pub reserve2:[u8;24],
}

impl Paramdef for LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {
const NAME: &'static str = "LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST";
const VERSION: u16 = 0;
}
