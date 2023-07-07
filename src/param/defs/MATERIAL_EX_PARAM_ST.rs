/* This file was automatically generated from XML paramdefs. */
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MATERIAL_EX_PARAM_ST {

	/// NAME: Material parameter name - マテリアルパラメータ名
	/// DESC: Set the parameter name of the material. Up to 31 characters - マテリアルのパラメータ名を設定する。最大31文字まで
	pub paramName:[u16;32],

	/// NAME: Target material ID - 対象マテリアルID
	/// DESC: NPC Para: Resident Material Expansion Para ID setting -1 for all materials - NPCパラ：常駐マテリアル拡張パラID用設定　-1なら全マテリアル対象
	pub materialId:i32,

	/// NAME: Overwrite value 1 (R) - 上書き値1(R)
	/// DESC: NPC Para: Resident Material Extended Para ID Settings - NPCパラ：常駐マテリアル拡張パラID用設定
	pub materialParamValue0:f32,

	/// NAME: Overwrite value 2 (G) - 上書き値2(G)
	/// DESC: NPC Para: Resident Material Expansion Para ID Settings - NPCパラ：常駐マテリアル拡張パラID用設定
	pub materialParamValue1:f32,

	/// NAME: Overwrite value 3 (B) - 上書き値3(B)
	/// DESC: NPC Para: Resident Material Expansion Para ID Settings - NPCパラ：常駐マテリアル拡張パラID用設定
	pub materialParamValue2:f32,

	/// NAME: Overwrite value 4 (A) - 上書き値4(A)
	/// DESC: NPC Para: Resident Material Extended Para ID Settings - NPCパラ：常駐マテリアル拡張パラID用設定
	pub materialParamValue3:f32,

	/// NAME: Overwrite value 5 (I) - 上書き値5(I)
	/// DESC: NPC Para: Resident Material Expansion Para ID Settings - NPCパラ：常駐マテリアル拡張パラID用設定
	pub materialParamValue4:f32,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad:[u8;8],
}

