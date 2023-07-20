/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CUTSCENE_GPARAM_TIME_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Morning - 朝
	/// DESC: Morning (Refer to the cutscene time conversion setting sheet for the conversion time) - 朝(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_Morning:u8,

	/// NAME: Noon A - 昼A
	/// DESC: Day A (Refer to the cutscene time conversion setting sheet for the conversion time) - 昼A(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_Noon:u8,

	/// NAME: Noon B - 昼B
	/// DESC: Noon-B (Refer to the cutscene time conversion setting sheet for the conversion time) - 昼B(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_AfterNoon:u8,

	/// NAME: evening - 夕
	/// DESC: Evening (Refer to the cutscene time conversion setting sheet for the conversion time) - 夕(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_Evening:u8,

	/// NAME: Night - 夜
	/// DESC: Night (Refer to the cutscene time conversion setting sheet for the conversion time) - 夜(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_Night:u8,

	/// NAME: Midnight A - 深夜A
	/// DESC: Midnight A (Refer to the cutscene time conversion setting sheet for the conversion time) - 深夜A(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_DeepNightA:u8,

	/// NAME: Midnight B - 深夜B
	/// DESC: Midnight B (Refer to the cutscene time conversion setting sheet for the conversion time) - 深夜B(変換時刻はカットシーン時間変換設定シートを参照）
	pub DstTimezone_DeepNightB:u8,

	/// NAME: reserved - reserved
	/// DESC: reserved - reserved
	pub reserved:[u8;1],

	/// NAME: Specify in-game time at the end of playback [hour] - 再生終了時のインゲーム時刻指定[hour]
	/// DESC: Specify in-game time at the end of playback [hour] [-1.0 to 24.0] (-1 (less than 0): do nothing) - 再生終了時のインゲーム時刻指定[hour][-1.0～24.0](-1(0より小さい)：何もしない)
	pub PostPlayIngameTime:f32,
}

impl Paramdef for CUTSCENE_GPARAM_TIME_PARAM_ST {
const NAME: &'static str = "CUTSCENE_GPARAM_TIME_PARAM_ST";
const VERSION: u16 = 3;
}
impl CUTSCENE_GPARAM_TIME_PARAM_ST {
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
	/// Parameters marked with a circle are excluded from all packages (because they are for debugging). - ○をつけたパラメータは全パッケージから除外します（デバッグ用なので）
	/// Bitfield1
	pub fn get_disableParam_Debug(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_disableParam_Debug(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// Reserve for package output 1 - パッケージ出力用リザーブ1
	/// Bitfield1
	pub fn get_disableParamReserve1(&self) -> u8 {
		&self.Bitfield1 & 0xFC
	}

	/// Bitfield1 MAX: 63
	pub fn set_disableParamReserve1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 2) & 0xFC;
			let newVal = &self.Bitfield1 & 0x3 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x3
		}
	}
}
