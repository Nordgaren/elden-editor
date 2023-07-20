/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GESTURE_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Reference item ID - 参照アイテムID
	/// DESC: Reference item ID. Used to bring in the gesture text ID, icon ID, and sort ID for each menu. Register the tool ID of the equipment parameter - 参照アイテムID。各メニューでのジェスチャのテキストID、アイコンID、ソートIDを持ってくるのに使用される。装備品パラメータの道具IDを登録します
	pub itemId:i32,

	/// NAME: Anime ID for message attachment - メッセージ添付用アニメID
	/// DESC: Anime ID for attaching messages. Specify the animation ID when attaching a message - メッセージ添付用アニメID。メッセージ添付時のアニメIDを指定します
	pub msgAnimId:i32,

	/// NAME: Is it prohibited to use while riding? - 騎乗中使用禁止か
	/// DESC: Is it prohibited to use while riding (default: ×)? If ○, it cannot be used while riding - 騎乗中使用禁止か(デフォルト:×)。○なら騎乗中に使用できない
	pub Bitfield2:u8,

	/// NAME: Reserved area - 予約領域
	pub pad1:[u8;3],
}

impl Paramdef for GESTURE_PARAM_ST {
const NAME: &'static str = "GESTURE_PARAM_ST";
const VERSION: u16 = 2;
}
impl GESTURE_PARAM_ST {
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
	}	/// Is it prohibited to use while riding (default: ×)? If ○, it cannot be used while riding - 騎乗中使用禁止か(デフォルト:×)。○なら騎乗中に使用できない
	/// Bitfield2
	pub fn get_cannotUseRiding(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_cannotUseRiding(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// 
	/// Bitfield2
	pub fn get_pad2(&self) -> u8 {
		&self.Bitfield2 & 0xFE
	}

	/// Bitfield2 MAX: 127
	pub fn set_pad2(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield2 & 0x1 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0x1
		}
	}
}
