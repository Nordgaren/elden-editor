/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EVENT_FLAG_USAGE_PARAM_ST {

	/// NAME: Use - 用途
	/// DESC: Use of the flag. - フラグの用途。
	pub usageType:u8,

	/// NAME: Playlog category - プレイログカテゴリ
	/// DESC: Valid only when the usage is "ON / OFF". If this is set, the play log will be collected when the flag is turned ON. - 用途が「ON/OFF」の場合のみ有効。これを設定するとフラグがONになったときにプレイログを収集する。
	pub playlogCategory:u8,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub padding1:[u8;2],

	/// NAME: Number of secured flags - 確保フラグ数
	/// DESC: Set to 1 for "ON / OFF". In the case of "frame allocation" and "integer", "parameter number-parameter number + number of secured flags-1" is secured. - 「ON/OFF」の場合は1を設定する。「枠割り当て」「整数」の場合は「パラメータ番号～パラメータ番号+確保フラグ数-1」が確保される範囲になる。
	pub flagNum:i32,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub padding2:[u8;24],
}

impl Paramdef for EVENT_FLAG_USAGE_PARAM_ST {
const NAME: &'static str = "EVENT_FLAG_USAGE_PARAM_ST";
const VERSION: u16 = 1;
}
