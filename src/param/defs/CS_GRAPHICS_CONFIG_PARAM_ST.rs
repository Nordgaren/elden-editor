/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_GRAPHICS_CONFIG_PARAM_ST {

	/// NAME: Texture filter quality - テクスチャフィルタ品質
	/// DESC: Texture filter quality (default Midele) - テクスチャフィルタ品質(デフォルトMidele)
	pub m_textureFilterQuality:u8,

	/// NAME: AA quality - AA品質
	/// DESC: AA quality (default High) - AA品質(デフォルトHigh)
	pub m_aaQuality:u8,

	/// NAME: SSAO quality - SSAO品質
	/// DESC: SSAO quality (default High) - SSAO品質(デフォルトHigh)
	pub m_ssaoQuality:u8,

	/// NAME: Depth of field quality - 被写界深度品質
	/// DESC: Depth of field quality (default High) - 被写界深度品質(デフォルトHigh)
	pub m_dofQuality:u8,

	/// NAME: Motion blur quality - モーションブラー品質
	/// DESC: Motion blur quality (default High) - モーションブラー品質(デフォルトHigh)
	pub m_motionBlurQuality:u8,

	/// NAME: Shadow quality - シャドウ品質
	/// DESC: Shadow quality (default High) - シャドウ品質(デフォルトHigh)
	pub m_shadowQuality:u8,

	/// NAME: Lighting quality - ライティング品質
	/// DESC: Lighting quality (default High) - ライティング品質(デフォルトHigh)
	pub m_lightingQuality:u8,

	/// NAME: Effect quality - エフェクト品質
	/// DESC: Effect quality (default High) - エフェクト品質(デフォルトHigh)
	pub m_effectQuality:u8,

	/// NAME: Decal quality - デカール品質
	/// DESC: Decal quality (default High) - デカール品質(デフォルトHigh)
	pub m_decalQuality:u8,

	/// NAME: Reflection quality - 反射品質
	/// DESC: Reflection quality (default High) - 反射品質(デフォルトHigh)
	pub m_reflectionQuality:u8,

	/// NAME: Water quality - ウォーター品質
	/// DESC: Water quality (default High) - ウォーター品質(デフォルトHigh)
	pub m_waterQuality:u8,

	/// NAME: Shader quality - シェーダー品質
	/// DESC: Shader quality (default High) - シェーダー品質(デフォルトHigh)
	pub m_shaderQuality:u8,

	/// NAME: Volumetric effect quality - ボリューメトリック効果品質
	/// DESC: Volumetric effect quality (default High) - ボリューメトリック効果品質(デフォルトHigh)
	pub m_volumetricEffectQuality:u8,

	/// NAME: dmy - dmy
	pub m_dummy:[u8;3],
}

