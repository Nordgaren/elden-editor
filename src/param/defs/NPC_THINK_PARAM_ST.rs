/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NPC_THINK_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Logic script ID - ロジックスクリプトID
	/// DESC: Set the ID of the logic created by the script. - スクリプトで作成したロジックのIDを設定します。
	pub logicId:i32,

	/// NAME: Battle goal ID - 戦闘ゴールID
	/// DESC: Battle goal ID - 戦闘ゴールID
	pub battleGoalID:i32,

	/// NAME: Searching enemy vision_distance [m] - 索敵視覚_距離[m]
	/// DESC: Search range by visual search. - 索敵視覚による索敵範囲.
	pub searchEye_dist:u16,

	/// NAME: Searching enemy vision_angle (width) [deg] - 索敵視覚_角度（幅）[deg]
	/// DESC: Search range by visual search. - 索敵視覚による索敵範囲.
	pub searchEye_angY:u8,

	/// NAME: Do you not detour a huge enemy? - 巨大敵を迂回しないか
	/// DESC: Do you not detour a huge enemy? - 巨大敵を迂回しないか
	pub Bitfield2:u8,

	/// NAME: Special effects ID for ranged attacks - 遠隔攻撃用特殊効果ID
	/// DESC: Special effects ID for ranged attacks - 遠隔攻撃用特殊効果ID
	pub spEffectId_RangedAttack:i32,

	/// NAME: Search enemy Lv1 Target Forget time [sec] - 索敵Lv1ターゲット忘れる時間[sec]
	/// DESC: Search enemy Lv1 Target Time to forget. - 索敵Lv1ターゲット忘れる時間。
	pub searchTargetLv1ForgetTime:f32,

	/// NAME: Search enemy Lv2 Target Forget time [sec] - 索敵Lv2ターゲット忘れる時間[sec]
	/// DESC: Search enemy Lv2 time to forget the target. - 索敵Lv2ターゲット忘れる時間。
	pub searchTargetLv2ForgetTime:f32,

	/// NAME: Back Home time when touching an enemy wall [sec] - 敵壁接触時のBackHome時間[sec]
	/// DESC: The life of the BackToHome goal when it touches an enemy wall that blocks the block - ブロックをさえぎる敵壁に接触したとき、BackToHomeゴールの寿命
	pub BackHomeLife_OnHitEneWal:f32,

	/// NAME: Time to forget the visual target [sec] - 視覚ターゲット忘れる時間[sec]
	/// DESC: Time to forget the visual target. - 視覚ターゲット忘れる時間。
	pub SightTargetForgetTime:f32,

	/// NAME: EzState number to do when you get stuck - 動けなくなったときに行うEzState番号
	/// DESC: An action that automatically takes place when a destructible object stops it. - 破壊可能なオブジェクトによって動きが止まっている場合、自動的に実行する行動。
	pub idAttackCannotMove:i32,

	/// NAME: Hearing_distance [m] - 聴覚_距離[m]
	/// DESC: Hearing search range ... - 聴覚による索敵範囲.。
	pub ear_dist:f32,

	/// NAME: Companion call response action anime ID - 仲間呼び 応答アクションアニメID
	/// DESC: Animation ID (EzStateAnimID) when responding - 応答する時のアニメID(EzStateAnimID)
	pub callHelp_ActionAnimId:i32,

	/// NAME: Fellow call _ Fellow call action ID - 仲間呼び_仲間呼びアクションID
	/// DESC: Action ID (EzStateAnimID) when calling a friend - 仲間呼ぶときのアクションID(EzStateAnimID)
	pub callHelp_CallActionId:i32,

	/// NAME: Visual_distance [m] - 視覚_距離[m]
	/// DESC: Visual search range. - 視覚による索敵範囲.
	pub eye_dist:u16,

	/// NAME: Whether to use a guard during action - 行動時ガード使用するか
	/// DESC: Whether to guard against actions (assumed when returning home, looking toward the target) - 行動にガードするか（帰宅時、ターゲットの方を見ている時想定）
	pub isGuard_Act:u8,

	/// NAME: Padding - パディング
	pub pad6:[u8;1],

	/// NAME: Hearing influence cut distance [m] - 聴覚　影響カット距離[m]
	/// DESC: The distance to reduce the size of the sound source. You will not hear any sound below this distance. - 音源のサイズを小さくする距離。この距離未満の音が聞こえなくなります。
	pub ear_soundcut_dist:u16,

	/// NAME: Smell _ distance [m] - 嗅覚_距離[m]
	/// DESC: Search range by smell. - 嗅覚による索敵範囲.
	pub nose_dist:u16,

	/// NAME: Distance to go home no matter what [m] - 何があっても帰宅する距離[m]
	/// DESC: COMMON_SetBattleActLogic argument - COMMON_SetBattleActLogicの引き数
	pub maxBackhomeDist:u16,

	/// NAME: Distance to return home while fighting [m] - 戦闘しつつ帰宅する距離[m]
	/// DESC: COMMON_SetBattleActLogic argument - COMMON_SetBattleActLogicの引き数
	pub backhomeDist:u16,

	/// NAME: Distance to give up and fight to return to the nest [m] - 巣に帰るのをあきらめて戦闘する距離[m]
	/// DESC: COMMON_SetBattleActLogic argument - COMMON_SetBattleActLogicの引き数
	pub backhomeBattleDist:u16,

	/// NAME: Non-combat action time when conscious of the enemy [sec] - 敵を意識しているときの非戦闘行動時間[sec]
	/// DESC: COMMON_SetBattleActLogic argument - COMMON_SetBattleActLogicの引き数
	pub nonBattleActLife:u16,

	/// NAME: When returning home: Time to look at the target [sec] - 帰宅時：ターゲットを見ている時間[sec]
	/// DESC: When returning home: Time to look at the target [sec] - 帰宅時：ターゲットを見ている時間[sec]
	pub BackHome_LookTargetTime:u16,

	/// NAME: When returning home: Distance to see the target [m] - 帰宅時：ターゲットを見ている距離[m]
	/// DESC: When returning home: Distance to see the target [m] - 帰宅時：ターゲットを見ている距離[m]
	pub BackHome_LookTargetDist:u16,

	/// NAME: Time to forget the sound target [sec] - 音ターゲット忘れる時間[sec]
	/// DESC: Time to forget the sound target. - 音ターゲット忘れる時間。
	pub SoundTargetForgetTime:f32,

	/// NAME: Battle start distance [m] - 戦闘開始距離[m]
	pub BattleStartDist:u16,

	/// NAME: Calling friends Your fellow group ID - 仲間呼び 自分の仲間グループID
	/// DESC: My fellow group ID - 自分の仲間グループID
	pub callHelp_MyPeerId:u16,

	/// NAME: Companion call Companion group ID - 仲間呼び 呼ぶ仲間グループID
	/// DESC: Companion group ID to call a companion - 仲間を呼ぶ対象となる仲間グループID
	pub callHelp_CallPeerId:u16,

	/// NAME: Damage impact rate [%] - ダメージ影響率[％]
	/// DESC: Damage impact rate acquisition (target system evaluation information) - ダメージ影響率取得(ターゲットシステム評価情報)
	pub targetSys_DmgEffectRate:u16,

	/// NAME: Team attack influence [0-100] - チーム攻撃影響力[0-100]
	/// DESC: A value for determining the number of simultaneous attacks in a team. Increasing the value reduces the number of people who can participate in the attack at the same time. - チーム内の同時攻撃人数を決めるための値。値を大きくすると、同時に攻撃参加できる人数が少なくなる。
	pub TeamAttackEffectivity:u8,

	/// NAME: Vision_angle (height) [deg] - 視覚_角度（高さ）[deg]
	/// DESC: Visual search range. - 視覚による索敵範囲.
	pub eye_angX:u8,

	/// NAME: Visual_angle (width) [deg] - 視覚_角度（幅）[deg]
	/// DESC: Visual search range. - 視覚による索敵範囲.
	pub eye_angY:u8,

	/// NAME: Does not affect the darkness - 暗闇影響しない
	/// DESC: Whether the alert vision_distance and battle start distance are not affected by the darkness - 警戒視覚_距離、戦闘開始距離が暗闇による影響を受けないようにするか
	pub disableDark:u8,

	/// NAME: Role in the caravan - キャラバンでの役割
	/// DESC: Role in the caravan - キャラバンでの役割
	pub caravanRole:u8,

	/// NAME: Companion call_minimum distance to the target [m] - 仲間呼び_ターゲットとの最低距離[m]
	/// DESC: If it is closer than this value, you cannot call a friend. - この値より近い場合は仲間呼びできない.
	pub callHelp_CallValidMinDistTarget:u8,

	/// NAME: Calling friends_Effective distance to call friends [m] - 仲間呼び_仲間を呼ぶ有効距離[m]
	/// DESC: Do not call if the companion is farther than this value. - この値より仲間が遠い場合は呼ばない。
	pub callHelp_CallValidRange:u8,

	/// NAME: Time to forget after answering a fellow call [sec] - 仲間呼び 応答してから忘れる時間[sec]
	/// DESC: Time to respond - 応答する時間
	pub callHelp_ForgetTimeByArrival:u8,

	/// NAME: Minimum waiting time for response [ssm => ss. mSec] - 応答時の待機最小時間[ssm=>ss．mSec]
	/// DESC: Minimum time for the first wait goal of the response goal [101 => 10.1sec] - 応答ゴールの最初の待機ゴールでの最小時間[101=>10．1sec]
	pub callHelp_MinWaitTime:u8,

	/// NAME: Maximum waiting time for response [ssm => ss. mSec] - 応答時の待機最大時間[ssm=>ss．mSec]
	/// DESC: Maximum time for the first waiting goal of the response goal [101 => 10.1sec] - 応答ゴールの最初の待機ゴールでの最大時間[101=>10．1sec]
	pub callHelp_MaxWaitTime:u8,

	/// NAME: Goal action: alert / normal sound - ゴールアクション：警戒状態/通常音
	/// DESC: Goal action: Target becomes alert due to normal sound detection - ゴールアクション：ターゲットが通常音の感知により警戒状態になった
	pub goalAction_ToCaution:u8,

	/// NAME: Hearing_audible AI sound level - 聴覚_可聴AI音レベル
	/// DESC: How good your ears are - どれくらい耳が良いのか
	pub ear_listenLevel:u8,

	/// NAME: Behavior type after responding to a fellow call - 仲間呼び 応答後の行動タイプ
	/// DESC: Action type to target position after response - 応答後、目標位置までの行動タイプ
	pub callHelp_ReplyBehaviorType:u8,

	/// NAME: Do not move the path - パス移動しない
	/// DESC: Whether to move directly without following the path even if a path move command comes - パス移動命令が来てもパスを辿らずに直接移動するか
	pub disablePathMove:u8,

	/// NAME: Do you want to skip the arrival judgment by the line of sight? - 視線による到着判定をスキップするか？
	/// DESC: Do you want to skip the arrival judgment by the line of sight? When set to On, arrival judgment is performed even if the line of sight does not pass. - 視線による到着判定をスキップするか？Onにすると、視線が通っていなくても、到着判定を行う。
	pub skipArrivalVisibleCheck:u8,

	/// NAME: Will you be a companion? - 取巻き役になるか？
	/// DESC: Thinking attribute: When turned on, it plays a role of surrounding. - 思考属性：ＯＮにすると取巻き役を演じます。
	pub thinkAttr_doAdmirer:u8,

	/// NAME: Can you pass the flag "cliff"? - フラグ「崖」通れるか？
	/// DESC: Can you pass through the node "cliff"? (def: 1) - ノード「崖」を通過できるか？(def:1)
	pub Bitfield3:u8,

	/// NAME: Really booked - ほんとに予約
	/// DESC: If you need a new flag, put it here (Not Padding) - フラグが新しく必要になったらここにいれます（NotPadding)
	pub enableNaviFlg_reserve1:[u8;3],

	/// NAME: Search enemy Lv0 → Lv1 threshold - 索敵Lv0→Lv1の閾値
	/// DESC: Search enemy Lv0 → Lv1 threshold - 索敵Lv0→Lv1の閾値
	pub searchThreshold_Lv0toLv1:i32,

	/// NAME: Search enemy Lv1 → Lv2 threshold - 索敵Lv1→Lv2の閾値
	/// DESC: Search enemy Lv1 → Lv2 threshold - 索敵Lv1→Lv2の閾値
	pub searchThreshold_Lv1toLv2:i32,

	/// NAME: Platoon reaction delay time [sec] - 小隊反応遅延時間[sec]
	/// DESC: Platoon reaction delay time [sec] - 小隊反応遅延時間[sec]
	pub platoonReplyTime:f32,

	/// NAME: Platoon reaction additional random time [sec] - 小隊反応追加ランダム時間[sec]
	/// DESC: Platoon reaction additional random time [sec] - 小隊反応追加ランダム時間[sec]
	pub platoonReplyAddRandomTime:f32,

	/// NAME: Searching enemy vision_angle (height) [deg] - 索敵視覚_角度(高さ)[deg]
	/// DESC: Searching enemy vision_angle (height) [deg] - 索敵視覚_角度(高さ)[deg]
	pub searchEye_angX:u8,

	/// NAME: Do you want to overwrite the combat view? - 戦闘視界を上書きするか？
	/// DESC: Do you want to overwrite the battle view? - 戦闘視界を上書きするか
	pub isUpdateBattleSight:u8,

	/// NAME: Battle Vision_Overwrite Distance [m] - 戦闘視覚_上書き距離[m]
	/// DESC: Battle Vision_Overwrite Distance [m] - 戦闘視覚_上書き距離[m]
	pub battleEye_updateDist:u16,

	/// NAME: Battle Vision_Overwrite Angle (Height) [deg] - 戦闘視覚_上書き角度(高さ)[deg]
	/// DESC: Battle Vision_Overwrite Angle (Height) [deg] - 戦闘視覚_上書き角度(高さ)[deg]
	pub battleEye_updateAngX:u8,

	/// NAME: Battle Vision_Overwrite Angle (Width) [deg] - 戦闘視覚_上書き角度(幅)[deg]
	/// DESC: Battle Vision_Overwrite Angle (Width) [deg] - 戦闘視覚_上書き角度(幅)[deg]
	pub battleEye_updateAngY:u8,

	/// NAME: Padding - パディング
	pub pad4:[u8;16],

	/// NAME: Visual_occurrence distance [m] - 視覚_発生距離[m]
	/// DESC: The viewing angle start position is behind this distance from the center of the character. - キャラの中心からこの距離後ろが視角開始位置になる
	pub eye_BackOffsetDist:u16,

	/// NAME: Visual_cut distance [m] - 視覚_カット距離[m]
	/// DESC: This distance is not recognized from the viewing angle generation position - 視角発生位置からこの距離は認識しない
	pub eye_BeginDist:u16,

	/// NAME: Behavior type at the time of path search failure / homecoming limit - パス検索失敗/帰巣限界時の行動種別
	/// DESC: Default action type to be performed when the path search fails, when the end point of the alternative path is reached / when the homecoming limit distance is reached - パス検索失敗時、代替パスの終点に到達した際/帰巣限界距離まで到達した際に行うデフォルトの行動種別
	pub actTypeOnFailedPath:u8,

	/// NAME: Goal Action: Alert / Important Sound - ゴールアクション：警戒状態/重要音
	/// DESC: Goal action: Target becomes alert due to detection of important sound - ゴールアクション：ターゲットが重要音の感知により警戒状態になった
	pub goalAction_ToCautionImportant:u8,

	/// NAME: Changeover animation ID for ranged attacks - 遠隔攻撃用持ち替えアニメID
	/// DESC: Reference ID for AI attack parameters - AI攻撃パラメータの参照ID
	pub shiftAnimeId_RangedAttack:i32,

	/// NAME: Behavior when path search fails (during non-combat) - パス検索失敗時の挙動（非戦闘中）
	/// DESC: Target [None] Sometimes, the action to be taken after rewriting the current location to a nest - ターゲット【なし】時に、現在地点を巣に書き換えた後に取る行動 
	pub actTypeOnNonBtlFailedPath:u8,

	/// NAME: Buddy AI - バディAI
	/// DESC: Thinking for a buddy - バディ用の思考か
	pub isBuddyAI:u8,

	/// NAME: Goal action: Search enemy Lv1 - ゴールアクション：索敵Lv1
	/// DESC: Goal action: Target becomes Lv1 - ゴールアクション：ターゲットが索敵Lv1になった
	pub goalAction_ToSearchLv1:u8,

	/// NAME: Goal action: Search enemy Lv2 - ゴールアクション：索敵Lv2
	/// DESC: Goal action: Target became enemy Lv2 - ゴールアクション：ターゲットが索敵Lv2になった
	pub goalAction_ToSearchLv2:u8,

	/// NAME: Do you use the edge "jump" (non-combat state)? - エッジ「ジャンプ」使うか（非戦闘状態）
	/// DESC: Whether to jump over the user edge for jumping (non-combat state) - ジャンプ用ユーザーエッジを飛び越えて移動するか(非戦闘状態)
	pub enableJumpMove:u8,

	/// NAME: Do not move around - 回避移動しない
	/// DESC: Do you want to turn off the behavior (local steering) that tries to move while avoiding other characters? - 他のキャラクターを回避しながら移動しようとする挙動(ローカルステアリング)をオフにするか？
	pub disableLocalSteering:u8,

	/// NAME: Goal action: Memory target state - ゴールアクション：記憶ターゲット状態
	/// DESC: Goal action: Lost target - ゴールアクション：ターゲットを見失った
	pub goalAction_ToDisappear:u8,

	/// NAME: Goal action: Start normal state - ゴールアクション：通常状態開始
	/// DESC: Action when transitioning to the normal state - 通常状態に遷移したときのアクション
	pub changeStateAction_ToNormal:u8,

	/// NAME: Time to forget memory target [sec] - 記憶ターゲット忘れる時間[sec]
	/// DESC: Time to forget the memory target. - 記憶ターゲット忘れる時間。
	pub MemoryTargetForgetTime:f32,

	/// NAME: Ranged attack anime ID - 遠隔攻撃アニメID
	/// DESC: A parameter that specifies the attack ID issued by the enemy when making a ranged attack - 遠隔攻撃する際にエネミーが出す攻撃IDを指定するパラメータ
	pub rangedAttackId:i32,

	/// NAME: Do you use the edge "jump" (non-combat state)? - エッジ「飛び降り」使うか（非戦闘状態）
	/// DESC: Allows AI to pass the jumping edge in non-combat state - AIが非戦闘状態で、飛び降りエッジを通行できるようにする
	pub useFall_onNormalCaution:u8,

	/// NAME: Do you use the edge "jump" (combat state)? - エッジ「飛び降り」使うか（戦闘状態）
	/// DESC: Allow AI to pass through the jumping edge in combat - AIが戦闘状態で、飛び降りエッジを通行できるようにする 
	pub useFall_onSearchBattle:u8,

	/// NAME: Do you use the edge "jump" (combat state)? - エッジ「ジャンプ」使うか（戦闘状態）
	/// DESC: Whether to jump over the user edge for jumping (combat state) - ジャンプ用ユーザーエッジを飛び越えて移動するか(戦闘状態)
	pub enableJumpMove_onBattle:u8,

	/// NAME: Behavior when addicted to the homecoming limit - 帰巣限界でハマった時の挙動
	/// DESC: Behavior when addicted to the homecoming limit - 帰巣限界でハマった時の挙動
	pub backToHomeStuckAct:u8,

	/// NAME: Padding - パディング
	/// DESC: pad - pad
	pub pad3:[u8;4],

	/// NAME: Behavior ID1 - 振る舞いID1
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId01:i32,

	/// NAME: Behavior ID2 - 振る舞いID2
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId02:i32,

	/// NAME: Behavior ID3 - 振る舞いID3
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId03:i32,

	/// NAME: Behavior ID4 - 振る舞いID4
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId04:i32,

	/// NAME: Behavior ID5 - 振る舞いID5
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId05:i32,

	/// NAME: Behavior ID6 - 振る舞いID6
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId06:i32,

	/// NAME: Behavior ID7 - 振る舞いID7
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId07:i32,

	/// NAME: Behavior ID8 - 振る舞いID8
	/// DESC: Corresponds to the behavior ID of the sound target that can be heard - 聴くことのできる音ターゲットの振る舞いIDに対応
	pub soundBehaviorId08:i32,

	/// NAME: Special effect ID at the time of sword delivery - 納刀時特殊効果ID
	/// DESC: Special effect ID at the time of sword delivery - 納刀時特殊効果ID
	pub weaponOffSpecialEffectId:i32,

	/// NAME: Special effect ID when pulling out a sword - 抜刀時特殊効果ID
	/// DESC: Special effect ID when pulling out a sword - 抜刀時特殊効果ID
	pub weaponOnSpecialEffectId:i32,

	/// NAME: Katana Anime ID - 納刀アニメID
	/// DESC: Katana Anime ID - 納刀アニメID
	pub weaponOffAnimId:i32,

	/// NAME: Sword animation ID - 抜刀アニメID
	/// DESC: Sword animation ID - 抜刀アニメID
	pub weaponOnAnimId:i32,

	/// NAME: Amazing anime ID - 驚くアニメID
	/// DESC: Amazing anime ID - 驚くアニメID
	pub surpriseAnimId:i32,
}

impl Paramdef for NPC_THINK_PARAM_ST {
const NAME: &'static str = "NPC_THINK_PARAM_ST";
const VERSION: u16 = 2;
}
impl NPC_THINK_PARAM_ST {
	/// Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	/// Bitfield1
	pub fn get_disableParam_NT(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_disableParam_NT(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Reserve for package output 1 - パッケージ出力用リザーブ1
	/// Bitfield1
	pub fn get_disableParamReserve1(&self) -> u8 {
		&self.Bitfield1 & 0xFE
	}

	/// Bitfield1 MAX: 127
	pub fn set_disableParamReserve1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield1 & 0x1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1
		}
	}	/// Do you not detour a huge enemy? - 巨大敵を迂回しないか
	/// Bitfield2
	pub fn get_isNoAvoidHugeEnemy(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isNoAvoidHugeEnemy(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Do you want to pull out the sword? - 納刀抜刀するか
	/// Bitfield2
	pub fn get_enableWeaponOnOff(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_enableWeaponOnOff(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Do you aim for Rock Damipoli (for enemies)? - ロックダミポリ(エネミー用)を狙うか
	/// Bitfield2
	pub fn get_targetAILockDmyPoly(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_targetAILockDmyPoly(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// 
	/// Bitfield2
	pub fn get_pad8(&self) -> u8 {
		&self.Bitfield2 & 0xF8
	}

	/// Bitfield2 MAX: 31
	pub fn set_pad8(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 3) & 0xF8;
			let newVal = &self.Bitfield2 & 0x7 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0x7
		}
	}	/// Can you pass through the node "cliff"? (def: 1) - ノード「崖」を通過できるか？(def:1)
	/// Bitfield3
	pub fn get_enableNaviFlg_Edge(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_Edge(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Can you pass through the node "wide"? (def: 1) - ノード「広い」を通過できるか？(def:1)
	/// Bitfield3
	pub fn get_enableNaviFlg_LargeSpace(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_LargeSpace(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Can you pass through the node "ladder"? (def: 0) - ノード「梯子」を通過できるか？(def:0)
	/// Bitfield3
	pub fn get_enableNaviFlg_Ladder(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_Ladder(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Can you go through the node "hole"? (def: 0) - ノード「穴」を通過できるか？(def:0)
	/// Bitfield3
	pub fn get_enableNaviFlg_Hole(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_Hole(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Can you pass through the node "door"? (def: 0) - ノード「扉」を通過できるか？(def:0)
	/// Bitfield3
	pub fn get_enableNaviFlg_Door(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_Door(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Can you pass through the node "in the wall"? (def: 0) - ノード「壁中」を通過できるか？(def:0)
	/// Bitfield3
	pub fn get_enableNaviFlg_InSideWall(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_InSideWall(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Can you pass through the node "lava"? (def: 0) - ノード「溶岩」を通過できるか？(def:0)
	/// Bitfield3
	pub fn get_enableNaviFlg_Lava(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_Lava(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Can you pass the node "cliff" under normal / alert conditions? (def: 1) - 通常／警戒状態でノード「崖」を通過できるか？(def:1)
	/// Bitfield3
	pub fn get_enableNaviFlg_Edge_Ordinary(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_enableNaviFlg_Edge_Ordinary(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}

}
