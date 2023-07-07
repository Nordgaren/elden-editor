/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_EFFECT_QUALITY_DETAIL {

	/// NAME: Soft particles enabled - ソフトパーティクル有効
	/// DESC: Soft particles enabled - ソフトパーティクル有効
	pub softParticleEnabled:u8,

	/// NAME: Glow effective - グロー有効
	/// DESC: Glow effective - グロー有効
	pub glowEnabled:u8,

	/// NAME: Distortion effective - 歪み有効
	/// DESC: Distortion effective - 歪み有効
	pub distortionEnable:u8,

	/// NAME: Enable bilateral upscale - バイラテラルアップスケールを有効
	/// DESC: Bilateral upscale effective - バイラテラルアップスケール有効
	pub cs_upScaleEnabledType:u8,

	/// NAME: Number of Emits at one time - 一回のエミット数
	/// DESC: Number of Emits at one time - 一回のエミット数
	pub fNumOnceEmitsScale:f32,

	/// NAME: Emit interval - エミット間隔
	/// DESC: Emit interval - エミット間隔
	pub fEmitSpanScale:f32,

	/// NAME: 1st stage LOD distance scale - 1段階目のLOD距離スケール
	/// DESC: 1st stage LOD distance scale - 1段階目のLOD距離スケール
	pub fLodDistance1Scale:f32,

	/// NAME: Second stage LOD distance scale - 2段階目のLOD距離スケール
	/// DESC: Second stage LOD distance scale - 2段階目のLOD距離スケール
	pub fLodDistance2Scale:f32,

	/// NAME: 3rd stage LOD distance scale - 3段階目のLOD距離スケール
	/// DESC: 3rd stage LOD distance scale - 3段階目のLOD距離スケール
	pub fLodDistance3Scale:f32,

	/// NAME: 4th stage LOD distance scale - 4段階目のLOD距離スケール
	/// DESC: 4th stage LOD distance scale - 4段階目のLOD距離スケール
	pub fLodDistance4Scale:f32,

	/// NAME: Magnification to the distance registered in the reduction buffer - 縮小バッファへ登録される距離への倍率
	/// DESC: Magnification to the distance registered in the reduction buffer - 縮小バッファへ登録される距離への倍率
	pub fScaleRenderDistanceScale:f32,

	/// NAME: dummy - ダミー
	pub dmy:[u8;4],
}

