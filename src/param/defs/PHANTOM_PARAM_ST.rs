/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct PHANTOM_PARAM_ST {

	/// NAME: A - A
	/// DESC: Edge color A. - エッジ色Aです。
	pub edgeColorA:f32,

	/// NAME: A - A
	/// DESC: The front color is A. - 正面色Aです。
	pub frontColorA:f32,

	/// NAME: A - A
	/// DESC: Diffuse multiplication color A. - ディフューズ乗算色Aです。
	pub diffMulColorA:f32,

	/// NAME: A - A
	/// DESC: Specular multiplication color A. - スペキュラ乗算色Aです。
	pub specMulColorA:f32,

	/// NAME: A - A
	/// DESC: Light color A. - ライト色Aです。
	pub lightColorA:f32,

	/// NAME: R - R
	/// DESC: Edge color R. - エッジ色Rです。
	pub edgeColorR:u8,

	/// NAME: G - G
	/// DESC: Edge color G. - エッジ色Gです。
	pub edgeColorG:u8,

	/// NAME: B - B
	/// DESC: Edge color B. - エッジ色Bです。
	pub edgeColorB:u8,

	/// NAME: R - R
	/// DESC: The front color is R. - 正面色Rです。
	pub frontColorR:u8,

	/// NAME: G - G
	/// DESC: The front color is G. - 正面色Gです。
	pub frontColorG:u8,

	/// NAME: B - B
	/// DESC: Front color B. - 正面色Bです。
	pub frontColorB:u8,

	/// NAME: R - R
	/// DESC: Diffuse multiplication color R. - ディフューズ乗算色Rです。
	pub diffMulColorR:u8,

	/// NAME: G - G
	/// DESC: Diffuse multiplication color G. - ディフューズ乗算色Gです。
	pub diffMulColorG:u8,

	/// NAME: B - B
	/// DESC: Diffuse multiplication color B. - ディフューズ乗算色Bです。
	pub diffMulColorB:u8,

	/// NAME: R - R
	/// DESC: Specular multiplication color R. - スペキュラ乗算色Rです。
	pub specMulColorR:u8,

	/// NAME: G - G
	/// DESC: Specular multiplication color G. - スペキュラ乗算色Gです。
	pub specMulColorG:u8,

	/// NAME: B - B
	/// DESC: Specular multiplication color B. - スペキュラ乗算色Bです。
	pub specMulColorB:u8,

	/// NAME: R - R
	/// DESC: Light color R. - ライト色Rです。
	pub lightColorR:u8,

	/// NAME: G - G
	/// DESC: Light color G. - ライト色Gです。
	pub lightColorG:u8,

	/// NAME: B - B
	/// DESC: Light color B. - ライト色Bです。
	pub lightColorB:u8,

	/// NAME: Reserve - 予備
	pub reserve:[u8;1],

	/// NAME: α - α
	/// DESC: The overall transparency. - 全体の透過度です。
	pub alpha:f32,

	/// NAME: Blend rate - ブレンド率
	/// DESC: The blend ratio. - ブレンド率です。
	pub blendRate:f32,

	/// NAME: α type - α種類
	/// DESC: The type of alpha blend. - αブレンドの種類です。
	pub blendType:u8,

	/// NAME: Whether to perform edge color subtraction - エッジ色減算を行うか
	/// DESC: Whether to perform edge color subtraction. - エッジ色減算を行うかです。
	pub isEdgeSubtract:u8,

	/// NAME: Whether to perform front color subtraction - 正面色減算を行うか
	/// DESC: Is it front color subtraction? - 正面色減算を行うかです。
	pub isFrontSubtract:u8,

	/// NAME: Do not do 2pass - 2passを行わない
	/// DESC: Do you not do 2pass? - 2passを行わないかです。
	pub isNo2Pass:u8,

	/// NAME: Edge width - エッジの幅
	/// DESC: Edge width - エッジの幅
	pub edgePower:f32,

	/// NAME: Glow strength - Glowの強さ
	/// DESC: Glow strength - Glowの強さ
	pub glowScale:f32,
}

