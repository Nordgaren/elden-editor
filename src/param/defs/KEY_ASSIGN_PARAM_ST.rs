/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct KEY_ASSIGN_PARAM_ST {

	/// NAME: pad - パッド
	/// DESC: Pad (physical key) - パッド（物理キー）
	pub padKeyId:i32,

	/// NAME: Keyboard modifier - キーボード修飾
	/// DESC: Keyboard modifier keys - キーボード修飾キー
	pub keyboardModifyKey:i32,

	/// NAME: keyboard - キーボード
	/// DESC: Keyboard (physical keys) - キーボード（物理キー）
	pub keyboardKeyId:i32,

	/// NAME: Mouse modification - マウス修飾
	/// DESC: Mouse modifier key - マウス修飾キー
	pub mouseModifyKey:i32,

	/// NAME: mouse - マウス
	/// DESC: Mouse (physical key) - マウス（物理キー）
	pub mouseKeyId:i32,

	/// NAME: ---- - ----
	pub reserved:[u8;12],
}

