/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MOVE_PARAM_ST {

	/// NAME: stand-by - 待機
	/// DESC: stand-by - 待機
	pub stayId:i32,

	/// NAME: Before walking - 歩行 前
	/// DESC: Before walking - 歩行 前
	pub walkF:i32,

	/// NAME: After walking - 歩行 後
	/// DESC: After walking - 歩行 後
	pub walkB:i32,

	/// NAME: Walking left - 歩行 左
	/// DESC: Walking left - 歩行 左
	pub walkL:i32,

	/// NAME: Walking right - 歩行 右
	/// DESC: Walking right - 歩行 右
	pub walkR:i32,

	/// NAME: Before driving - 走行 前
	/// DESC: Before driving - 走行 前
	pub dashF:i32,

	/// NAME: After running - 走行 後
	/// DESC: After running - 走行 後
	pub dashB:i32,

	/// NAME: Driving left - 走行 左
	/// DESC: Driving left - 走行 左
	pub dashL:i32,

	/// NAME: Driving right - 走行 右
	/// DESC: Driving right - 走行 右
	pub dashR:i32,

	/// NAME: Dash move - ダッシュ移動
	/// DESC: Dash move - ダッシュ移動
	pub superDash:i32,

	/// NAME: Before emergency avoidance - 緊急回避 前
	/// DESC: Before emergency avoidance - 緊急回避 前
	pub escapeF:i32,

	/// NAME: After emergency avoidance - 緊急回避 後
	/// DESC: After emergency avoidance - 緊急回避 後
	pub escapeB:i32,

	/// NAME: Emergency avoidance left - 緊急回避 左
	/// DESC: Emergency avoidance left - 緊急回避 左
	pub escapeL:i32,

	/// NAME: Emergency avoidance right - 緊急回避 右
	/// DESC: Emergency avoidance right - 緊急回避 右
	pub escapeR:i32,

	/// NAME: 90 degree turn left - 90度旋回 左
	/// DESC: 90 degree turn left - 90度旋回 左
	pub turnL:i32,

	/// NAME: 90 degree turn right - 90度旋回 右
	/// DESC: 90 degree turn right - 90度旋回 右
	pub trunR:i32,

	/// NAME: 180 degree turn left - 180度旋回 左
	/// DESC: 180 degree turn left - 180度旋回 左
	pub largeTurnL:i32,

	/// NAME: 180 degree turn right - 180度旋回 右
	/// DESC: 180 degree turn right - 180度旋回 右
	pub largeTurnR:i32,

	/// NAME: Step move - ステップ移動
	/// DESC: 180 degree turn right - 180度旋回 右
	pub stepMove:i32,

	/// NAME: Flight standby - 飛行待機
	/// DESC: Flight standby - 飛行待機
	pub flyStay:i32,

	/// NAME: Flight advance - 飛行前進
	/// DESC: Flight advance - 飛行前進
	pub flyWalkF:i32,

	/// NAME: Fly left forward - 飛行左前進
	/// DESC: Fly left forward. Low rotation - 飛行左前進。低回転
	pub flyWalkFL:i32,

	/// NAME: Flight right forward - 飛行右前進
	/// DESC: Fly right forward. Low rotation - 飛行右前進。低回転
	pub flyWalkFR:i32,

	/// NAME: Flight left forward 2 - 飛行左前進2
	/// DESC: Flight left forward 2. High rotation - 飛行左前進2。高回転
	pub flyWalkFL2:i32,

	/// NAME: Flight right forward 2 - 飛行右前進2
	/// DESC: Flight right forward 2. High rotation - 飛行右前進2。高回転
	pub flyWalkFR2:i32,

	/// NAME: High speed flight advance - 高速飛行前進
	/// DESC: High speed flight advance - 高速飛行前進
	pub flyDashF:i32,

	/// NAME: High speed flight left forward - 高速飛行左前進
	/// DESC: High-speed flight left forward. Low rotation - 高速飛行左前進。低回転
	pub flyDashFL:i32,

	/// NAME: High speed flight right forward - 高速飛行右前進
	/// DESC: High speed flight right forward. Low rotation - 高速飛行右前進。低回転
	pub flyDashFR:i32,

	/// NAME: High speed flight left forward 2 - 高速飛行左前進2
	/// DESC: High speed flight left forward 2. High rotation - 高速飛行左前進2。高回転
	pub flyDashFL2:i32,

	/// NAME: High speed flight right forward 2 - 高速飛行右前進2
	/// DESC: High speed flight right forward 2. High rotation - 高速飛行右前進2。高回転
	pub flyDashFR2:i32,

	/// NAME: Before dash emergency avoidance - ダッシュ緊急回避前
	/// DESC: Before dash emergency avoidance - ダッシュ緊急回避前
	pub dashEscapeF:i32,

	/// NAME: After dash emergency avoidance - ダッシュ緊急回避後
	/// DESC: After dash emergency avoidance - ダッシュ緊急回避後
	pub dashEscapeB:i32,

	/// NAME: Dash emergency avoidance left - ダッシュ緊急回避左
	/// DESC: Dash emergency avoidance left - ダッシュ緊急回避左
	pub dashEscapeL:i32,

	/// NAME: Dash emergency avoidance right - ダッシュ緊急回避右
	/// DESC: Dash emergency avoidance right - ダッシュ緊急回避右
	pub dashEscapeR:i32,

	/// NAME: Analog moving para ID - アナログ移動パラＩＤ
	/// DESC: Mobile animation parameter ID used in mobile animation blend - 移動アニメブレンドで使用される移動アニメパラメータＩＤ
	pub analogMoveParamId:i32,

	/// NAME: No animation turning angle [deg] - アニメなし旋回角度[deg]
	/// DESC: If the turning angle is less than this value, the turning animation will not be played (only the enemy turning control is effective). - 旋回角度がこの値以下だと旋回アニメを再生しません（敵旋回制御のみ有効）
	pub turnNoAnimAngle:u8,

	/// NAME: 45 degree turning animation angle [deg] - 45度旋回アニメ角度[deg]
	/// DESC: If the turning angle is less than this value, the 45 degree turning animation will be played (only 2 bosses are valid). - 旋回角度がこの値以下だと45度旋回アニメを再生します（ボス2足のみ有効）
	pub turn45Angle:u8,

	/// NAME: 90 degree turning animation angle [deg] - 90度旋回アニメ角度[deg]
	/// DESC: If the turning angle is less than this value, the 90 degree turning animation will be played (only the enemy turning control is effective). - 旋回角度がこの値以下だと90度旋回アニメを再生します（敵旋回制御のみ有効）
	pub turn90Angle:u8,

	/// NAME: No animation when stopped Turning angle [deg] - 停止時アニメなし旋回角度[deg]
	/// DESC: If the turning angle is less than this value, the turning animation will not be played [when stopped] (only valid for 2 boss legs) - 旋回角度がこの値以下だと旋回アニメを再生しません[停止時]（ボス2足のみ有効）
	pub turnWaitNoAnimAngle:u8,
}

