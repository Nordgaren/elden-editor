/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct AI_ATTACK_PARAM_ST {

	/// NAME: Reference ID - 参照ID
	/// DESC: ID specified by NPC thinking parameters - NPC思考パラメータで指定するID
	pub attackTableId:i32,

	/// NAME: Attack ID - 攻撃ID
	/// DESC: Attack number - 攻撃の番号
	pub attackId:i32,

	/// NAME: Success judgment distance - 成功判定距離
	/// DESC: For arguments of Common_Attack type subgoals - Common_Attack系のサブゴールの引数用
	pub successDistance:f32,

	/// NAME: Turn time before attack - 攻撃前旋回時間
	/// DESC: For arguments of Common_Attack type subgoals - Common_Attack系のサブゴールの引数用
	pub turnTimeBeforeAttack:f32,

	/// NAME: Front judgment angle - 正面判定角度
	/// DESC: For arguments of Common_Attack type subgoals - Common_Attack系のサブゴールの引数用
	pub frontAngleRange:i16,

	/// NAME: Upward execution threshold - 上方実行閾値
	/// DESC: For arguments of Common_Attack type subgoals - Common_Attack系のサブゴールの引数用
	pub upAngleThreshold:i16,

	/// NAME: Downward execution threshold - 下方実行閾値
	/// DESC: For arguments of Common_Attack type subgoals - Common_Attack系のサブゴールの引数用
	pub downAngleThershold:i16,

	/// NAME: Is it a starting technique? - 始動技か
	/// DESC: Attacks from the second stage of the combo are × - コンボの2段目以降の攻撃は×
	pub isFirstAttack:u8,

	/// NAME: Whether to select outside the proper distance - 適正距離外で選択するか
	/// DESC: Whether to select when the distance is out of the proper distance - 適正距離外の時に選択対象にするかどうか
	pub doesSelectOnOutRange:u8,

	/// NAME: Minimum proper distance - 最小適正距離
	/// DESC: Minimum value of proper attack distance - 攻撃の適正距離の最小値
	pub minOptimalDistance:f32,

	/// NAME: Maximum proper distance - 最大適正距離
	/// DESC: Maximum attack suitability distance - 攻撃の適性距離の最大値
	pub maxOptimalDistance:f32,

	/// NAME: Appropriate angle reference direction 1 - 適正角度基準方向1
	/// DESC: Direction that serves as a reference for the proper angle of attack (XZ plane) - 攻撃の適正角度の基準となる方向（XZ平面）
	pub baseDirectionForOptimalAngle1:i16,

	/// NAME: Appropriate angle reference range 1 - 適正角度基準範囲1
	/// DESC: Range of aptitude angles for attacks - 攻撃の適性角度の範囲
	pub optimalAttackAngleRange1:i16,

	/// NAME: Appropriate angle reference direction 2 - 適正角度基準方向2
	/// DESC: Direction that serves as a reference for attack aptitude accuracy (XZ plane) - 攻撃の適性確度の基準となる方向（XZ平面）
	pub baseDirectionForOptimalAngle2:i16,

	/// NAME: Appropriate angle reference range 2 - 適正角度基準範囲2
	/// DESC: Range of aptitude angles for attacks - 攻撃の適性角度の範囲
	pub optimalAttackAngleRange2:i16,

	/// NAME: Executable interval - 実行可能インターバル
	/// DESC: Time required to attack once and then use it again - 一度攻撃を行ってから再度使うために必要な時間
	pub intervalForExec:f32,

	/// NAME: Selection rate - 選択レート
	/// DESC: Specify the ease of selection by magnification - 選択されやすさを倍率で指定する
	pub selectionTendency:f32,

	/// NAME: Short range selection rate - 近距離選択レート
	/// DESC: Selection rate at close range - 近距離での選択レート
	pub shortRangeTendency:f32,

	/// NAME: Medium range selection rate - 中距離選択レート
	/// DESC: Selection rate at medium range - 中距離での選択レート
	pub middleRangeTendency:f32,

	/// NAME: Distance selection rate - 遠距離選択レート
	/// DESC: Selection rate at long distances - 遠距離での選択レート
	pub farRangeTendency:f32,

	/// NAME: Out of range rate - 範囲外レート
	/// DESC: Selection rate out of range - 範囲外での選択レート
	pub outRangeTendency:f32,

	/// NAME: Derived attack 1 - 派生攻撃1
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId1:i32,

	/// NAME: Derived attack 2 - 派生攻撃2
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId2:i32,

	/// NAME: Derived attack 3 - 派生攻撃3
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId3:i32,

	/// NAME: Derived attack 4 - 派生攻撃4
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId4:i32,

	/// NAME: Derived attack 5 - 派生攻撃5
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId5:i32,

	/// NAME: Derived attack 6 - 派生攻撃6
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId6:i32,

	/// NAME: Derived attack 7 - 派生攻撃7
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId7:i32,

	/// NAME: Derived attack 8 - 派生攻撃8
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId8:i32,

	/// NAME: Derived attack 9 - 派生攻撃9
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId9:i32,

	/// NAME: Derived attack 10 - 派生攻撃10
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId10:i32,

	/// NAME: Derived attack 11 - 派生攻撃11
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId11:i32,

	/// NAME: Derived attack 12 - 派生攻撃12
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId12:i32,

	/// NAME: Derived attack 13 - 派生攻撃13
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId13:i32,

	/// NAME: Derived attack 14 - 派生攻撃14
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId14:i32,

	/// NAME: Derived attack 15 - 派生攻撃15
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId15:i32,

	/// NAME: Derived attack 16 - 派生攻撃16
	/// DESC: Derivable attack number - 派生可能な攻撃の番号
	pub deriveAttackId16:i32,

	/// NAME: Minimum life of the goal - ゴールの最小寿命
	/// DESC: Minimum life of the goal - ゴールの最小寿命
	pub goalLifeMin:f32,

	/// NAME: Maximum life span of the goal - ゴールの最大寿命
	/// DESC: Maximum life span of the goal - ゴールの最大寿命
	pub goalLifeMax:f32,

	/// NAME: Whether to select within the appropriate distance - 適正距離内で選択するか
	/// DESC: Whether to select when within the appropriate distance - 適正距離内の時に選択対象にするかどうか
	pub doesSelectOnInnerRange:u8,

	/// NAME: Whether to use it as the first hit - 初撃として使用するか
	/// DESC: Whether to use it as the first hit - 初撃として使用するか
	pub enableAttackOnBattleStart:u8,

	/// NAME: Whether to select when the target is down - ターゲットダウン時選択するか
	/// DESC: Whether to select when the target is down - ターゲットダウン時選択するか
	pub doesSelectOnTargetDown:u8,

	/// NAME: pad - pad
	pub pad1:[u8;1],

	/// NAME: Minimum reach judgment distance - 最小到達判定距離
	/// DESC: Minimum reach judgment distance - 最小到達判定距離
	pub minArriveDistance:f32,

	/// NAME: Maximum reach judgment distance - 最大到達判定距離
	/// DESC: Maximum reach judgment distance - 最大到達判定距離
	pub maxArriveDistance:f32,

	/// NAME: Continuous attack execution distance - 連続攻撃実行距離
	/// DESC: Distance used to determine the execution of attacks from the second stage onward - 二段目以降の攻撃の実行判定に使用する距離
	pub comboExecDistance:f32,

	/// NAME: Continuous attack execution angle - 連続攻撃実行角度
	/// DESC: Distance used to determine the execution of attacks from the second stage onward - 二段目以降の攻撃の実行判定に使用する距離
	pub comboExecRange:f32,
}

impl Paramdef for AI_ATTACK_PARAM_ST {
const NAME: &'static str = "AI_ATTACK_PARAM_ST";
const VERSION: u16 = 1;
}
