/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_SSAO_QUALITY_DETAIL {

	/// NAME: SSAO enabled - SSAO有効
	/// DESC: SSAO enabled - SSAO有効
	pub enabled:u8,

	/// NAME: Reprojection enabled - リプロジェクション有効
	/// DESC: When reprojection is forcibly enabled, Prevent Ghost is also enabled. - リプロジェクション強制有効の時は、PreventGhostも有効になる
	pub cs_reprojEnabledType:u8,

	/// NAME: Bilateral upscale effective - バイラテラルアップスケール有効
	/// DESC: Bilateral upscale effective - バイラテラルアップスケール有効
	pub cs_upScaleEnabledType:u8,

	/// NAME: Valid to use normals - 法線使用有効
	/// DESC: Valid to use normals - 法線使用有効
	pub cs_useNormalEnabledType:u8,

	/// NAME: dmy - dmy
	pub dmy:[u8;1],
}

