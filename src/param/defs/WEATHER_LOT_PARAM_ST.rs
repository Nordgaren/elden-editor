/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 5
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WEATHER_LOT_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Weather type 0 - 天候種類0
	/// DESC: Weather type 0 - 天候種類0
	pub weatherType0:i16,

	/// NAME: Weather type 1 - 天候種類1
	/// DESC: Weather type 1 - 天候種類1
	pub weatherType1:i16,

	/// NAME: Weather type 2 - 天候種類2
	/// DESC: Weather type 2 - 天候種類2
	pub weatherType2:i16,

	/// NAME: Weather type 3 - 天候種類3
	/// DESC: Weather type 3 - 天候種類3
	pub weatherType3:i16,

	/// NAME: Weather type 4 - 天候種類4
	/// DESC: Weather type 4 - 天候種類4
	pub weatherType4:i16,

	/// NAME: Weather type 5 - 天候種類5
	/// DESC: Weather type 5 - 天候種類5
	pub weatherType5:i16,

	/// NAME: Weather type 6 - 天候種類6
	/// DESC: Weather type 6 - 天候種類6
	pub weatherType6:i16,

	/// NAME: Weather type 7 - 天候種類7
	/// DESC: Weather type 7 - 天候種類7
	pub weatherType7:i16,

	/// NAME: Weather type 8 - 天候種類8
	/// DESC: Weather type 8 - 天候種類8
	pub weatherType8:i16,

	/// NAME: Weather type 9 - 天候種類9
	/// DESC: Weather type 9 - 天候種類9
	pub weatherType9:i16,

	/// NAME: Weather type 10 - 天候種類10
	/// DESC: Weather type 10 - 天候種類10
	pub weatherType10:i16,

	/// NAME: Weather type 11 - 天候種類11
	/// DESC: Weather type 11 - 天候種類11
	pub weatherType11:i16,

	/// NAME: Weather type 12 - 天候種類12
	/// DESC: Weather type 12 - 天候種類12
	pub weatherType12:i16,

	/// NAME: Weather type 13 - 天候種類13
	/// DESC: Weather type 13 - 天候種類13
	pub weatherType13:i16,

	/// NAME: Weather type 14 - 天候種類14
	/// DESC: Weather type 14 - 天候種類14
	pub weatherType14:i16,

	/// NAME: Weather type 15 - 天候種類15
	/// DESC: Weather type 15 - 天候種類15
	pub weatherType15:i16,

	/// NAME: Lottery weight 0 - 抽選ウェイト0
	/// DESC: Lottery weight 0 - 抽選ウェイト0
	pub lotteryWeight0:u16,

	/// NAME: Lottery weight 1 - 抽選ウェイト1
	/// DESC: Lottery weight 1 - 抽選ウェイト1
	pub lotteryWeight1:u16,

	/// NAME: Lottery weight 2 - 抽選ウェイト2
	/// DESC: Lottery weight 2 - 抽選ウェイト2
	pub lotteryWeight2:u16,

	/// NAME: Lottery weight 3 - 抽選ウェイト3
	/// DESC: Lottery weight 3 - 抽選ウェイト3
	pub lotteryWeight3:u16,

	/// NAME: Lottery weight 4 - 抽選ウェイト4
	/// DESC: Lottery weight 4 - 抽選ウェイト4
	pub lotteryWeight4:u16,

	/// NAME: Lottery weight 5 - 抽選ウェイト5
	/// DESC: Lottery weight 5 - 抽選ウェイト5
	pub lotteryWeight5:u16,

	/// NAME: Lottery weight 6 - 抽選ウェイト6
	/// DESC: Lottery weight 6 - 抽選ウェイト6
	pub lotteryWeight6:u16,

	/// NAME: Lottery weight 7 - 抽選ウェイト7
	/// DESC: Lottery weight 7 - 抽選ウェイト7
	pub lotteryWeight7:u16,

	/// NAME: Lottery weight 8 - 抽選ウェイト8
	/// DESC: Lottery weight 8 - 抽選ウェイト8
	pub lotteryWeight8:u16,

	/// NAME: Lottery weight 9 - 抽選ウェイト9
	/// DESC: Lottery weight 9 - 抽選ウェイト9
	pub lotteryWeight9:u16,

	/// NAME: Lottery weight 10 - 抽選ウェイト10
	/// DESC: Lottery weight 10 - 抽選ウェイト10
	pub lotteryWeight10:u16,

	/// NAME: Lottery weight 11 - 抽選ウェイト11
	/// DESC: Lottery weight 11 - 抽選ウェイト11
	pub lotteryWeight11:u16,

	/// NAME: Lottery weight 12 - 抽選ウェイト12
	/// DESC: Lottery weight 12 - 抽選ウェイト12
	pub lotteryWeight12:u16,

	/// NAME: Lottery weight 13 - 抽選ウェイト13
	/// DESC: Lottery weight 13 - 抽選ウェイト13
	pub lotteryWeight13:u16,

	/// NAME: Lottery weight 14 - 抽選ウェイト14
	/// DESC: Lottery weight 14 - 抽選ウェイト14
	pub lotteryWeight14:u16,

	/// NAME: Lottery weight 15 - 抽選ウェイト15
	/// DESC: Lottery weight 15 - 抽選ウェイト15
	pub lotteryWeight15:u16,

	/// NAME: Time zone condition list - 時間帯条件リスト
	/// DESC: Time zone condition list - 時間帯条件リスト
	pub timezoneLimit:u8,

	/// NAME: Direct time specification_start_hour - 直接時間指定_開始_時
	/// DESC: Direct time specification_start_hour - 直接時間指定_開始_時
	pub timezoneStartHour:u8,

	/// NAME: Direct time specification_start_minute - 直接時間指定_開始_分
	/// DESC: Direct time specification_start_minute - 直接時間指定_開始_分
	pub timezoneStartMinute:u8,

	/// NAME: Direct time specification_start_hour - 直接時間指定_開始_時
	/// DESC: Direct time specification_start_hour - 直接時間指定_開始_時
	pub timezoneEndHour:u8,

	/// NAME: Direct time specification_start_minute - 直接時間指定_開始_分
	/// DESC: Direct time specification_start_minute - 直接時間指定_開始_分
	pub timezoneEndMinute:u8,

	/// NAME: Reserve - リザーブ
	/// DESC: Reserved area - 予約領域
	pub reserve:[u8;9],
}

impl Paramdef for WEATHER_LOT_PARAM_ST {
const NAME: &'static str = "WEATHER_LOT_PARAM_ST";
const VERSION: u16 = 5;
}
impl WEATHER_LOT_PARAM_ST {
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
