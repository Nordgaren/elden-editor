/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct RESIST_CORRECT_PARAM_ST {

	/// NAME: Addition after the first activation pt - 1回目発動後加算pt
	/// DESC: A value that is added to the resistance value after the abnormal condition is activated once. - 状態異常が1回発動した後に耐性値に加算される値
	pub addPoint1:f32,

	/// NAME: Addition after the second activation pt - 2回目発動後加算pt
	/// DESC: A value that is added to the resistance value after the abnormal condition is activated twice. - 状態異常が2回発動した後に耐性値に加算される値
	pub addPoint2:f32,

	/// NAME: Addition after the third activation pt - 3回目発動後加算pt
	/// DESC: A value that is added to the resistance value after the abnormal condition is activated 3 times. - 状態異常が3回発動した後に耐性値に加算される値
	pub addPoint3:f32,

	/// NAME: Addition after the 4th activation pt - 4回目発動後加算pt
	/// DESC: A value that is added to the resistance value after the abnormal condition is activated 4 times. - 状態異常が4回発動した後に耐性値に加算される値
	pub addPoint4:f32,

	/// NAME: Addition after the 5th activation pt - 5回目発動後加算pt
	/// DESC: A value that is added to the resistance value after the abnormal condition is activated 5 times. - 状態異常が5回発動した後に耐性値に加算される値
	pub addPoint5:f32,

	/// NAME: Magnification after the first activation - 1回目発動後倍率
	/// DESC: Magnification applied to the resistance value after the abnormal condition is activated once - 状態異常が1回発動した後に耐性値に掛かる倍率
	pub addRate1:f32,

	/// NAME: Magnification after the second activation - 2回目発動後倍率
	/// DESC: Magnification applied to the resistance value after the abnormal condition is activated twice - 状態異常が2回発動した後に耐性値に掛かる倍率
	pub addRate2:f32,

	/// NAME: Magnification after the third activation - 3回目発動後倍率
	/// DESC: Magnification applied to the resistance value after the abnormal condition is activated 3 times - 状態異常が3回発動した後に耐性値に掛かる倍率
	pub addRate3:f32,

	/// NAME: Magnification after the 4th activation - 4回目発動後倍率
	/// DESC: Magnification applied to the resistance value after the abnormal condition is activated 4 times - 状態異常が4回発動した後に耐性値に掛かる倍率
	pub addRate4:f32,

	/// NAME: Magnification after the 5th activation - 5回目発動後倍率
	/// DESC: Magnification applied to the resistance value after the abnormal condition is activated 5 times - 状態異常が5回発動した後に耐性値に掛かる倍率
	pub addRate5:f32,
}

