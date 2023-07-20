/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {

	/// NAME: Mimicry weight 0 - 擬態重み0
	/// DESC: Mimicry weight 0 - 擬態重み0
	pub mimicryEstablishment0:f32,

	/// NAME: Mimicry weight 1 - 擬態重み1
	/// DESC: Mimicry weight 1 - 擬態重み1
	pub mimicryEstablishment1:f32,

	/// NAME: Mimicry weight 2 - 擬態重み2
	/// DESC: Mimicry weight 2 - 擬態重み2
	pub mimicryEstablishment2:f32,

	/// NAME: Mimicry 0 SFXID Forward swing - 擬態0 SFXID 前振り
	/// DESC: Mimicry 0 SFXID Forward swing - 擬態0 SFXID 前振り
	pub mimicryBeginSfxId0:i32,

	/// NAME: Mimicry 0 SFXID body - 擬態0 SFXID 本体
	/// DESC: Mimicry 0 SFXID body - 擬態0 SFXID 本体
	pub mimicrySfxId0:i32,

	/// NAME: Mimicry 0 SFXID release - 擬態0 SFXID 解除
	/// DESC: Mimicry 0 SFXID release - 擬態0 SFXID 解除
	pub mimicryEndSfxId0:i32,

	/// NAME: Mimicry 1 SFXID Forward swing - 擬態1 SFXID 前振り
	/// DESC: Mimicry 1 SFXID Forward swing - 擬態1 SFXID 前振り
	pub mimicryBeginSfxId1:i32,

	/// NAME: Mimicry 1 SFXID body - 擬態1 SFXID 本体
	/// DESC: Mimicry 1 SFXID body - 擬態1 SFXID 本体
	pub mimicrySfxId1:i32,

	/// NAME: Mimicry 1 SFXID release - 擬態1 SFXID 解除
	/// DESC: Mimicry 1 SFXID release - 擬態1 SFXID 解除
	pub mimicryEndSfxId1:i32,

	/// NAME: Mimicry 2 SFXID Forward swing - 擬態2 SFXID 前振り
	/// DESC: Mimicry 2 SFXID Forward swing - 擬態2 SFXID 前振り
	pub mimicryBeginSfxId2:i32,

	/// NAME: Mimicry 2 SFXID body - 擬態2 SFXID 本体
	/// DESC: Mimicry 2 SFXID body - 擬態2 SFXID 本体
	pub mimicrySfxId2:i32,

	/// NAME: Mimicry 2 SFXID cancellation - 擬態2 SFXID 解除
	/// DESC: Mimicry 2 SFXID cancellation - 擬態2 SFXID 解除
	pub mimicryEndSfxId2:i32,

	/// NAME: pad - パッド
	/// DESC: pad - pad
	pub pad1:[u8;16],
}

impl Paramdef for MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {
const NAME: &'static str = "MAP_MIMICRY_ESTABLISHMENT_PARAM_ST";
const VERSION: u16 = 1;
}
