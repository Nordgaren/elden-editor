/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NPC_AI_ACTION_PARAM_ST {

	/// NAME: Move direction input - 移動方向入力
	/// DESC: Moving direction to enter - 入力する移動方向
	pub moveDir:u8,

	/// NAME: Key input 1 - キー入力1
	/// DESC: Key to enter - 入力するキー
	pub key1:u8,

	/// NAME: Key input 2 - キー入力2
	/// DESC: Key to enter - 入力するキー
	pub key2:u8,

	/// NAME: Key input 3 - キー入力3
	/// DESC: Key to enter - 入力するキー
	pub key3:u8,

	/// NAME: Press and hold the movement direction input? - 移動方向入力は長押し？
	/// DESC: Whether to handle the input movement direction as long press - 入力する移動方向を長押し扱いするか
	pub bMoveDirHold:u8,

	/// NAME: Press and hold key input 1? - キー入力1は長押し？
	/// DESC: Whether to treat the key to be entered as a long press - 入力するキーを長押扱いするか
	pub bKeyHold1:u8,

	/// NAME: Press and hold key input 2? - キー入力2は長押し？
	/// DESC: Whether to treat the key to be entered as a long press - 入力するキーを長押扱いするか
	pub bKeyHold2:u8,

	/// NAME: Press and hold key input 3? - キー入力3は長押し？
	/// DESC: Whether to treat the key to be entered as a long press - 入力するキーを長押扱いするか
	pub bKeyHold3:u8,

	/// NAME: Gesture ID (valid only when any key input is GESTURE) - ジェスチャーID（どれかのキー入力がGESTUREの時のみ有効）
	/// DESC: Gesture ID - ジェスチャーID
	pub gestureId:i32,

	/// NAME: Treat as successful when life is over - 寿命が尽きた時成功扱いにする
	/// DESC: If this is ON, the AI goal will not be successful until the end of its life - これがONならAIのゴールが寿命まで成功にならない
	pub bLifeEndSuccess:u8,

	/// NAME: pad - パッド
	/// DESC: pad - pad
	pub pad1:[u8;3],
}

impl Paramdef for NPC_AI_ACTION_PARAM_ST {
const NAME: &'static str = "NPC_AI_ACTION_PARAM_ST";
const VERSION: u16 = 1;
}
