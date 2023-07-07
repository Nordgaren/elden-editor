/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GAME_INFO_PARAM {

	/// NAME: Title MsgID - タイトルのMsgID
	/// DESC: Title name - タイトル名
	pub titleMsgId:i32,

	/// NAME: Content MsgID - 内容のMsgID
	/// DESC: Contents - 内容
	pub contentMsgId:i32,

	/// NAME: price - 価格
	/// DESC: price - 価格
	pub value:i32,

	/// NAME: Sort ID - ソートID
	/// DESC: Sort ID - ソートID
	pub sortId:i32,

	/// NAME: Action ID - アクションID
	/// DESC: This is the action ID that determines the sales status. - 販売状況を判断するアクションIDです。
	pub eventId:i32,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub Pad:[u8;12],
}

