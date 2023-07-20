/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {

	/// NAME: Asset sound sound source effective distance setting [m] - アセットサウンド音源有効距離設定[m]
	/// DESC: Asset sound sound source effective distance [m] (less than 0: invalid) - アセットサウンド音源有効距離[m](0より小さい:無効)
	pub SoundObjEnableDist:f32,
}

impl Paramdef for SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {
const NAME: &'static str = "SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST";
const VERSION: u16 = 1;
}
