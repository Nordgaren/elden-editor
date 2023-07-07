/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GRAPHICS_COMMON_PARAM_ST {

	/// NAME: Decal generation position offset when a bullet hits HIT INS - HIT INSに弾丸が当たった時のデカール発生位置オフセット
	/// DESC: The position where the decal that occurs when hitting HIT INS is offset by this value in the normal direction. - HIT INSに当たった時に発生するデカールの発生位置を法線方向にこの値だけオフセットする
	pub hitBulletDecalOffset_HitIns:f32,

	/// NAME: reserve - 予約
	/// DESC: (dummy8) - (dummy8)
	pub reserved02:[u8;8],

	/// NAME: Decal fade range when the character gets wet [m] - キャラが濡れた時のデカールフェード範囲[m]
	/// DESC: Fade range that erases decals when the character gets wet [m] - キャラが濡れた時にデカールを消すフェード範囲[m]
	pub charaWetDecalFadeRange:f32,

	/// NAME: reserve - 予約
	/// DESC: (dummy8) - (dummy8)
	pub reserved04:[u8;240],
}

