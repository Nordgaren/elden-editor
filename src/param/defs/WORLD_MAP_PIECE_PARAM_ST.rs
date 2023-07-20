/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WORLD_MAP_PIECE_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Open event flag ID - 開放イベントフラグID
	/// DESC: Event flag ID of open condition - 開放条件のイベントフラグID
	pub openEventFlagId:u32,

	/// NAME: Opened traversal area: Xmin - 開放される踏破エリア：Xmin
	/// DESC: Coordinates of the traversal area that expands when opened (Xmin) - 開放時に拡張する踏破エリアの座標（Xmin）
	pub openTravelAreaLeft:f32,

	/// NAME: Opened traversal area: Xmax - 開放される踏破エリア：Xmax
	/// DESC: Coordinates of the traversal area that expands when opened (Xmax) - 開放時に拡張する踏破エリアの座標（Xmax）
	pub openTravelAreaRight:f32,

	/// NAME: Opened traversal area: Ymin - 開放される踏破エリア：Ymin
	/// DESC: Coordinates of the traversal area that expands when opened (Ymin) - 開放時に拡張する踏破エリアの座標（Ymin）
	pub openTravelAreaTop:f32,

	/// NAME: Opened traversal area: Ymax - 開放される踏破エリア：Ymax
	/// DESC: Coordinates of the traversal area that expands when opened (Ymax) - 開放時に拡張する踏破エリアの座標（Ymax）
	pub openTravelAreaBottom:f32,

	/// NAME: Acquisition production event flag ID - 入手演出イベントフラグID
	/// DESC: Event flag ID of the acquisition production start condition. Assuming that only one of the map fragments is On - 入手演出開始条件のイベントフラグID。いずれかの地図断片ひとつのみがOnになっている想定
	pub acquisitionEventFlagId:u32,

	/// NAME: Acquisition effect: Display magnification - 入手演出：表示倍率
	/// DESC: Display magnification at the time of acquisition production - 入手演出時の表示倍率
	pub acquisitionEventScale:f32,

	/// NAME: Obtained production: Center coordinates X - 入手演出：中心座標X
	/// DESC: Center coordinates (X) at the time of acquisition production - 入手演出時の中心座標（X）
	pub acquisitionEventCenterX:f32,

	/// NAME: Obtained production: Center coordinates Y - 入手演出：中心座標Y
	/// DESC: For the central seat at the time of acquisition production (Y) - 入手演出時の中心座用（Y）
	pub acquisitionEventCenterY:f32,

	/// NAME: Acquisition effect: Resource multiplier - 入手演出：リソース倍率
	/// DESC: Display magnification of blindfold resources for acquisition production - 入手演出用目隠しリソースの表示倍率
	pub acquisitionEventResScale:f32,

	/// NAME: Acquisition effect: Resource offset X - 入手演出：リソースオフセットX
	/// DESC: Display position offset (X) of blindfold resource for acquisition production - 入手演出用目隠しリソースの表示位置オフセット（X）
	pub acquisitionEventResOffsetX:f32,

	/// NAME: Acquisition effect: Resource offset Y - 入手演出：リソースオフセットY
	/// DESC: Offset of display position of blindfold resource for acquisition production (Y) - 入手演出用目隠しリソースの表示位置オフセット（Y）
	pub acquisitionEventResOffsetY:f32,

	/// NAME: pad - パッド
	pub pad:[u8;12],
}

impl Paramdef for WORLD_MAP_PIECE_PARAM_ST {
const NAME: &'static str = "WORLD_MAP_PIECE_PARAM_ST";
const VERSION: u16 = 2;
}
impl WORLD_MAP_PIECE_PARAM_ST {
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
