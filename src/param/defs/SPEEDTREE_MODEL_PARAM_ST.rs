/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SPEEDTREE_MODEL_PARAM_ST {

	/// NAME: Leaf minimum fade value - Leafの最小フェード値
	pub MinFadeLeaf:f32,

	/// NAME: Frond minimum fade value - Frondの最小フェード値
	pub MinFadeFrond:f32,

	/// NAME: Branch minimum fade value - Branchの最小フェード値
	pub MinFadeBranch:f32,

	/// NAME: Minimum transmitted light of Leaf - Leafの透過光最小値
	pub MinTranslucencyLeaf:f32,

	/// NAME: Maximum transmitted light of Leaf - Leafの透過光最大値
	pub MaxTranslucencyLeaf:f32,

	/// NAME: Minimum transmitted light of Frond - Frondの透過光最小値
	pub MinTranslucencyFrond:f32,

	/// NAME: Maximum transmitted light of Frond - Frondの透過光最大値
	pub MaxTranslucencyFrond:f32,

	/// NAME: Minimum transmitted light of Branch - Branchの透過光最小値
	pub MinTranslucencyBranch:f32,

	/// NAME: Maximum transmitted light of Branch - Branchの透過光最大値
	pub MaxTranslucencyBranch:f32,

	/// NAME: Billboard Specular suppression value - ビルボードのSpecular抑制値
	pub BillboardBackSpecularWeakenParam:f32,
}

