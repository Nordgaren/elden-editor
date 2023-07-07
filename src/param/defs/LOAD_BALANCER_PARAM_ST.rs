/* This file was automatically generated from XML paramdefs. */
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct LOAD_BALANCER_PARAM_ST {

	/// NAME: lowerFpsThreshold - lowerFpsThreshold
	/// DESC: If it falls below this FPS, increase the load balance level by 1. - このFPSを下回ったら、ロードバランスレベルを1上げる
	pub lowerFpsThreshold:f32,

	/// NAME: upperFpsThreshold - upperFpsThreshold
	/// DESC: If you exceed this FPS, lower the load balance level by 1. - このFPSを上回ったら、ロードバランスレベルを1下げる
	pub upperFpsThreshold:f32,

	/// NAME: lowerFpsContinousCount - lowerFpsContinousCount
	/// DESC: If this frame continues below the threshold, level up - このフレーム連続してしきい値を下回ったら、レベルアップ
	pub lowerFpsContinousCount:u32,

	/// NAME: upperFpsContinousCount - upperFpsContinousCount
	/// DESC: If the threshold is exceeded continuously for this frame, level down - このフレーム連続してしきい値を上回ったら、レベルダウン
	pub upperFpsContinousCount:u32,

	/// NAME: downAfterChangeSleep - downAfterChangeSleep
	/// DESC: Sleep frame count after level down - レベルダウン後のスリープフレームカウント
	pub downAfterChangeSleep:u32,

	/// NAME: upAfterChangeSleep - upAfterChangeSleep
	/// DESC: Sleep frame count after leveling up - レベルアップ後のスリープフレームカウント
	pub upAfterChangeSleep:u32,

	/// NAME: Light shaft - ライトシャフト
	/// DESC: Light shaft of filter - フィルタのライトシャフト
	pub postProcessLightShaft:u8,

	/// NAME: Bloom - Bloom
	/// DESC: Bloom - ブルーム
	pub postProcessBloom:u8,

	/// NAME: Glow - Glow
	/// DESC: Glow - グロー
	pub postProcessGlow:u8,

	/// NAME: AA - AA
	/// DESC: Antialiasing - アンチエイリアス
	pub postProcessAA:u8,

	/// NAME: SSAO - SSAO
	/// DESC: SSAO - SSAO
	pub postProcessSSAO:u8,

	/// NAME: DOF - DOF
	/// DESC: DOF - DOF
	pub postProcessDOF:u8,

	/// NAME: Motion Blur - MotionBlur
	/// DESC: Motion Blur - MotionBlur
	pub postProcessMotionBlur:u8,

	/// NAME: MotionBlurIteration - MotionBlurIteration
	/// DESC: Reduce the number of Motion Blur iterations - MotionBlurのイテレーション回数を下げる
	pub postProcessMotionBlurIteration:u8,

	/// NAME: Reserve - 予備
	/// DESC: Reserve - 予備
	pub reserve0:[u8;1],

	/// NAME: Shadow Blur - Shadow Blur
	/// DESC: Cut the shadow blur - 影のブラーを切る
	pub shadowBlur:u8,

	/// NAME: Around the SFX Emit - SFXのエミット回り
	/// DESC: Emit interval, number of emits, LOD distance halved in graphics config - エミット間隔、エミット数、LOD距離をグラフィックスコンフィグの半分に
	pub sfxParticleHalf:u8,

	/// NAME: SFX reflection - SFXの反射
	/// DESC: Omit the reflection scene SFX - 反射シーンSFXをオミット
	pub sfxReflection:u8,

	/// NAME: Water interaction - 水面インタラクション
	/// DESC: Omit water surface interact SFX - 水面インタラクトSFXをオミット
	pub sfxWaterInteraction:u8,

	/// NAME: SFX glow - SFXのグロー
	/// DESC: Omit Glow playing with SFX - SFXでかけてるGlowをオミット
	pub sfxGlow:u8,

	/// NAME: SFX distortion - SFXの歪み
	/// DESC: Omit of distortion applied by SFX - SFXでかけてる歪みのオミット
	pub sfxDistortion:u8,

	/// NAME: Soft sprite - ソフトスプライト
	/// DESC: Soft sprite omit with SFX - SFXでかけてるソフトスプライトのオミット
	pub sftSoftSprite:u8,

	/// NAME: Light shaft - ライトシャフト
	/// DESC: SFX Light Shaft Omit - SFXのライトシャフトのオミット
	pub sfxLightShaft:u8,

	/// NAME: Scale to the distance judgment of the effect dynamically registered in the reduction buffer - 動的に縮小バッファに登録されるエフェクトの距離判定にスケール
	/// DESC: Scale to the distance judgment of the effect dynamically registered in the reduction buffer by the distance of SFX - SFXの距離で動的に縮小バッファに登録されるエフェクトの距離判定にスケール
	pub sfxScaleRenderDistanceScale:u8,

	/// NAME: Dynamic resolution - 動的解像度
	/// DESC: Dynamic resolution - 動的解像度
	pub dynamicResolution:u8,

	/// NAME: Shadow Cascade0 Resolution Half - Shadow Cascade0 ResolutionHalf
	/// DESC: Cut the shadow (cascade 0) resolution in half - 影（カスケード0）の解像度を半分に下げる
	pub shadowCascade0ResolutionHalf:u8,

	/// NAME: Shadow Cascade1 Resolution Half - Shadow Cascade1 ResolutionHalf
	/// DESC: Cut the shadow (cascade 1) resolution in half - 影（カスケード1）の解像度を半分に下げる
	pub shadowCascade1ResolutionHalf:u8,

	/// NAME: Local player - ローカルプレイヤー
	/// DESC: Turn off the water wetting process for local players - ローカルプレイヤーの水濡れ処理を切る
	pub chrWetDisablePlayer:u8,

	/// NAME: Remote player - リモートプレイヤー
	/// DESC: Turn off the water wetting process of the remote player - リモートプレイヤーの水濡れ処理を切る
	pub chrWetDisableRemotePlayer:u8,

	/// NAME: Enemy character - 敵キャラ
	/// DESC: Turn off the water wetting process of enemy characters - 敵キャラの水濡れ処理を切る
	pub chrWetDisableEnemy:u8,

	/// NAME: Resolution reduction lower limit (%) - 解像度引き下げ 下限(%)
	/// DESC: Resolution reduction lower limit (%) - 解像度引き下げ 下限(%)
	pub dynamicResolutionPercentageMin:u8,

	/// NAME: Resolution reduction upper limit (%) - 解像度引き下げ 上限(%)
	/// DESC: Resolution reduction upper limit (%) - 解像度引き下げ 上限(%)
	pub dynamicResolutionPercentageMax:u8,

	/// NAME: Reserve - 予備
	/// DESC: Reserve - 予備
	pub reserve1:[u8;30],
}

