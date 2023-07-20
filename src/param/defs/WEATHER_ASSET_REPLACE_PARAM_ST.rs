/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WEATHER_ASSET_REPLACE_PARAM_ST {

	/// NAME: Map number - マップ番号
	/// DESC: Specify the map number in 8 digits. Only open and legacy celestial sphere placement maps can be specified - マップ番号を8桁で指定します。オープン、レガシーの天球配置マップのみ指定可能です
	pub mapId:u32,

	/// NAME: weather - 天候
	/// DESC: Specifies the assets that will be valid in this weather. - この天候で有効になるアセットを指定します。
	pub TransitionSrcWeather:i16,

	/// NAME: reserved0 - reserved0
	pub padding0:[u8;2],

	/// NAME: After "fire ashes"? - 「火の灰」後か？
	/// DESC: Specifies whether or not to adapt to the "fire ash" state of the map. - マップの「火の灰」状態での適応の有無を指定します。
	pub isFireAsh:u8,

	/// NAME: padding0 - padding0
	pub padding1:[u8;3],

	/// NAME: reserved2 - reserved2
	pub reserved2:u32,

	/// NAME: Display Asset 1 - 表示アセット1
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId0:i32,

	/// NAME: Display asset 2 - 表示アセット2
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId1:i32,

	/// NAME: Display assets 3 - 表示アセット3
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId2:i32,

	/// NAME: Display assets 4 - 表示アセット4
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId3:i32,

	/// NAME: Display assets 5 - 表示アセット5
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId4:i32,

	/// NAME: Display assets 6 - 表示アセット6
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId5:i32,

	/// NAME: Display assets 7 - 表示アセット7
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId6:i32,

	/// NAME: Display assets 8 - 表示アセット8
	/// DESC: -1: Invalid Specify the asset ID to generate. AEG999_999-> 999999 - -1:無効 生成するアセットIDを指定します。AEG999_999 -> 999999
	pub AssetId7:i32,

	/// NAME: reserved0 - reserved0
	pub reserved0:[u8;8],

	/// NAME: Generation limit ID 0 - 生成制限ID0
	/// DESC: ID for production restriction. -1: Unlimited. Generation is allowed only if it matches the generation limit ID in "Map default parameter .xlsm" (SEQ08921). - 生成制限用のIDです。-1:無制限。「マップデフォルトパラメータ.xlsm」の生成制限IDと一致した場合のみ生成が許可されます(SEQ08921)
	pub CreateAssetLimitId0:i8,

	/// NAME: Generation limit ID1 - 生成制限ID1
	/// DESC: ID for production restriction. -1: Unlimited. Generation is allowed only if it matches the generation limit ID in "Map default parameter .xlsm" (SEQ08921). - 生成制限用のIDです。-1:無制限。「マップデフォルトパラメータ.xlsm」の生成制限IDと一致した場合のみ生成が許可されます(SEQ08921)
	pub CreateAssetLimitId1:i8,

	/// NAME: Generation limit ID2 - 生成制限ID2
	/// DESC: ID for production restriction. -1: Unlimited. Generation is allowed only if it matches the generation limit ID in "Map default parameter .xlsm" (SEQ08921). - 生成制限用のIDです。-1:無制限。「マップデフォルトパラメータ.xlsm」の生成制限IDと一致した場合のみ生成が許可されます(SEQ08921)
	pub CreateAssetLimitId2:i8,

	/// NAME: Generation limit ID3 - 生成制限ID3
	/// DESC: ID for production restriction. -1: Unlimited. Generation is allowed only if it matches the generation limit ID in "Map default parameter .xlsm" (SEQ08921). - 生成制限用のIDです。-1:無制限。「マップデフォルトパラメータ.xlsm」の生成制限IDと一致した場合のみ生成が許可されます(SEQ08921)
	pub CreateAssetLimitId3:i8,

	/// NAME: reserved1 - reserved1
	pub reserved1:[u8;4],
}

impl Paramdef for WEATHER_ASSET_REPLACE_PARAM_ST {
const NAME: &'static str = "WEATHER_ASSET_REPLACE_PARAM_ST";
const VERSION: u16 = 1;
}
