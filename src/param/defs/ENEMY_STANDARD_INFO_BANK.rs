/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ENEMY_STANDARD_INFO_BANK {

	/// NAME: Behavior id - 挙動ｉｄ
	/// DESC: Enemy behavior ID - 敵の挙動ＩＤ
	pub EnemyBehaviorID:i32,

	/// NAME: Hit points - ヒットポイント
	/// DESC: Hit points - ヒットポイント
	pub HP:u16,

	/// NAME: Offensive power - 攻撃力
	/// DESC: Attack power (for proto only) - 攻撃力（プロト専用）
	pub AttackPower:u16,

	/// NAME: Character type - キャラタイプ
	/// DESC: Character type - キャラタイプ
	pub ChrType:i32,

	/// NAME: Height per [m] - あたりの高さ[m]
	/// DESC: Height per (Please specify a size larger than the diameter) - あたりの高さ（直径以上のサイズを指定してください）
	pub HitHeight:f32,

	/// NAME: Radius around [m] - あたりの半径[m]
	/// DESC: Radius around - あたりの半径
	pub HitRadius:f32,

	/// NAME: Weight [kg] - 重さ[kg]
	/// DESC: The weight of the character - キャラの重さ
	pub Weight:f32,

	/// NAME: Dynamic friction force - 動摩擦力
	/// DESC: Dynamic friction force - 動摩擦力
	pub DynamicFriction:f32,

	/// NAME: Static friction - 静摩擦力
	/// DESC: Static friction force - 静止摩擦力
	pub StaticFriction:f32,

	/// NAME: Upper body initial state - 上半身初期状態
	/// DESC: Upper body initial state (PG input) - 上半身初期状態（PG入力）
	pub UpperDefState:i32,

	/// NAME: Initial state of action - アクション初期状態
	/// DESC: Action initial state (PG input) - アクション初期状態（PG入力）
	pub ActionDefState:i32,

	/// NAME: Angle that can be turned per unit time [deg / s] - 単位時間当たり旋回できる角度[deg/s]
	/// DESC: Y-axis turning angle per unit time [deg / s] - 単位時間当たりのＹ軸旋回角度[deg/s]
	pub RotY_per_Second:f32,

	/// NAME: reserve - 予約
	pub reserve0:[u8;20],

	/// NAME: unused - 未使用
	/// DESC: unused - 未使用
	pub RotY_per_Second_old:u8,

	/// NAME: Can you move left and right? - 左右移動できるか
	/// DESC: Can you move left and right? - 左右移動できるか
	pub EnableSideStep:u8,

	/// NAME: Whether to use a ragdoll per character - キャラあたりにラグドールを使用するか
	/// DESC: Whether to use a ragdoll per character - キャラあたりにラグドールを使用するか
	pub UseRagdollHit:u8,

	/// NAME: reserve - 予約
	pub reserve_last:[u8;5],

	/// NAME: Amount of stamina - スタミナ量
	/// DESC: Total amount of stamina - スタミナ総量
	pub stamina:u16,

	/// NAME: Stamina recovery - スタミナ回復
	/// DESC: Stamina recovery amount per second - 1秒間あたりのスタミナ回復量
	pub staminaRecover:u16,

	/// NAME: Stamina basic consumption - スタミナ基本消費
	/// DESC: Basic value of stamina consumption used when attacking and guarding - 攻撃、ガード時に使用するスタミナ消費の基本値
	pub staminaConsumption:u16,

	/// NAME: Physical defense - 物理防御力
	/// DESC: Damage reduction base value for physical attacks - 物理攻撃に対するダメージ減少基本値
	pub deffenct_Phys:u16,

	/// NAME: Reservation 1 - 予約1
	pub reserve_last2:[u8;48],
}

impl Paramdef for ENEMY_STANDARD_INFO_BANK {
const NAME: &'static str = "ENEMY_STANDARD_INFO_BANK";
const VERSION: u16 = 1;
}
