/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WEATHER_PARAM_ST {

	/// NAME: Weather SfxId (common) - 天候SfxId(共通)
	/// DESC: SfxId for weather -1: No weather Sfx Set for indoor and outdoor use. Items that can be erased with Above Shadow, such as raindrops of interactive particles, are OK here. - 天候用SfxId -1：天候Sfxなし 屋内屋外共通で出すものを設定。インタラクティブパーティクルの雨粒など、雨遮蔽(AboveShadow)で消せるものはこちらでOK
	pub SfxId:i32,

	/// NAME: Wind SfxId (outdoor) - 風SfxId(屋外)
	/// DESC: Wind SfxId -1: No wind Sfx Created only outdoors - 風SfxId -1：風Sfxなし 屋外のみ作成されます
	pub WindSfxId:i32,

	/// NAME: SfxId for ground hit effect - 地面ヒットエフェクト用SfxId
	/// DESC: SfxId for ground hit effect -1: None for ground hit effect - 地面ヒットエフェクト用SfxId -1：地面ヒットエフェクト用なし
	pub GroundHitSfxId:i32,

	/// NAME: SoundId for weather (common) - 天候用SoundId(共通)
	/// DESC: SoundId for weather -1: No Sound - 天候用SoundId -1：Soundなし
	pub SoundId:i32,

	/// NAME: Wet time (seconds) - 濡れ時間(秒)
	/// DESC: Time to get completely wet (time until the weight of m00_00_0000_WeatherBase reaches 1.0) -1: No wetness (m00_00_0000_WeatherBase remains 0.0) - 完全に濡れるまでの時間(m00_00_0000_WeatherBaseのウェイトが1.0になるまでの時間) -1：濡れなし(m00_00_0000_WeatherBaseは0.0のまま)
	pub WetTime:f32,

	/// NAME: GparamId for weather - 天候用GparamId
	/// DESC: Specify the XXX part of Gparam (m00_00_? XXX_WeatherOutdoor.gparamxml) for outdoor weather. Gparam to be used can be shared between weather. - 屋外天候用Gparam(m00_00_?XXX_WeatherOutdoor.gparamxml)のXXXの部分を指定する。天候間で使用するGparamの共有が可能。
	pub GparamId:u32,

	/// NAME: Minimum time until the next weather lottery (in-game seconds) - 次回天候抽選までの最小時間(インゲーム秒)
	/// DESC: Specify the minimum time to the next weather lottery in in-game seconds. When transitioning to this weather, the time to the next weather will be a random time between the minimum and maximum. - 次回天候抽選までの時間の最最小値をインゲーム秒単位で指定します。この天候に遷移時、次の天候までの時間が最小から最大の間のランダムな時間になります。
	pub NextLotIngameSecondsMin:u32,

	/// NAME: Maximum time until the next weather lottery (in-game seconds) - 次回天候抽選までの最大時間(インゲーム秒)
	/// DESC: Specify the maximum time until the next weather lottery in in-game seconds. When transitioning to this weather, the time to the next weather will be a random time between the minimum and maximum. - 次回天候抽選までの時間の最大値をインゲーム秒単位で指定します。この天候に遷移時、次の天候までの時間が最小から最大の間のランダムな時間になります。
	pub NextLotIngameSecondsMax:u32,

	/// NAME: Wet special effect ID_00 - 濡れ特殊効果ID_00
	/// DESC: Special effect ID on the character (-1: None) - キャラに掛かる特殊効果ID(-1：なし)
	pub WetSpEffectId00:i32,

	/// NAME: Wet special effect ID_01 - 濡れ特殊効果ID_01
	/// DESC: Special effect ID on the character (-1: None) - キャラに掛かる特殊効果ID(-1：なし)
	pub WetSpEffectId01:i32,

	/// NAME: Wet special effect ID_02 - 濡れ特殊効果ID_02
	/// DESC: Special effect ID on the character (-1: None) - キャラに掛かる特殊効果ID(-1：なし)
	pub WetSpEffectId02:i32,

	/// NAME: Wet special effect ID_03 - 濡れ特殊効果ID_03
	/// DESC: Special effect ID on the character (-1: None) - キャラに掛かる特殊効果ID(-1：なし)
	pub WetSpEffectId03:i32,

	/// NAME: Wet special effect ID_04 - 濡れ特殊効果ID_04
	/// DESC: Special effect ID on the character (-1: None) - キャラに掛かる特殊効果ID(-1：なし)
	pub WetSpEffectId04:i32,

	/// NAME: Weather SfxId (indoor) - 天候SfxId(屋内)
	/// DESC: Weather SfxId -1: No weather Sfx indoors only - 天候用SfxId -1：天候Sfxなし　屋内のみ
	pub SfxIdInoor:i32,

	/// NAME: Weather SfxId (outdoor) - 天候SfxId(屋外)
	/// DESC: Weather SfxId -1: No weather Sfx, outdoor only - 天候用SfxId -1：天候Sfxなし　屋外のみ
	pub SfxIdOutdoor:i32,

	/// NAME: AI field of view magnification - AI視界倍率
	/// DESC: AI field of view magnification - AI視界倍率
	pub aiSightRate:f32,

	/// NAME: Overwrite weight value in the distance camera - 遠見台カメラ中ウェイト値上書き
	/// DESC: Overwrite weight value in distant view camera (SEQ16724) - 遠見台カメラ中ウェイト値上書き(SEQ16724)
	pub DistViewWeatherGparamOverrideWeight:f32,
}

