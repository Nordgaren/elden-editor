/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Lifting flag - 解禁フラグ
	/// DESC: Lifting flag (default: 0). If this event flag is set, the ban will be lifted (displayed on the loading screen). If it is 0, the ban is always lifted. The invalid flag has priority - 解禁フラグ(default: 0)。このイベントフラグが立っていれば解禁される（ローディング画面に表示される）。0なら常に解禁されている。無効フラグの方が優先される
	pub unlockFlagId:u32,

	/// NAME: Invalid flag - 無効フラグ
	/// DESC: Invalid flag (default: 0). If this event flag is set, it will be disabled (it will not be displayed on the loading screen). If it is 0, this flag is not always set. - 無効フラグ(default: 0)。このイベントフラグが立っていると無効化（ローディング画面に表示されなくなる）。0なら常にこのフラグは立っていないものとする
	pub invalidFlagId:u32,

	/// NAME: Text ID - テキストID
	/// DESC: Text ID (Loading Text.xlsx). Used for both loading titles and loading text - テキストID(Loading Text.xlsx)。ローディングタイトルとローディングテキスト両方に使われる
	pub msgId:i32,
}

impl Paramdef for KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {
const NAME: &'static str = "KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST";
const VERSION: u16 = 1;
}
impl KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {
	/// Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	/// Bitfield1
	pub fn get_disableParam_NT(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_disableParam_NT(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Reserve for package output 1 - パッケージ出力用リザーブ1
	/// Bitfield1
	pub fn get_disableParamReserve1(&self) -> u8 {
		&self.Bitfield1 & 0xFE
	}

	/// Bitfield1 MAX: 127
	pub fn set_disableParamReserve1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield1 & 0x1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1
		}
	}
}
