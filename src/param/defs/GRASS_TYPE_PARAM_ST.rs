/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GRASS_TYPE_PARAM_ST {

	/// NAME: LOD distance - LOD距離
	pub lodRange:u16,

	/// NAME: LOD0 cluster type - LOD0のクラスタータイプ
	pub lod0ClusterType:u8,

	/// NAME: LOD1 cluster type - LOD1のクラスタータイプ
	pub lod1ClusterType:u8,

	/// NAME: LOD2 cluster type - LOD2のクラスタータイプ
	pub lod2ClusterType:u8,

	/// NAME: pad0 - pad0
	pub pad0:[u8;2],

	/// NAME: Placement method - 配置方法
	pub distributionType:u8,

	/// NAME: Basic density - 基本密度
	pub baseDensity:f32,

	/// NAME: Model name (0) - モデル名（０）
	pub model0Name:[u16;16],

	/// NAME: Flat texture name - フラットテクスチャー名
	pub flatTextureName:[u16;32],

	/// NAME: Billboard texture name - ビルボードのテクスチャー名
	pub billboardTextureName:[u16;32],

	/// NAME: Effect of tilt (%) - 傾きの影響（％）
	pub normalInfluence:u8,

	/// NAME: Maximum tilt angle (degrees) - 傾きの最大角度（度）
	pub inclinationMax:u8,

	/// NAME: Randomness of tilt angle (degrees) - 傾斜角のランダム性（度）
	pub inclinationJitter:u8,

	/// NAME: Width scale range (min,%) - 幅のスケール範囲(min，％)
	pub scaleBaseMin:u8,

	/// NAME: Width scale range (max,%) - 幅のスケール範囲(max，％)
	pub scaleBaseMax:u8,

	/// NAME: Height scale range (min,%) - 高さのスケール範囲(min，％)
	pub scaleHeightMin:u8,

	/// NAME: Height scale range (max,%) - 高さのスケール範囲(max，％)
	pub scaleHeightMax:u8,

	/// NAME: Multiplication color 1 (red) - 乗算カラー１ (赤）
	pub colorShade1_r:u8,

	/// NAME: Multiplication color 1 (green) - 乗算カラー１ （緑）
	pub colorShade1_g:u8,

	/// NAME: Multiplication color 1 (blue) - 乗算カラー １（青）
	pub colorShade1_b:u8,

	/// NAME: Multiplication color 2 (red) - 乗算カラー２ （赤）
	pub colorShade2_r:u8,

	/// NAME: Multiplication color 2 (green) - 乗算カラー２ （緑）
	pub colorShade2_g:u8,

	/// NAME: Multiplication color 2 (blue) - 乗算カラー ２（青）
	pub colorShade2_b:u8,

	/// NAME: Separation of planes - 平面の分離
	pub flatSplitType:u8,

	/// NAME: Number of planes - 平面の枚数
	pub flatBladeCount:u8,

	/// NAME: Plane angle (degrees) - 平面の角度（度）
	pub flatSlant:i8,

	/// NAME: Plane distance - 平面の距離
	pub flatRadius:f32,

	/// NAME: Do you want to cast a shadow - 影を落とすか
	pub castShadow:u8,

	/// NAME: Amplitude (magnitude of shaking) - 振幅(揺れの大きさ)
	pub windAmplitude:u8,

	/// NAME: pad1 - pad1
	pub pad1:[u8;1],

	/// NAME: Cycle (speed) - 周期(速度)
	pub windCycle:u8,

	/// NAME: Direction (degree) - 方向（度）
	/// DESC: Random for -1 - -1の場合はランダム
	pub orientationAngle:f32,

	/// NAME: Directional range (degrees) - 方向の範囲（度）
	pub orientationRange:f32,

	/// NAME: Model spacing - モデル間隔
	pub spacing:f32,

	/// NAME: Dithering - ディザリング
	pub dithering:u8,

	/// NAME: pad2 - pad2
	pub pad:[u8;3],

	/// NAME: Simple model name - Simpleモデル名
	pub simpleModelName:[u16;16],

	/// NAME: Model name (1) - モデル名（１）
	pub model1Name:[u16;16],
}

impl Paramdef for GRASS_TYPE_PARAM_ST {
const NAME: &'static str = "GRASS_TYPE_PARAM_ST";
const VERSION: u16 = 1;
}
