/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ESTUS_FLASK_RECOVERY_PARAM_ST {

	/// NAME: host - ホスト
	/// DESC: Number of host est recovery - ホストのエスト回復数
	pub host:u8,

	/// NAME: No intrusion route_orb_ - 侵入経路_オーブ_なし
	/// DESC: The number of est recovery of the power of the orb whose invasion route is - 侵入経路がオーブの勢力のエスト回復数
	pub invadeOrb_None:u8,

	/// NAME: Invasion route_Orb_Sun - 侵入経路_オーブ_太陽
	/// DESC: The number of est recovery of the power of the orb whose invasion route is - 侵入経路がオーブの勢力のエスト回復数
	pub invadeOrb_Umbasa:u8,

	/// NAME: Intrusion route_Orb_Berserker - 侵入経路_オーブ_バーサーカー
	/// DESC: The number of est recovery of the power of the orb whose invasion route is - 侵入経路がオーブの勢力のエスト回復数
	pub invadeOrb_Berserker:u8,

	/// NAME: Invasion route_Orb_Sinner - 侵入経路_オーブ_罪人
	/// DESC: The number of est recovery of the power of the orb whose invasion route is - 侵入経路がオーブの勢力のエスト回復数
	pub invadeOrb_Sinners:u8,

	/// NAME: No intrusion route_sign_ - 侵入経路_サイン_なし
	/// DESC: The number of est recovery of the power whose invasion route is a sign - 侵入経路がサインの勢力のエスト回復数
	pub invadeSign_None:u8,

	/// NAME: Invasion route_sign_sun - 侵入経路_サイン_太陽
	/// DESC: The number of est recovery of the power whose invasion route is a sign - 侵入経路がサインの勢力のエスト回復数
	pub invadeSign_Umbasa:u8,

	/// NAME: Intrusion route_sign_berserker - 侵入経路_サイン_バーサーカー
	/// DESC: The number of est recovery of the power whose invasion route is a sign - 侵入経路がサインの勢力のエスト回復数
	pub invadeSign_Berserker:u8,

	/// NAME: Intrusion route_sign_sinner - 侵入経路_サイン_罪人
	/// DESC: The number of est recovery of the power whose invasion route is a sign - 侵入経路がサインの勢力のエスト回復数
	pub invadeSign_Sinners:u8,

	/// NAME: Invasion route_ring_sinner - 侵入経路_指輪_罪人
	/// DESC: The number of est recovery of the power of the ring whose invasion route is - 侵入経路が指輪の勢力のエスト回復数
	pub invadeRing_Sinners:u8,

	/// NAME: Invasion route_Ring_Boss guard (Rosalia) - 侵入経路_指輪_ボス守(ロザリア)
	/// DESC: The number of est recovery of the power of the ring whose invasion route is - 侵入経路が指輪の勢力のエスト回復数
	pub invadeRing_Rosalia:u8,

	/// NAME: Invasion route_Ring_Map Mamoru (Forest) - 侵入経路_指輪_マップ守(森)
	/// DESC: The number of est recovery of the power of the ring whose invasion route is - 侵入経路が指輪の勢力のエスト回復数
	pub invadeRing_Forest:u8,

	/// NAME: Cooperation route_sign_ None - 協力経路_サイン_なし
	/// DESC: The number of est recovery of the power whose cooperation route is a sign - 協力経路がサインの勢力のエスト回復数
	pub coopSign_None:u8,

	/// NAME: Cooperation route_sign_sun - 協力経路_サイン_太陽
	/// DESC: The number of est recovery of the power whose cooperation route is a sign - 協力経路がサインの勢力のエスト回復数
	pub coopSign_Umbasa:u8,

	/// NAME: Cooperation route_sign_berserker - 協力経路_サイン_バーサーカー
	/// DESC: The number of est recovery of the power whose cooperation route is a sign - 協力経路がサインの勢力のエスト回復数
	pub coopSign_Berserker:u8,

	/// NAME: Cooperation route_sign_sinner - 協力経路_サイン_罪人
	/// DESC: The number of est recovery of the power whose cooperation route is a sign - 協力経路がサインの勢力のエスト回復数
	pub coopSign_Sinners:u8,

	/// NAME: Cooperation route _ ring _ red hunting - 協力経路_指輪 _赤狩り
	/// DESC: Cooperation route is the number of est recovery of the power of the ring - 協力経路が指輪の勢力のエスト回復数
	pub coopRing_RedHunter:u8,

	/// NAME: Intrusion route_Ring_Map guard (Anor) - 侵入経路_指輪_マップ守(アノール)
	/// DESC: The number of est recovery of the power of the ring whose invasion route is - 侵入経路が指輪の勢力のエスト回復数
	pub invadeRing_Anor:u8,

	/// NAME: Recovery number Parameter replacement rate - 回復数パラメータ差し替え率
	/// DESC: Recovery number Parameter replacement rate - 回復数パラメータ差し替え率
	pub paramReplaceRate:u16,

	/// NAME: Recovery number Parameter replacement destination ID - 回復数パラメータ差し替え先ID
	/// DESC: Recovery number Parameter replacement destination ID - 回復数パラメータ差し替え先ID
	pub paramReplaceId:i32,

	/// NAME: pad - pad
	pub pad:[u8;8],
}

impl Paramdef for ESTUS_FLASK_RECOVERY_PARAM_ST {
const NAME: &'static str = "ESTUS_FLASK_RECOVERY_PARAM_ST";
const VERSION: u16 = 2;
}
