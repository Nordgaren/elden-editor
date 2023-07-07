/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MENU_VALUE_TABLE_SPEC {

	/// NAME: Value to compare - 比較する値
	/// DESC: Value to compare - 比較する値
	pub value:i32,

	/// NAME: Converted text ID - 変換後のテキストID
	/// DESC: Converted text ID - 変換後のテキストID
	pub textId:i32,

	/// NAME: Comparison type - 比較タイプ
	/// DESC: Comparison type - 比較タイプ
	pub compareType:i8,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub padding:[u8;3],
}

