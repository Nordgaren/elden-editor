/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL {

	/// NAME: Fog enabled - フォグ有効
	/// DESC: Fog enabled - フォグ有効
	pub fogEnabled:u8,

	/// NAME: Fog shadow permission - フォグシャドウ許可
	/// DESC: Fog shadow permission - フォグシャドウ許可
	pub fogShadowEnabled:u8,

	/// NAME: dmy - dmy
	/// DESC: dmy - dmy
	pub dmy:[u8;2],

	/// NAME: Shadow sample count offset. - シャドウのサンプルカウントオフセット。
	/// DESC: Shadow sample count offset. - シャドウのサンプルカウントオフセット。
	pub fogShadowSampleCountBias:i32,

	/// NAME: Local light calculation distance scale (0 does not calculate local light) - ローカルライト計算距離スケール (0にするとローカルライト計算をしない)
	/// DESC: Local light calculation distance scale (0 does not calculate local light) - ローカルライト計算距離スケール (0にするとローカルライト計算をしない)
	pub fogLocalLightDistScale:f32,

	/// NAME: Fog volume size scaler - フォグボリュームサイズスケーラ
	/// DESC: Fog volume size scaler - フォグボリュームサイズスケーラ
	pub fogVolueSizeScaler:u32,

	/// NAME: Fog volume size division - フォグボリュームサイズ除算
	/// DESC: Fog volume size division - フォグボリュームサイズ除算
	pub fogVolueSizeDivider:u32,

	/// NAME: Fog Volume Depth Slice Scaler - フォグボリューム奥行きスライススケーラ
	/// DESC: Fog Volume Depth Slice Scaler - フォグボリューム奥行きスライススケーラ
	pub fogVolumeDepthScaler:u32,

	/// NAME: Fog volume depth slice division - フォグボリューム奥行きスライス除算
	/// DESC: Fog volume depth slice division - フォグボリューム奥行きスライス除算
	pub fogVolumeDepthDivider:u32,

	/// NAME: Arranged fog volume enabled - 配置式フォグボリューム有効
	/// DESC: Arranged fog volume enabled - 配置式フォグボリューム有効
	pub fogVolumeEnabled:u8,

	/// NAME: Upscale type - アップスケール種別
	/// DESC: Method type at the time of upscale - アップスケール時の手法種別
	pub fogVolumeUpScaleType:u8,

	/// NAME: Edge correction level performed only when bilateral - バイラテラル時のみ行われるエッジ補正レベル
	/// DESC: Edge correction level performed only at bilateral (0: invalid, edge redraw permission at 1: 1 / 2x1 / 2 resolution, edge redraw permission with parameter reduction at 2: 1 / 2x1 / 2 + 1x1 resolution, 3 : 1 / 2x1 / 2 + 1x1 resolution edge redraw permission) - バイラテラル時のみ行われるエッジ補正レベル(0:無効,1:1/2x1/2解像度でのエッジ再描画許可,2:1/2x1/2+1x1解像度でパラメータ削減ありのエッジ再描画許可,3:1/2x1/2+1x1解像度でのエッジ再描画許可)
	pub fogVolumeEdgeCorrectionLevel:u8,

	/// NAME: Offset of sampling number during ray marching - レイマーチング時のサンプリング数のオフセット
	/// DESC: Offset of sampling number during ray marching - レイマーチング時のサンプリング数のオフセット
	pub fogVolumeRayMarcingSampleCountOffset:i8,

	/// NAME: Shadow permission - シャドウ許可
	/// DESC: Shadow permission (refers to shadow processing due to density changes in the area, where shadows are cast on the area) - シャドウ許可(領域に影が落ちる、領域内の密度変化による陰影処理を指す)
	pub fogVolumeShadowEnabled:u8,

	/// NAME: Forcibly casts a shadow on the area regardless of the setting when shadow is allowed - シャドウ許可時に設定にかかわらず領域に強制的に影を落とす
	/// DESC: Forcibly casts a shadow on the area regardless of the setting when shadow is permitted (shadow processing is not affected) - シャドウ許可時に設定にかかわらず領域に強制的に影を落とす(陰影処理は影響をうけない)
	pub fogVolumeForceShadowing:u8,

	/// NAME: Fog volume upscale processing resolution - フォグボリュームのアップスケール処理解像度
	pub fogVolumeResolution:u8,

	/// NAME: pad - pad
	pub pad2:[u8;1],
}

