/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WHITE_SIGN_COOL_TIME_PARAM_ST {

	/// NAME: Time limit (normal, no finger) - 制限時間（通常・指なし）
	/// DESC: Time limit [sec] (normal, dry fingerless) - 制限時間[sec]（通常・干からびた指無）
	pub limitationTime_Normal:f32,

	/// NAME: Time limit (normal, with fingers) - 制限時間（通常・指あり）
	/// DESC: Time limit [sec] (normal / dry finger) - 制限時間[sec]（通常・干からびた指有）
	pub limitationTime_NormalDriedFinger:f32,

	/// NAME: Time limit (map protection, no fingers) - 制限時間（マップ守護・指なし）
	/// DESC: Time limit [sec] (Map guardian, dry fingerless) - 制限時間[sec]（マップ守護・干からびた指無）
	pub limitationTime_Guardian:f32,

	/// NAME: Time limit (map protection, with fingers) - 制限時間（マップ守護・指あり）
	/// DESC: Time limit [sec] (Map guardian / dry finger) - 制限時間[sec]（マップ守護・干からびた指有）
	pub limitationTime_GuardianDriedFinger:f32,
}

impl Paramdef for WHITE_SIGN_COOL_TIME_PARAM_ST {
const NAME: &'static str = "WHITE_SIGN_COOL_TIME_PARAM_ST";
const VERSION: u16 = 1;
}
