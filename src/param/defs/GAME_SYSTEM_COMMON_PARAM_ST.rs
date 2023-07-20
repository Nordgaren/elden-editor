/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct GAME_SYSTEM_COMMON_PARAM_ST {
    /// NAME: Basic toughness durability recovery time - 基本強靭度耐久値回復時間
    /// DESC: This is the basic value of toughness recovery time. (Seconds) - 強靭度回復時間の基本値です。（秒）
    pub baseToughnessRecoverTime: f32,

    /// NAME: Character event turning animation (90 ° left) - キャラのイベント旋回アニメーション（左90°）
    /// DESC: This is a 90 ° left turn animation for the "Character Turn" event. - 「キャラの旋回」イベント用の左90°旋回アニメーションです。
    pub chrEventTrun_byLeft90: i32,

    /// NAME: Character event turning animation (right 90 °) - キャラのイベント旋回アニメーション（右90°）
    /// DESC: This is a 90 ° right turn animation for the "Character Turn" event. - 「キャラの旋回」イベント用の右90°旋回アニメーションです。
    pub chrEventTrun_byRight90: i32,

    /// NAME: Character event turning animation (180 ° left) - キャラのイベント旋回アニメーション（左180°）
    /// DESC: This is a 180 ° left turn animation for the "Character Turn" event. - 「キャラの旋回」イベント用の左180°旋回アニメーションです。
    pub chrEventTrun_byLeft180: i32,

    /// NAME: Character event turning animation (right 180 °) - キャラのイベント旋回アニメーション（右180°）
    /// DESC: This is a 180 ° right turn animation for the "Character Turn" event. - 「キャラの旋回」イベント用の右180°旋回アニメーションです。
    pub chrEventTrun_byRight180: i32,

    /// NAME: Character event turning 90 ° Animation start angle - キャラのイベント旋回90°アニメーション開始角度
    /// DESC: The starting angle of the angle to which the 90 ° swivel animation for the "Character Swirl" event is applied. If the event starts at an angle smaller than this angle, a system turn will occur. - 「キャラの旋回」イベント用の90°旋回アニメーションを適用する角度の開始角度。この角度より小さい角度でイベントが始まった場合は、システム旋回が行われます
    pub chrEventTrun_90TurnStartAngle: i16,

    /// NAME: Character event turning 180 ° Animation start angle - キャラのイベント旋回180°アニメーション開始角度
    /// DESC: The starting angle of the angle to which the 180 ° swivel animation for the "Character Swirl" event is applied. - 「キャラの旋回」イベント用の180°旋回アニメーションを適用する角度の開始角度。
    pub chrEventTrun_180TurnStartAngle: i16,

    /// NAME: Stealth attack damage multiplier - ステルス攻撃被ダメージ倍率
    /// DESC: Stealth attack damage multiplier - ステルス攻撃被ダメージ倍率
    pub stealthAtkDamageRate: f32,

    /// NAME: Damage cut rate when the guard is successful when repelled - はじかれ時ガード成功時ダメージカット率
    /// DESC: Damage cut rate when the guard is successful when repelled. Multiply the final damage - はじかれ時ガード成功時ダメージカット率。最終ダメージに乗算
    pub flickDamageCutRateSuccessGurad: f32,

    /// NAME: Difference angle to start playing NPC conversation animation - NPC会話のアニメ再生開始する差分角度
    /// DESC: This is the difference angle at which the animation playback of the motion during the conversation of the NPC conversation starts. - NPC会話の会話中モーションのアニメ再生開始する差分角度です。
    pub npcTalkAnimBeginDiffAngle: f32,

    /// NAME: Difference angle to stop animation playback of NPC conversation - NPC会話のアニメ再生停止する差分角度
    /// DESC: This is the difference angle at which the animation playback of the motion during the conversation of the NPC conversation is stopped. - NPC会話の会話中モーションのアニメ再生停止する差分角度です。
    pub npcTalkAnimEndDiffAngle: f32,

    /// NAME: Nemuri Item Acquisition Range_Action Button Para ID - ネムリアイテム取得範囲_アクションボタンパラID
    /// DESC: Nemuri Item Acquisition Range_Action Button Para ID. TAE flag Used as the default value when not overwriting with "Event> Nemuri item registration". - ネムリアイテム取得範囲_アクションボタンパラID。TAEフラグ「イベント＞ネムリアイテム登録」で上書きしないときのデフォルト値として使われる。
    pub sleepCollectorItemActionButtonParamId: i32,

    /// NAME: Buddy item permission_SFX occurrence interval [s] - バディアイテム許可_SFX発生間隔[s]
    /// DESC: Buddy item permission_SFX occurrence interval [s] - バディアイテム許可_SFX発生間隔[s]
    pub allowUseBuddyItem_sfxInterval: f32,

    /// NAME: Buddy Item Permission_SFX Occurrence PC Damipoli ID - バディアイテム許可_SFX発生PCダミポリID
    /// DESC: Buddy Item Permission_SFX Occurrence PC Damipoli ID - バディアイテム許可_SFX発生PCダミポリID
    pub allowUseBuddyItem_sfxDmyPolyId: i32,

    /// NAME: Buddy item permission_SFX generated horse Damipoli ID_when riding - バディアイテム許可_SFX発生馬ダミポリID_騎乗時
    /// DESC: Buddy item permission_SFX generated horse Damipoli ID_when riding - バディアイテム許可_SFX発生馬ダミポリID_騎乗時
    pub allowUseBuddyItem_sfxDmyPolyId_horse: i32,

    /// NAME: Buddy Item Permission_Occurrence SFXID - バディアイテム許可_発生SFXID
    /// DESC: Buddy Item Permission_Occurrence SFXID - バディアイテム許可_発生SFXID
    pub allowUseBuddyItem_sfxId: i32,

    /// NAME: Buddy Summoning_Initial Range_SFX Occurrence Interval [s] - バディ召喚中_起動範囲内_SFX発生間隔[s]
    /// DESC: Buddy Summoning_Initial Range_SFX Occurrence Interval [s] - バディ召喚中_起動範囲内_SFX発生間隔[s]
    pub onBuddySummon_inActivateRange_sfxInterval: f32,

    /// NAME: Buddy Summoning_Initial Range_SFX Occurrence PC Damipoli ID - バディ召喚中_起動範囲内_SFX発生PCダミポリID
    /// DESC: Buddy Summoning_Initial Range_SFX Occurrence PC Damipoli ID - バディ召喚中_起動範囲内_SFX発生PCダミポリID
    pub onBuddySummon_inActivateRange_sfxDmyPolyId: i32,

    /// NAME: Buddy Summoning_Initial Range_SFX Generated Horse Damipoli ID_When Riding - バディ召喚中_起動範囲内_SFX発生馬ダミポリID_騎乗時
    /// DESC: Buddy Summoning_Initial Range_SFX Generated Horse Damipoli ID_When Riding - バディ召喚中_起動範囲内_SFX発生馬ダミポリID_騎乗時
    pub onBuddySummon_inActivateRange_sfxDmyPolyId_horse: i32,

    /// NAME: Buddy Summoning_Initial Range_Occurrence SFXID - バディ召喚中_起動範囲内_発生SFXID
    /// DESC: Buddy Summoning_Initial Range_Occurrence SFXID - バディ召喚中_起動範囲内_発生SFXID
    pub onBuddySummon_inActivateRange_sfxId: i32,

    /// NAME: Buddy Summoning_Initial range Special effect ID_For PC - バディ召喚中_起動範囲内特殊効果ID_PC用
    /// DESC: Buddy Summoning_Initial range Special effect ID_For PC - バディ召喚中_起動範囲内特殊効果ID_PC用
    pub onBuddySummon_inActivateRange_spEffectId_pc: i32,

    /// NAME: Buddy Summoning_Warning Range Special Effect ID_For PC - バディ召喚中_警告範囲内特殊効果ID_PC用
    /// DESC: Buddy Summoning_Warning Range Special Effect ID_For PC - バディ召喚中_警告範囲内特殊効果ID_PC用
    pub onBuddySummon_inWarnRange_spEffectId_pc: i32,

    /// NAME: Buddy Summoning_Buddy Return Special Effect ID_For PC - バディ召喚中_バディ帰還時特殊効果ID_PC用
    /// DESC: Buddy Summoning_Buddy Return Special Effect ID_For PC - バディ召喚中_バディ帰還時特殊効果ID_PC用
    pub onBuddySummon_atBuddyUnsummon_spEffectId_pc: i32,

    /// NAME: Buddy Summoning_Warning Range Special Effect ID_For Buddy - バディ召喚中_警告範囲内特殊効果ID_バディ用
    /// DESC: Buddy Summoning_Warning Range Special Effect ID_For Buddy - バディ召喚中_警告範囲内特殊効果ID_バディ用
    pub onBuddySummon_inWarnRange_spEffectId_buddy: i32,

    /// NAME: Morning in-game time (hours) - 朝のインゲーム時間（時）
    /// DESC: Morning in-game time (hours). Used in conversation. - 朝のインゲーム時間（時）。会話で使用します。
    pub morningIngameHour: u8,

    /// NAME: Morning in-game time (minutes) - 朝のインゲーム時間（分）
    /// DESC: Morning in-game time (minutes). Used in conversation. - 朝のインゲーム時間（分）。会話で使用します。
    pub morningIngameMinute: u8,

    /// NAME: Morning in-game time (seconds) - 朝のインゲーム時間（秒）
    /// DESC: Morning in-game time (seconds). Used in conversation. - 朝のインゲーム時間（秒）。会話で使用します。
    pub morningIngameSecond: u8,

    /// NAME: Daytime in-game time (hours) - 昼のインゲーム時間（時）
    /// DESC: Daytime in-game time (hours). Used in conversation. - 昼のインゲーム時間（時）。会話で使用します。
    pub noonIngameHour: u8,

    /// NAME: Daytime in-game time (minutes) - 昼のインゲーム時間（分）
    /// DESC: Daytime in-game time (minutes). Used in conversation. - 昼のインゲーム時間（分）。会話で使用します。
    pub noonIngameMinute: u8,

    /// NAME: Daytime in-game time (seconds) - 昼のインゲーム時間（秒）
    /// DESC: Daytime in-game time (seconds). Used in conversation. - 昼のインゲーム時間（秒）。会話で使用します。
    pub noonIngameSecond: u8,

    /// NAME: Night in-game time (hours) - 夜のインゲーム時間（時）
    /// DESC: Night in-game time (hours). Used in conversation. - 夜のインゲーム時間（時）。会話で使用します。
    pub nightIngameHour: u8,

    /// NAME: Night in-game time (minutes) - 夜のインゲーム時間（分）
    /// DESC: Night in-game time (minutes). Used in conversation. - 夜のインゲーム時間（分）。会話で使用します。
    pub nightIngameMinute: u8,

    /// NAME: Night in-game time (seconds) - 夜のインゲーム時間（秒）
    /// DESC: Night in-game time (seconds). Used in conversation. - 夜のインゲーム時間（秒）。会話で使用します。
    pub nightIngameSecond: u8,

    /// NAME: AI visibility magnification_morning_start time (hour) - AI視界倍率_朝_開始時刻(時)
    /// DESC: AI visibility magnification_morning_start time (hour) - AI視界倍率_朝_開始時刻(時)
    pub aiSightRateStart_Morning_Hour: u8,

    /// NAME: AI visibility magnification_morning_start time (minutes) - AI視界倍率_朝_開始時刻(分)
    /// DESC: AI visibility magnification_morning_start time (minutes) - AI視界倍率_朝_開始時刻(分)
    pub aiSightRateStart_Morning_Minute: u8,

    /// NAME: AI visibility magnification_day_start time (hour) - AI視界倍率_昼_開始時刻(時)
    /// DESC: AI visibility magnification_day_start time (hour) - AI視界倍率_昼_開始時刻(時)
    pub aiSightRateStart_Noon_Hour: u8,

    /// NAME: AI visibility magnification_day_start time (minutes) - AI視界倍率_昼_開始時刻(分)
    /// DESC: AI visibility magnification_day_start time (minutes) - AI視界倍率_昼_開始時刻(分)
    pub aiSightRateStart_Noon_Minute: u8,

    /// NAME: AI visibility magnification_evening_start time (hour) - AI視界倍率_夕_開始時刻(時)
    /// DESC: AI visibility magnification_evening_start time (hour) - AI視界倍率_夕_開始時刻(時)
    pub aiSightRateStart_Evening_Hour: u8,

    /// NAME: AI visibility magnification_evening_start time (minutes) - AI視界倍率_夕_開始時刻(分)
    /// DESC: AI visibility magnification_evening_start time (minutes) - AI視界倍率_夕_開始時刻(分)
    pub aiSightRateStart_Evening_Minute: u8,

    /// NAME: AI visibility magnification_night_start time (hour) - AI視界倍率_夜_開始時刻(時)
    /// DESC: AI visibility magnification_night_start time (hour) - AI視界倍率_夜_開始時刻(時)
    pub aiSightRateStart_Night_Hour: u8,

    /// NAME: AI visibility magnification_night_start time (minutes) - AI視界倍率_夜_開始時刻(分)
    /// DESC: AI visibility magnification_night_start time (minutes) - AI視界倍率_夜_開始時刻(分)
    pub aiSightRateStart_Night_Minute: u8,

    /// NAME: AI visibility magnification_midnight_start time (hour) - AI視界倍率_深夜_開始時刻(時)
    /// DESC: AI visibility magnification_midnight_start time (hour) - AI視界倍率_深夜_開始時刻(時)
    pub aiSightRateStart_Midnight_Hour: u8,

    /// NAME: AI visibility magnification_midnight_start time (minutes) - AI視界倍率_深夜_開始時刻(分)
    /// DESC: AI visibility magnification_midnight_start time (minutes) - AI視界倍率_深夜_開始時刻(分)
    pub aiSightRateStart_Midnight_Minute: u8,

    /// NAME: SA large damage hit production SFX_ Occurrence condition SA damage threshold ratio [%] - SA大ダメージヒット演出SFX_発生条件SAダメージ閾値比率[％]
    /// DESC: SA large damage hit production SFX_ Occurrence condition SA damage threshold ratio [%] - SA大ダメージヒット演出SFX_発生条件SAダメージ閾値比率[％]
    pub saLargeDamageHitSfx_Threshold: u8,

    /// NAME: SA heavy damage hit production SFX_SFXID - SA大ダメージヒット演出SFX_SFXID
    /// DESC: SA heavy damage hit production SFX_SFXID - SA大ダメージヒット演出SFX_SFXID
    pub saLargeDamageHitSfx_SfxId: i32,

    /// NAME: Distance to create a sign away from a safe position [m] - 安全位置から離れてサインを作成できる距離[m]
    /// DESC: Distance at which you can create a sign away from the last safe position on your PC [m] - PCの最後の安全位置から離れてサインを作成できる距離[m]
    pub signCreatableDistFromSafePos: f32,

    /// NAME: Distance between host and guest where resummoning occurs [m] - 再召喚が発生するホストとゲストの距離[m]
    /// DESC: Distance between host and guest where resummoning occurs [m] - 再召喚が発生するホストとゲストの距離[m]
    pub guestResummonDist: f32,

    /// NAME: Distance to notify the guest that they are about to leave the host [m] - ゲストがホストから離れそうになってることを通知する距離[m]
    /// DESC: Distance [m] to notify the guest that they are about to leave the host. Notify me when I'm farther than this distance. - ゲストがホストから離れそうになってることを通知する距離[m]。この距離より離れたら通知する。
    pub guestLeavingMessageDistMax: f32,

    /// NAME: Distance that allows guests to be re-notified that they are about to leave the host [m] - ゲストがホストから離れそうになってることを再通知可能にする距離[m]
    /// DESC: Distance [m] that allows the guest to be re-notified that they are about to leave the host. Do not re-notify until closer than this distance. - ゲストがホストから離れそうになってることを再通知可能にする距離[m]。この距離より近づくまで再通知しない。
    pub guestLeavingMessageDistMin: f32,

    /// NAME: Maximum distance the guest can be away from the host [m] - ゲストがホストから離れられる最大距離[m]
    /// DESC: Maximum distance [m] that a guest can leave the host. If a certain period of time elapses while the distance is longer than this distance, the session will be withdrawn. - ゲストがホストから離れられる最大距離[m]。この距離より離れた状態で一定時間経過するとセッション脱退する。
    pub guestLeaveSessionDist: f32,

    /// NAME: Retry area radius_default value (m) - リトライエリア半径_デフォルト値(m)
    /// DESC: Retry area radius_default value (m). It is used as the default value when the radius and area are not set in the event type "Retry Point" of MapStudio. - リトライエリア半径_デフォルト値(m)。MapStudioのイベントタイプ「リトライポイント」で半径も領域も未設定の場合のデフォルト値として使われる。
    pub retryPointAreaRadius: f32,

    /// NAME: Special effect ID that activates when Nemuri items can be acquired - ネムリアイテム取得可能時に発動する特殊効果ID
    /// DESC: A special effect ID that activates when a Nemuri item can be acquired. TAE flag Used as the default value when not overwriting with "Event> Nemuri item registration". - ネムリアイテム取得可能時に発動する特殊効果ID。TAEフラグ「イベント＞ネムリアイテム登録」で上書きしないときのデフォルト値として使われる。
    pub sleepCollectorSpEffectId: i32,

    /// NAME: "Recover below HP maximum" Special effect completion notification Special effect ID - 「HP最大以下で回復」特殊効果完了通知特殊効果ID
    /// DESC: ID of the special effect that notifies that "Recovery below HP maximum" is completed. Mainly used for multi synchronization. - 「HP最大以下で回復」が完了したことを通知する特殊効果のID。主にマルチの同期用に使われる。
    pub recoverBelowMaxHpCompletionNoticeSpEffectId: i32,

    /// NAME: HP Est absorption production SFXID - HPエスト吸収演出SFXID
    /// DESC: Absorption effect SFXID when recovering the number of times the HP Est bottle has been used when defeating an intruder - 侵入者撃破時などにHPエスト瓶の使用回数を回復する際の吸収演出SFXID
    pub estusFlaskRecovery_AbsorptionProductionSfxId_byHp: i32,

    /// NAME: MP Est Absorption Production SFXID - MPエスト吸収演出SFXID
    /// DESC: Absorption effect SFXID when recovering the number of times the MP Est bottle has been used when defeating an intruder - 侵入者撃破時などにMPエスト瓶の使用回数を回復する際の吸収演出SFXID
    pub estusFlaskRecovery_AbsorptionProductionSfxId_byMp: i32,

    /// NAME: Special effect ID for determining the activation of the resurrection special effect - 復活特殊効果発動判定用特殊効果ID
    /// DESC: ID of the special effect that notifies that the resurrection special effect has been activated. Mainly used for multi synchronization. - 復活特殊効果が発動したことを通知する特殊効果のID。主にマルチの同期用に使われる。
    pub respawnSpecialEffectActiveCheckerSpEffectId: i32,

    /// NAME: Buddy Summoning_Initial Range Special Effect ID_For Buddy - バディ召喚中_起動範囲内特殊効果ID_バディ用
    /// DESC: Buddy Summoning_Initial Range Special Effect ID_For Buddy - バディ召喚中_起動範囲内特殊効果ID_バディ用
    pub onBuddySummon_inActivateRange_spEffectId_buddy: i32,

    /// NAME: Time from the start of est absorption SFX playback to the start of est addition processing - エスト吸収SFX再生開始からエスト追加処理を行うまでの時間
    /// DESC: Time from the start of est absorption SFX playback to the start of est addition processing - エスト吸収SFX再生開始からエスト追加処理を行うまでの時間
    pub estusFlaskRecovery_AddEstusTime: f32,

    /// NAME: Soul correction value acquired when defeating enemies at multi time_host - マルチ時エネミー撃破時取得ソウル補正値_ホスト
    /// DESC: Correction value of the amount of soul acquired by the host when defeating a normal enemy in multiplayer - マルチプレイで通常敵を撃破した時のホストの取得ソウル量の補正値
    pub defeatMultiModeEnemyOfSoulCorrectRate_byHost: f32,

    /// NAME: Soul correction value acquired when defeating enemies at multi time_cooperative spirit - マルチ時エネミー撃破時取得ソウル補正値_協力霊
    /// DESC: Correction value of the amount of soul acquired by cooperating spirits when defeating a normal enemy in multiplayer - マルチプレイで通常敵を撃破した時の協力霊の取得ソウル量の補正値
    pub defeatMultiModeEnemyOfSoulCorrectRate_byTeamGhost: f32,

    /// NAME: Soul correction value acquired when defeating the boss at the time of multi _ host - マルチ時ボス撃破時取得ソウル補正値_ホスト
    /// DESC: Correction value of the amount of soul acquired by the host when defeating the boss in multiplayer - マルチプレイでボスを撃破した時のホストの取得ソウル量の補正値
    pub defeatMultiModeBossOfSoulCorrectRate_byHost: f32,

    /// NAME: Soul correction value acquired when defeating the boss at the time of multi _ cooperation spirit - マルチ時ボス撃破時取得ソウル補正値_協力霊
    /// DESC: Correction value of the amount of soul acquired when the boss is defeated in multiplayer - マルチプレイでボスを撃破した時の協力霊の取得ソウル量の補正値
    pub defeatMultiModeBossOfSoulCorrectRate_byTeamGhost: f32,

    /// NAME: Offset to prevent the HP gauge of the enemy character from being cut off on the screen - 敵キャラのHPゲージが画面上に見切れないようにするためのオフセット
    /// DESC: Offset value [pixel] (FullHD standard) that fits in the screen when the enemy's HP gauge is cut off on the screen - 敵のHPゲージが画面上に見切れた時に画面内に収めるオフセット値[pixel]（FullHD基準）
    pub enemyHpGaugeScreenOffset_byUp: u16,

    /// NAME: Play area collection radius - プレイ領域収集半径
    /// DESC: Collection radius of the play area around the PC - PC周辺のプレイ領域の収集半径
    pub playRegionCollectDist: u16,

    /// NAME: Bullet firing position during "enemy detection" Damipoli ID - 「敵探知」時弾丸発射位置ダミポリID
    /// DESC: Detect bullet firing position Damipoli ID - 探知弾丸の発射位置ダミポリID
    pub enemyDetectionSpEffect_ShootBulletDummypolyId: u16,

    /// NAME: Large Rune: Number of tools granted when the Greater Demon invades - 大ルーン：グレーターデーモン侵入時付与道具個数
    /// DESC: Large Rune: Number of tools granted when the Greater Demon invades - 大ルーン：グレーターデーモン侵入時付与道具個数
    pub bigRuneGreaterDemonBreakInGoodsNum: u16,

    /// NAME: Large Rune: Greater Demon Invasion Tool Item ID - 大ルーン：グレーターデーモン侵入時付与道具アイテムID
    /// DESC: Large Rune: Greater Demon Invasion Tool Item ID - 大ルーン：グレーターデーモン侵入時付与道具アイテムID
    pub bigRuneGreaterDemonBreakInGoodsId: i32,

    /// NAME: Large jump area SFXID - 大ジャンプ領域SFXID
    /// DESC: SFX ID in the riding large jump area - 騎乗大ジャンプ領域のSFXID
    pub rideJumpRegionDefaultSfxId: i32,

    /// NAME: Common_Riding special attack magnification - 共通_騎乗特攻倍率
    /// DESC: Magnification to be corrected at the time of riding special attack - 騎乗特攻時に補正する倍率
    pub saAttackRate_forVsRideAtk: f32,

    /// NAME: Special effects on the enemy side when drawing Nemuri items - ネムリアイテム抽選時に敵側にかかる特殊効果
    /// DESC: Special effects on the enemy side when drawing Nemuri items - ネムリアイテム抽選時に敵側にかかる特殊効果
    pub enemySpEffectIdAfterSleepCollectorItemLot: i32,

    /// NAME: Map UID on hold - 周回保留時マップUID
    /// DESC: Map UID on hold for lap, enter in 8 digits (example ... m60_42_36_00-> 60423600) - 周回保留時マップUID、8桁で入力（例…m60_42_36_00 -> 60423600）
    pub afterEndingMapUid: i32,

    /// NAME: Return point when lap is on hold - 周回保留時復帰ポイント
    /// DESC: Entity ID of return point on hold - 周回保留時復帰ポイントのエンティティID
    pub afterEndingReturnPointEntityId: u32,

    /// NAME: Bullet ID_cooperation ring_red hunting fired during "enemy detection" - 「敵探知」時発射弾丸ID_協力指輪_赤狩り
    /// DESC: Bullet ID to fly depending on enemy power / type (also used for multi-automatic firing) - 敵の勢力/タイプによって飛ばす弾丸のID(マルチ自動発射でも使う)
    pub enemyDetectionSpEffect_BulletId_byCoopRing_RedHunter: i32,

    /// NAME: No bullet ID_invasion orb_ fired during "enemy detection" - 「敵探知」時発射弾丸ID_侵入オーブ_なし
    /// DESC: Bullet ID to fly depending on enemy power / type (also used for multi-automatic firing) - 敵の勢力/タイプによって飛ばす弾丸のID(マルチ自動発射でも使う)
    pub enemyDetectionSpEffect_BulletId_byInvadeOrb_None: i32,

    /// NAME: For tutorial judgment: Event flag to turn on when accessing the distant view - チュートリアル判定用：遠見台にアクセスした時にONにするイベントフラグ
    /// DESC: For tutorial judgment: Event flag to turn on when accessing the distant view - チュートリアル判定用：遠見台にアクセスした時にONにするイベントフラグ
    pub tutorialFlagOnAccessDistView: u32,

    /// NAME: For tutorial judgment: Event flag to turn on when accessing the retry point - チュートリアル判定用：リトライポイントにアクセスした時にONにするイベントフラグ
    /// DESC: For tutorial judgment: Event flag to turn on when accessing the retry point - チュートリアル判定用：リトライポイントにアクセスした時にONにするイベントフラグ
    pub tutorialFlagOnAccessRetryPoint: u32,

    /// NAME: For tutorial judgment: Event flag to turn on when group reward is entered after defeating a group - チュートリアル判定用：集団を倒してグループ報酬が入った時にONにするイベントフラグ
    /// DESC: For tutorial judgment: Event flag to turn on when group reward is entered after defeating a group - チュートリアル判定用：集団を倒してグループ報酬が入った時にONにするイベントフラグ
    pub tutorialFlagOnGetGroupReward: u32,

    /// NAME: For tutorial judgment: Event flag to turn on when entering the riding big jump point - チュートリアル判定用：騎乗大ジャンプポイントに入った時にONにするイベントフラグ
    /// DESC: For tutorial judgment: Event flag to turn on when entering the riding big jump point - チュートリアル判定用：騎乗大ジャンプポイントに入った時にONにするイベントフラグ
    pub tutorialFlagOnEnterRideJumpRegion: u32,

    /// NAME: For tutorial judgment: Expand the riding large jump point by ○ [m] to judge inside and outside - チュートリアル判定用：騎乗大ジャンプポイントを○[m]拡張して内外判定
    /// DESC: For tutorial judgment: The riding large jump point is expanded by ○ [m] to judge inside and outside. ○ Set the value of [m] here. - チュートリアル判定用：騎乗大ジャンプポイントを○[m]拡張して内外判定。○[m]の値をここに設定する。
    pub tutorialCheckRideJumpRegionExpandRange: f32,

    /// NAME: PC animation ID when retry point is activated - リトライポイント起動時のPCアニメID
    /// DESC: PC animation ID when retry point is started. If it is -1, it will not be played. - リトライポイント起動時のPCアニメID。-1の場合は再生しない。
    pub retryPointActivatedPcAnimId: i32,

    /// NAME: Delay time of dialog display when retry point is started [seconds] - リトライポイント起動時のダイアログ表示の遅延時間[秒]
    /// DESC: Delay time of dialog display when retry point is started [seconds] - リトライポイント起動時のダイアログ表示の遅延時間[秒]
    pub retryPointActivatedDialogDelayTime: f32,

    /// NAME: Text ID of the dialog when starting the retry point - リトライポイント起動時のダイアログのテキストID
    /// DESC: The text ID of the dialog when the retry point is started. Set the text in EventText_ForMap.xlsm. If it is -1, the dialog is not displayed. - リトライポイント起動時のダイアログのテキストID。EventText_ForMap.xlsm のテキストを設定する。-1の場合はダイアログを出さない。
    pub retryPointActivatedDialogTextId: i32,

    /// NAME: PC animation ID at the time of sign accumulation startup - サイン溜まり起動時のPCアニメID
    /// DESC: PC animation ID at the time of sign accumulation startup. If it is -1, it will not be played. - サイン溜まり起動時のPCアニメID。-1の場合は再生しない。
    pub signPuddleOpenPcAnimId: i32,

    /// NAME: Delay time of dialog display at startup of sign accumulation [seconds] - サイン溜まり起動時のダイアログ表示の遅延時間[秒]
    /// DESC: Delay time of dialog display at startup of sign accumulation [seconds] - サイン溜まり起動時のダイアログ表示の遅延時間[秒]
    pub signPuddleOpenDialogDelayTime: f32,

    /// NAME: Bullet ID when the "Activity of the Dead" special effect is activated - 「死者の活性」特殊効果発動時弾丸ID
    /// DESC: Bullet ID fired when the "Activity of the Dead" special effect is activated - 「死者の活性」特殊効果が発動したときに発射する弾丸ID
    pub activityOfDeadSpEffect_BulletId: i32,

    /// NAME: Bullet generation position when the "Activity of the Dead" special effect is activated Damipoli ID - 「死者の活性」特殊効果発動時弾丸発生位置ダミポリID
    /// DESC: Damipoli ID at the position where the bullet is generated when the "Activity of the Dead" special effect is activated - 「死者の活性」特殊効果が発動したときに弾丸が発生する位置のダミポリID
    pub activityOfDeadSpEffect_ShootBulletDummypolyId: i32,

    /// NAME: Fade out time of the corpse when the "Activity of the Dead" special effect is activated - 「死者の活性」特殊効果発動時の死体のフェードアウト時間
    /// DESC: Fade time when the corpse fades out when the "Activity of the Dead" special effect is activated - 「死者の活性」特殊効果が発動したときに死体がフェードアウトする際のフェード時間
    pub activityOfDeadSpEffect_DeadFadeOutTime: f32,

    /// NAME: Time to ignore transitions due to network information at the start of throwing - 投げ開始時のネットワーク情報による遷移を無視する時間
    /// DESC: Time to ignore transitions due to network information at the start of throwing - 投げ開始時のネットワーク情報による遷移を無視する時間
    pub ignorNetStateSyncTime_ForThrow: f32,

    /// NAME: Multi-pre-penalty: LAN disconnection - マルチプレペナルティ：LAN切断
    /// DESC: Multi-pre-penalty: LAN disconnection - マルチプレペナルティ：LAN切断
    pub netPenaltyPointLanDisconnect: u16,

    /// NAME: Multi-pre-penalty: Profile sign-out - マルチプレペナルティ：プロフィールサインアウト
    /// DESC: Multi-pre-penalty: Profile sign-out - マルチプレペナルティ：プロフィールサインアウト
    pub netPenaltyPointProfileSignout: u16,

    /// NAME: Multi-pre-penalty: Power off - マルチプレペナルティ：電源断
    /// DESC: Multi-pre-penalty: Power off - マルチプレペナルティ：電源断
    pub netPenaltyPointReboot: u16,

    /// NAME: Multi-pre-penalty: Suspend / Pause - マルチプレペナルティ：サスペンド・一時停止
    /// DESC: Multi-pre-penalty: Suspend / Pause - マルチプレペナルティ：サスペンド・一時停止
    pub netPnaltyPointSuspend: u16,

    /// NAME: Multi-pre-penalty: Waiting time to start the generation (sale) of the bone of reason - マルチプレペナルティ：理の骨の生成（販売）開始待ち時間
    /// DESC: Multi-pre-penalty: Waiting time (seconds) to start the generation (sale) of the bone of reason - マルチプレペナルティ：理の骨の生成（販売）開始待ち時間(秒)
    pub netPenaltyForgiveItemLimitTime: f32,

    /// NAME: Multi-pre-penalty: Penalty judgment points - マルチプレペナルティ：ペナルティ判定ポイント
    /// DESC: Multi-pre-penalty: Penalty judgment points - マルチプレペナルティ：ペナルティ判定ポイント
    pub netPenaltyPointThreshold: u16,

    /// NAME: No operation judgment time - 未操作判定時間
    /// DESC: It is for leaving people who have not operated for a certain period of time in multiplayer. The unit is seconds. - マルチで一定期間操作ない人を退出させるためのもの。単位は秒。
    pub uncontrolledMoveThresholdTime: u16,

    /// NAME: Bullet ID_hostile NPC / enemy character fired during "enemy detection" - 「敵探知」時発射弾丸ID_敵対NPC/敵キャラ
    /// DESC: Bullet ID to shoot to hostile NPCs / enemy characters when hostility detection fails - 敵意の探知に失敗したときに敵対NPC/敵キャラに飛ばす弾丸のID
    pub enemyDetectionSpEffect_BulletId_byNpcEnemy: i32,

    /// NAME: Special effect ID to be applied to the target of "Activity target search of the dead" - 「死者の活性ターゲット検索」対象にかける特殊効果ID
    /// DESC: Special effects applied to the searched target - 検索した対象にかける特殊効果
    pub activityOfDeadTargetSearchSpEffect_OnHitSpEffect: i32,

    /// NAME: "Search for active targets of the dead" distance - 「死者の活性ターゲット検索」距離
    /// DESC: Maximum searchable distance - 検索可能最大距離
    pub activityOfDeadTargetSearchSpEffect_MaxLength: f32,

    /// NAME: Field of view_Minimum guaranteed distance [magnification conversion] - 視界_最低保証距離[倍率換算]
    /// DESC: Field of view_Minimum guaranteed distance [magnification conversion] - 視界_最低保証距離[倍率換算]
    pub sightRangeLowerPromiseRate: f32,

    /// NAME: SA large damage hit production SFX_ Occurrence condition SA damage minimum required [pt] - SA大ダメージヒット演出SFX_発生条件SAダメージ必要最低値[pt]
    /// DESC: SA large damage hit production SFX_ Occurrence condition SA damage minimum required [pt] - SA大ダメージヒット演出SFX_発生条件SAダメージ必要最低値[pt]
    pub saLargeDamageHitSfx_MinDamage: i16,

    /// NAME: SA large damage hit production SFX_ Occurrence condition SA damage forced occurrence minimum value [pt] - SA大ダメージヒット演出SFX_発生条件SAダメージ強制発生最低値[pt]
    /// DESC: SA large damage hit production SFX_ Occurrence condition SA damage forced occurrence minimum value [pt] - SA大ダメージヒット演出SFX_発生条件SAダメージ強制発生最低値[pt]
    pub saLargeDamageHitSfx_ForceDamage: i16,

    /// NAME: Maximum solo intrusion point - ソロ侵入最大ポイント
    /// DESC: Maximum solo intrusion point. When this value is exceeded, it will be invaded solo. - ソロ侵入ポイントの最大値。この値を越えたときにソロで侵入されるようになる
    pub soloBreakInMaxPoint: u32,

    /// NAME: NPC conversation voice playback timeout time - NPC会話のボイス再生タイムアウト時間
    /// DESC: Voice playback timeout for NPC conversations. If voice playback does not end after this time, proceed to the next message. - NPC会話のボイス再生タイムアウト時間。この時間経過してもボイス再生が終わらない場合は次のメッセージへ進む
    pub npcTalkTimeOutThreshold: f32,

    /// NAME: Playlog transmission interval - プレイログの送信間隔
    /// DESC: Interval to send item usage log etc. to the server - アイテム使用ログ などをサーバーへ送信する間隔
    pub sendPlayLogIntervalTime: f32,

    /// NAME: Maximum number of seven-colored stones installed - 七色石の最大設置数
    /// DESC: Maximum number of seven-colored stones installed - 七色石の最大設置数
    pub item370_MaxSfxNum: u8,

    /// NAME: Distance allowed to activate during character deactivation [m] - キャラディアクティベート中にアクティベート許可する距離[m]
    /// DESC: Distance allowed to activate during character deactivation (valid only for open placement characters) - キャラディアクティベート中にアクティベート許可する距離（オープン配置キャラのみ有効）
    pub chrActivateDist_forLeavePC: u8,

    /// NAME: Multi-weakening level correction factor 1 - マルチ弱体化レベル補正係数１
    /// DESC: Weakened stats when multiplayer. Host level addition correction - マルチ時ステータス弱体化。ホストのレベル加算補正
    pub summonDataCoopMatchingLevelUpperAbs: i16,

    /// NAME: Multi-weakening level correction factor 2 - マルチ弱体化レベル補正係数２
    /// DESC: Weakened stats when multiplayer. Host level magnification correction - マルチ時ステータス弱体化。ホストのレベル倍率補正
    pub summonDataCoopMatchingLevelUpperRel: i16,

    /// NAME: Multi-weapon maximum weapon correction factor - マルチ弱体化最大武器補正係数
    /// DESC: Weakened stats when multiplayer. Maximum weapon enhancement level correction - マルチ時ステータス弱体化。最大武器強化レベル補正
    pub summonDataCoopMatchingWepLevelMul: i16,

    /// NAME: Bullet ID for the effect when picking up the Berserker sign - バーサーカーサインを拾った時のまたたび効果用弾丸ID
    /// DESC: Bullet ID when generating a bullet for special effects at the sign position - サイン位置に特殊効果用の弾丸を発生させる際の弾丸ID
    pub pickUpBerserkerSignSpEffectBulletId: i32,

    /// NAME: Berserker succeeds in killing PC by himself Special effect ID for directing - バーサーカーがPC自力殺害に成功演出用特殊効果ID
    /// DESC: Special effect to play a special effect when Berserker succeeds in killing the PC by himself - バーサーカーがPC自力殺害に成功した際に専用の演出を再生する特殊効果
    pub succeedBerserkerSelfKillingEffectId: i32,

    /// NAME: Level sync application coefficient of determination 1 white - レベルシンク適用判定係数１白
    /// DESC: Soul level coefficient of whether to apply level sync - レベルシンク適用するかどうかのソウルレベル係数
    pub machingLevelWhiteSignUpperRel: u8,

    /// NAME: Level sync application coefficient of determination 2 white - レベルシンク適用判定係数２白
    /// DESC: Soul level coefficient of whether to apply level sync - レベルシンク適用するかどうかのソウルレベル係数
    pub machingLevelWhiteSignUpperAbs: u8,

    /// NAME: Level sync application coefficient of determination 1 red - レベルシンク適用判定係数１赤
    /// DESC: Soul level coefficient of whether to apply level sync - レベルシンク適用するかどうかのソウルレベル係数
    pub machingLevelRedSignUpperRel: u8,

    /// NAME: Level sync application coefficient of determination 2 red - レベルシンク適用判定係数２赤
    /// DESC: Soul level coefficient of whether to apply level sync - レベルシンク適用するかどうかのソウルレベル係数
    pub machingLevelRedSignUpperAbs: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 0 White - レベルシンク適用判定最大武器強化レベル係数０白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_0: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 1 White - レベルシンク適用判定最大武器強化レベル係数１白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_1: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 2 White - レベルシンク適用判定最大武器強化レベル係数２白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_2: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 3 White - レベルシンク適用判定最大武器強化レベル係数３白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_3: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 4 White - レベルシンク適用判定最大武器強化レベル係数４白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_4: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 5 White - レベルシンク適用判定最大武器強化レベル係数５白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_5: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 6 White - レベルシンク適用判定最大武器強化レベル係数６白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_6: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 7 White - レベルシンク適用判定最大武器強化レベル係数７白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_7: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 8 White - レベルシンク適用判定最大武器強化レベル係数８白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_8: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 9 White - レベルシンク適用判定最大武器強化レベル係数９白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_9: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 10 White - レベルシンク適用判定最大武器強化レベル係数１０白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_10: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 0 Red - レベルシンク適用判定最大武器強化レベル係数０赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_0: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 1 Red - レベルシンク適用判定最大武器強化レベル係数１赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_1: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 2 Red - レベルシンク適用判定最大武器強化レベル係数２赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_2: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 3 Red - レベルシンク適用判定最大武器強化レベル係数３赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_3: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 4 Red - レベルシンク適用判定最大武器強化レベル係数４赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_4: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 5 Red - レベルシンク適用判定最大武器強化レベル係数５赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_5: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 6 Red - レベルシンク適用判定最大武器強化レベル係数６赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_6: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 7 Red - レベルシンク適用判定最大武器強化レベル係数７赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_7: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 8 Red - レベルシンク適用判定最大武器強化レベル係数８赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_8: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 9 Red - レベルシンク適用判定最大武器強化レベル係数９赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_9: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 10 Red - レベルシンク適用判定最大武器強化レベル係数１０赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_10: u8,

    /// NAME: Automatic placement interval of intrusion points - 侵入ポイントの自動配置間隔
    /// DESC: Automatic placement interval of intrusion points - 侵入ポイントの自動配置間隔
    pub autoInvadePoint_generateDist: u8,

    /// NAME: Intrusion point automatic placement cancellation range - 侵入ポイント自動配置取り消し範囲
    /// DESC: Intrusion point automatic placement cancellation range - 侵入ポイント自動配置取り消し範囲
    pub autoInvadePoint_cancelDist: u8,

    /// NAME: Global event log transmission interval - グローバルイベントログの送信間隔
    /// DESC: Interval to send global event log to server - グローバルイベントログ をサーバーへ送信する間隔
    pub sendGlobalEventLogIntervalTime: f32,

    /// NAME: Solo intrusion point addition value_white sign - ソロ侵入ポイント加算値_白サイン
    /// DESC: Solo intrusion point addition value_white sign - ソロ侵入ポイント加算値_白サイン
    pub addSoloBreakInPoint_White: u16,

    /// NAME: Solo intrusion point addition value_red sign - ソロ侵入ポイント加算値_赤サイン
    /// DESC: Solo intrusion point addition value_red sign - ソロ侵入ポイント加算値_赤サイン
    pub addSoloBreakInPoint_Black: u16,

    /// NAME: Solo intrusion point addition value_intrusion - ソロ侵入ポイント加算値_乱入
    /// DESC: Solo intrusion point addition value_intrusion - ソロ侵入ポイント加算値_乱入
    pub addSoloBreakInPoint_ForceJoin: u16,

    /// NAME: Solo intrusion point addition value_Map guardian visit - ソロ侵入ポイント加算値_マップ守護訪問
    /// DESC: Solo intrusion point addition value_Map guardian visit - ソロ侵入ポイント加算値_マップ守護訪問
    pub addSoloBreakInPoint_VisitorGuardian: u16,

    /// NAME: Solo intrusion point addition value_Red Scare visit - ソロ侵入ポイント加算値_赤狩り訪問
    /// DESC: Solo intrusion point addition value_Red Scare visit - ソロ侵入ポイント加算値_赤狩り訪問
    pub addSoloBreakInPoint_VisitorRedHunter: u16,

    /// NAME: Invincible timer for initial sync PC - 初期同期PC用の無敵タイマー
    /// DESC: Invincible timer for initial sync PC - 初期同期PC用の無敵タイマー
    pub invincibleTimer_forNetPC_initSync: u8,

    /// NAME: Invincible timer for non-initial sync PCs - 初期同期PC以外用の無敵タイマー
    /// DESC: Invincible timer for non-initial sync PCs - 初期同期PC以外用の無敵タイマー
    pub invincibleTimer_forNetPC: u8,

    /// NAME: [Red Scare] Soul rate received when the host passes through the white door - 【赤狩り】ホストが白扉を通過した際にもらえるソウル率
    /// DESC: Soul that you can get red hunting when the host passes through the white door = Soul required for red hunting to change from the previous Lv to the current Lv * This magnification - ホストが白扉を通過した時に赤狩りがもらえるソウル=赤狩りが一つ前のLvから現在Lvになるために必要なソウル*この倍率
    pub redHunter_HostBossAreaGetSoulRate: f32,

    /// NAME: Decal parameter ID of the trace of wandering illusion - 徘徊幻影の痕跡のデカールパラメータID
    /// DESC: Decal parameter ID of the trace that the wandering illusion puts out while moving - 徘徊幻影が移動中に出す痕跡のデカールパラメータID
    pub ghostFootprintDecalParamId: i32,

    /// NAME: Counting time for warning message display outside the multiplayer limit [seconds] - マルチプレイ制限距離外の警告メッセージ表示のカウント時間[秒]
    /// DESC: If this count time elapses while you are out of the multiplayer limit distance, the multiplayer will be disbanded. - マルチプレイ制限距離外に出たままこのカウント時間経過したらマルチプレイの解散を行う
    pub leaveAroundHostWarningTime: f32,

    /// NAME: Hosting cost Item ID - ホスト化コストアイテムID
    /// DESC: ID of the cost item consumed when hosting is turned on - ホスト化をONにした際に消費するコストアイテムのID
    pub hostModeCostItemId: i32,

    /// NAME: AI jump deceleration parameter - AIジャンプ減速パラメータ
    /// DESC: Deceleration parameters for AI jump (0.0: constant velocity motion, 1.0: maximum deceleration, speed 0 at target point) - AIジャンプ用減速パラメータ(0.0：等速運動、1.0：最大減速、目標地点で速度0)
    pub aIJump_DecelerateParam: f32,

    /// NAME: Buddy Instance Deletion Guarantee Time - バディインスタンス削除保証時間
    /// DESC: The time from the death flag to the actual disappearance of the instance - 死亡フラグから実際にインスタンスが消滅するまでの時間
    pub buddyDisappearDelaySec: f32,

    /// NAME: AI jump jumping Y movement amount correction factor - AIジャンプ飛び降り時Y移動量補正率
    /// DESC: AI jump jumping Y movement amount correction factor - AIジャンプ飛び降り時Y移動量補正率
    pub aIJump_AnimYMoveCorrectRate_onJumpOff: f32,

    /// NAME: Stealth visibility magnification_Crouching without stealth effect - ステルス視界倍率_ステルス効果無しでしゃがみ
    /// DESC: Stealth visibility magnification_Crouching without stealth effect - ステルス視界倍率_ステルス効果無しでしゃがみ
    pub stealthSystemSightRate_NotInStealthRigid_NotSightHide_StealthMode: f32,

    /// NAME: Stealth visibility magnification_Standing in stealth ray shielded area - ステルス視界倍率_ステルスレイ遮蔽地帯で立ち
    /// DESC: Stealth visibility magnification_Standing in stealth ray shielded area - ステルス視界倍率_ステルスレイ遮蔽地帯で立ち
    pub stealthSystemSightRate_NotInStealthRigid_SightHide_NotStealthMode: f32,

    /// NAME: Stealth visibility magnification_Crouching in stealth ray shielded area - ステルス視界倍率_ステルスレイ遮蔽地帯でしゃがみ
    /// DESC: Stealth visibility magnification_Crouching in stealth ray shielded area - ステルス視界倍率_ステルスレイ遮蔽地帯でしゃがみ
    pub stealthSystemSightRate_NotInStealthRigid_SightHide_StealthMode: f32,

    /// NAME: Stealth visibility magnification_Standing within a stealth hit - ステルス視界倍率_ステルスヒット内で立ち
    /// DESC: Stealth visibility magnification_Standing within a stealth hit - ステルス視界倍率_ステルスヒット内で立ち
    pub stealthSystemSightRate_InStealthRigid_NotSightHide_NotStealthMode: f32,

    /// NAME: Stealth visibility magnification_Crouching within a stealth hit - ステルス視界倍率_ステルスヒット内でしゃがみ
    /// DESC: Stealth visibility magnification_Crouching within a stealth hit - ステルス視界倍率_ステルスヒット内でしゃがみ
    pub stealthSystemSightRate_InStealthRigid_NotSightHide_StealthMode: f32,

    /// NAME: Stealth visibility magnification_Stealth ray shielded area + standing in stealth hit - ステルス視界倍率_ステルスレイ遮蔽地帯＋ステルスヒット内で立ち
    /// DESC: Stealth visibility magnification_Stealth ray shielded area + standing in stealth hit - ステルス視界倍率_ステルスレイ遮蔽地帯＋ステルスヒット内で立ち
    pub stealthSystemSightRate_InStealthRigid_SightHide_NotStealthMode: f32,

    /// NAME: Stealth visibility magnification_Stealth ray shielded area + crouching in stealth hit - ステルス視界倍率_ステルスレイ遮蔽地帯＋ステルスヒット内でしゃがみ
    /// DESC: Stealth visibility magnification_Stealth ray shielded area + crouching in stealth hit - ステルス視界倍率_ステルスレイ遮蔽地帯＋ステルスヒット内でしゃがみ
    pub stealthSystemSightRate_InStealthRigid_SightHide_StealthMode: f32,

    /// NAME: Treasure corpse default action button parameter ID - 宝死体のデフォルトアクションボタンパラメータID
    /// DESC: Default action button parameter ID when "Treasure corpse" is selected in the treasure box type of treasure box information for OBJ of MapStudio event - MapStudioイベントのOBJ用宝箱情報の宝箱タイプで「宝死体」を選択したときのデフォルトアクションボタンパラメータID
    pub msbEventGeomTreasureInfo_actionButtonParamId_corpse: i32,

    /// NAME: Anime ID when acquiring the default item of the treasure corpse - 宝死体のデフォルトアイテム取得時アニメID
    /// DESC: When "Treasure corpse" is selected in the treasure box type of the treasure box information for OBJ of the MapStudio event, the default item is acquired. Animation ID - MapStudioイベントのOBJ用宝箱情報の宝箱タイプで「宝死体」を選択したときのデフォルトアイテム取得時アニメID
    pub msbEventGeomTreasureInfo_itemGetAnimId_corpse: i32,

    /// NAME: Treasure chest default action button parameter ID - 宝箱のデフォルトアクションボタンパラメータID
    /// DESC: Default action button parameter ID when "Treasure chest" is selected in the treasure chest type of treasure chest information for OBJ of MapStudio event - MapStudioイベントのOBJ用宝箱情報の宝箱タイプで「宝箱」を選択したときのデフォルトアクションボタンパラメータID
    pub msbEventGeomTreasureInfo_actionButtonParamId_box: i32,

    /// NAME: Anime ID when acquiring the default item of the treasure box - 宝箱のデフォルトアイテム取得時アニメID
    /// DESC: When "Treasure chest" is selected in the treasure chest type of the treasure chest information for OBJ of the MapStudio event, the default item acquisition animation ID - MapStudioイベントのOBJ用宝箱情報の宝箱タイプで「宝箱」を選択したときのデフォルトアイテム取得時アニメID
    pub msbEventGeomTreasureInfo_itemGetAnimId_box: i32,

    /// NAME: Item light default action button parameter ID - アイテム光のデフォルトアクションボタンパラメータID
    /// DESC: Default action button parameter ID when "Item light" is selected in the treasure chest type of treasure chest information for OBJ of MapStudio event - MapStudioイベントのOBJ用宝箱情報の宝箱タイプで「アイテム光」を選択したときのデフォルトアクションボタンパラメータID
    pub msbEventGeomTreasureInfo_actionButtonParamId_shine: i32,

    /// NAME: Item Hikari's default item acquisition animation ID - アイテム光のデフォルトアイテム取得時アニメID
    /// DESC: When "Item Hikari" is selected as the treasure chest type of the treasure chest information for OBJ of the MapStudio event, the default item acquisition animation ID - MapStudioイベントのOBJ用宝箱情報の宝箱タイプで「アイテム光」を選択したときのデフォルトアイテム取得時アニメID
    pub msbEventGeomTreasureInfo_itemGetAnimId_shine: i32,

    /// NAME: Sign pool: Asset ID - サイン溜まり：アセットID
    /// DESC: Assets used for sign collection - サイン溜まりに使うアセット
    pub signPuddleAssetId: i32,

    /// NAME: Sign accumulation: Sign appearance Damipoli 0 - サイン溜まり：サイン出現ダミポリ0
    /// DESC: Damipoli used to determine the display position of the sign in the sign pool - サイン溜まりのサインの表示位置を決定するのに使うダミポリ
    pub signPuddleAppearDmypolyId0: i32,

    /// NAME: Sign accumulation: Sign appearance Damipoli 1 - サイン溜まり：サイン出現ダミポリ1
    /// DESC: Damipoli used to determine the display position of the sign in the sign pool - サイン溜まりのサインの表示位置を決定するのに使うダミポリ
    pub signPuddleAppearDmypolyId1: i32,

    /// NAME: Sign accumulation: Sign appearance Damipoli 2 - サイン溜まり：サイン出現ダミポリ2
    /// DESC: Damipoli used to determine the display position of the sign in the sign pool - サイン溜まりのサインの表示位置を決定するのに使うダミポリ
    pub signPuddleAppearDmypolyId2: i32,

    /// NAME: Sign accumulation: Sign appearance Damipoli 3 - サイン溜まり：サイン出現ダミポリ3
    /// DESC: Damipoli used to determine the display position of the sign in the sign pool - サイン溜まりのサインの表示位置を決定するのに使うダミポリ
    pub signPuddleAppearDmypolyId3: i32,

    /// NAME: Rider's fall damage multiplier correction_for PC - 騎乗者の落下ダメージ倍率補正_PC用
    /// DESC: Rider's fall damage multiplier correction_for PC - 騎乗者の落下ダメージ倍率補正_PC用
    pub fallDamageRate_forRidePC: f32,

    /// NAME: Rider's Fall Damage Multiplier Correction_for NPCs - 騎乗者の落下ダメージ倍率補正_NPC用
    /// DESC: Rider's Fall Damage Multiplier Correction_for NPCs - 騎乗者の落下ダメージ倍率補正_NPC用
    pub fallDamageRate_forRideNPC: f32,

    /// NAME: Special effect ID when creating the old man sign of yellow robe - 黄衣の翁サイン作成時特殊効果ID
    /// DESC: Special effect ID when creating the old man sign of yellow robe - 黄衣の翁サイン作成時特殊効果ID
    pub OldMonkOfYellow_CreateSignSpEffectId: i32,

    /// NAME: Defeated soldier activation distance - 敗残兵起動距離
    /// DESC: Defeated soldier activation distance - 敗残兵起動距離
    pub StragglerActivateDist: f32,

    /// NAME: Permission to use defeated soldier items_Special effects for PC - 敗残兵アイテム使用許可_PC用特殊効果
    /// DESC: Permission to use defeated soldier items_Special effects for PC - 敗残兵アイテム使用許可_PC用特殊効果
    pub SpEffectId_EnableUseItem_StragglerActivate: i32,

    /// NAME: Activate defeated soldiers_Special effects for defeated soldier characters - 敗残兵起動_敗残兵キャラ用特殊効果
    /// DESC: Activate defeated soldiers_Special effects for defeated soldier characters - 敗残兵起動_敗残兵キャラ用特殊効果
    pub SpEffectId_StragglerWakeUp: i32,

    /// NAME: Defeated soldiers_Special effects for subjugation targets - 敗残兵_討伐対象用特殊効果
    /// DESC: Defeated soldiers_Special effects for subjugation targets - 敗残兵_討伐対象用特殊効果
    pub SpEffectId_StragglerTarget: i32,

    /// NAME: Defeated soldiers_Special effects after hostility - 敗残兵_敵対後特殊効果
    /// DESC: Defeated soldiers_Special effects after hostility - 敗残兵_敵対後特殊効果
    pub SpEffectId_StragglerOppose: i32,

    /// NAME: Time for a buddy to warp a player with a ray block [s] - レイ遮断でバディがプレイヤーにワープする時間[s]
    /// DESC: Time for a buddy to warp a player with a ray block [s] - レイ遮断でバディがプレイヤーにワープする時間[s]
    pub buddyWarp_TriggerTimeRayBlocked: f32,

    /// NAME: Distance at which the buddy warps the player at a straight line distance [m] - 直線距離でバディがプレイヤーにワープする距離[m]
    /// DESC: Distance at which the buddy warps the player at a straight line distance [m] - 直線距離でバディがプレイヤーにワープする距離[m]
    pub buddyWarp_TriggerDistToPlayer: f32,

    /// NAME: Time to judge that the buddy is stuck in the pass movement [s] - バディがパス移動で詰まった判定する時間[s]
    /// DESC: Judgment time when the buddy is stuck in the pass movement [s] - バディがパス移動で詰まった判定する時間[s]
    pub buddyWarp_ThresholdTimePathStacked: f32,

    /// NAME: Distance [m] that the buddy considers to be clogged with a pass move - バディがパス移動で詰まっているとみなす距離[m]
    /// DESC: Distance [m] that the buddy considers to be clogged with a pass move - バディがパス移動で詰まっているとみなす距離[m]
    pub buddyWarp_ThresholdRangePathStacked: f32,

    /// NAME: [Morning] AI visibility magnification - [朝]AI視界倍率
    /// DESC: [Morning] AI visibility magnification - [朝]AI視界倍率
    pub aiSightRate_morning: f32,

    /// NAME: [Day] AI visibility magnification - [昼]AI視界倍率
    /// DESC: [Day] AI visibility magnification - [昼]AI視界倍率
    pub aiSightRate_noonA: f32,

    /// NAME: Time when the buddy and the player collide and start slipping through [s] - バディとプレイヤーがぶつかって、すり抜け始める時間[s]
    /// DESC: Time when the buddy and the player collide and start slipping through [s] - バディとプレイヤーがぶつかって、すり抜け始める時間[s]
    pub buddyPassThroughTriggerTime: f32,

    /// NAME: [Evening] AI visibility magnification - [夕]AI視界倍率
    /// DESC: [Evening] AI visibility magnification - [夕]AI視界倍率
    pub aiSightRate_evening: f32,

    /// NAME: [Night] AI visibility magnification - [夜]AI視界倍率
    /// DESC: [Night] AI visibility magnification - [夜]AI視界倍率
    pub aiSightRate_night: f32,

    /// NAME: [Midnight] AI visibility magnification - [深夜]AI視界倍率
    /// DESC: [Midnight] AI visibility magnification - [深夜]AI視界倍率
    pub aiSightRate_midnightA: f32,

    /// NAME: Reserve - リザーブ
    /// DESC: (dummy8) - (dummy8)
    pub reserve4_2: [u8; 4],

    /// NAME: AI visibility magnification_place where the sun cannot be seen (bright) - AI視界倍率_太陽が見えない場所(明るい)
    /// DESC: AI visibility magnification_place where the sun cannot be seen (bright) - AI視界倍率_太陽が見えない場所(明るい)
    pub aiSightRate_sunloss_light: f32,

    /// NAME: AI visibility magnification_place where the sun cannot be seen (darkness) - AI視界倍率_太陽が見えない場所(暗闇)
    /// DESC: AI visibility magnification_place where the sun cannot be seen (darkness) - AI視界倍率_太陽が見えない場所(暗闇)
    pub aiSightRate_sunloss_dark: f32,

    /// NAME: AI visibility magnification_place where the sun cannot be seen (total darkness) - AI視界倍率_太陽が見えない場所(真っ暗闇)
    /// DESC: AI visibility magnification_place where the sun cannot be seen (total darkness) - AI視界倍率_太陽が見えない場所(真っ暗闇)
    pub aiSightRate_sunloss_veryDark: f32,

    /// NAME: Stealth view angle attenuation factor_ crouching without stealth effect - ステルス視界角度減衰率_ステルス効果無しでしゃがみ
    /// DESC: Stealth view angle attenuation factor_ crouching without stealth effect - ステルス視界角度減衰率_ステルス効果無しでしゃがみ
    pub stealthSystemSightAngleReduceRate_NotInStealthRigid_NotSightHide_StealthMode: f32,

    /// NAME: Stealth view angle attenuation rate_Standing in stealth ray shielded area - ステルス視界角度減衰率_ステルスレイ遮蔽地帯で立ち
    /// DESC: Stealth view angle attenuation rate_Standing in stealth ray shielded area - ステルス視界角度減衰率_ステルスレイ遮蔽地帯で立ち
    pub stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_NotStealthMode: f32,

    /// NAME: Stealth visibility angle attenuation _ crouching in stealth ray shielded area - ステルス視界角度減衰率_ステルスレイ遮蔽地帯でしゃがみ
    /// DESC: Stealth visibility angle attenuation _ crouching in stealth ray shielded area - ステルス視界角度減衰率_ステルスレイ遮蔽地帯でしゃがみ
    pub stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_StealthMode: f32,

    /// NAME: Stealth view angle attenuation rate_Standing within stealth hit - ステルス視界角度減衰率_ステルスヒット内で立ち
    /// DESC: Stealth view angle attenuation rate_Standing within stealth hit - ステルス視界角度減衰率_ステルスヒット内で立ち
    pub stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_NotStealthMode: f32,

    /// NAME: Stealth field of view angle attenuation rate_ crouching within stealth hit - ステルス視界角度減衰率_ステルスヒット内でしゃがみ
    /// DESC: Stealth view angle attenuation rate_ crouching within stealth hit - ステルス視界角度減衰率_ステルスヒット内でしゃがみ
    pub stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_StealthMode: f32,

    /// NAME: Stealth view angle attenuation rate_Stealth ray shielded area + standing in stealth hit - ステルス視界角度減衰率_ステルスレイ遮蔽地帯＋ステルスヒット内で立ち
    /// DESC: Stealth view angle attenuation rate_Stealth ray shielded area + standing in stealth hit - ステルス視界角度減衰率_ステルスレイ遮蔽地帯＋ステルスヒット内で立ち
    pub stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_NotStealthMode: f32,

    /// NAME: Stealth view angle attenuation rate_Stealth ray shielded area + crouching in stealth hit - ステルス視界角度減衰率_ステルスレイ遮蔽地帯＋ステルスヒット内でしゃがみ
    /// DESC: Stealth view angle attenuation rate_Stealth ray shielded area + crouching in stealth hit - ステルス視界角度減衰率_ステルスレイ遮蔽地帯＋ステルスヒット内でしゃがみ
    pub stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_StealthMode: f32,

    /// NAME: Weather lottery conditions_morning_start time_hour - 天候抽選条件_朝_開始時刻_時
    /// DESC: Weather lottery conditions_morning_start time_hour (SEQ09371) - 天候抽選条件_朝_開始時刻_時(SEQ09371)
    pub weatherLotConditionStart_Morning_Hour: u8,

    /// NAME: Weather lottery conditions_morning_start time_minutes - 天候抽選条件_朝_開始時刻_分
    /// DESC: Weather lottery conditions_morning_start time_minutes (SEQ09371) - 天候抽選条件_朝_開始時刻_分(SEQ09371)
    pub weatherLotConditionStart_Morning_Minute: u8,

    /// NAME: Weather lottery conditions_day_start time_hour - 天候抽選条件_昼_開始時刻_時
    /// DESC: Weather lottery conditions_day_start time_hour (SEQ09371) - 天候抽選条件_昼_開始時刻_時(SEQ09371)
    pub weatherLotConditionStart_Day_Hour: u8,

    /// NAME: Weather lottery conditions_day_start time_minutes - 天候抽選条件_昼_開始時刻_分
    /// DESC: Weather lottery conditions_day_start time_minutes (SEQ09371) - 天候抽選条件_昼_開始時刻_分(SEQ09371)
    pub weatherLotConditionStart_Day_Minute: u8,

    /// NAME: Weather lottery conditions_evening_start time_hour - 天候抽選条件_夕_開始時刻_時
    /// DESC: Weather lottery conditions_evening_start time_hour (SEQ09371) - 天候抽選条件_夕_開始時刻_時(SEQ09371)
    pub weatherLotConditionStart_Evening_Hour: u8,

    /// NAME: Weather lottery conditions_evening_start time_minutes - 天候抽選条件_夕_開始時刻_分
    /// DESC: Weather lottery conditions_evening_start time_minutes (SEQ09371) - 天候抽選条件_夕_開始時刻_分(SEQ09371)
    pub weatherLotConditionStart_Evening_Minute: u8,

    /// NAME: Weather lottery conditions_night_start time_hour - 天候抽選条件_夜_開始時刻_時
    /// DESC: Weather lottery conditions_night_start time_hour (SEQ09371) - 天候抽選条件_夜_開始時刻_時(SEQ09371)
    pub weatherLotConditionStart_Night_Hour: u8,

    /// NAME: Weather lottery conditions_night_start time_minutes - 天候抽選条件_夜_開始時刻_分
    /// DESC: Weather lottery conditions_night_start time_minutes (SEQ09371) - 天候抽選条件_夜_開始時刻_分(SEQ09371)
    pub weatherLotConditionStart_Night_Minute: u8,

    /// NAME: Weather lottery conditions_dawn_start time_hour - 天候抽選条件_夜明け_開始時刻_時
    /// DESC: Weather lottery conditions_dawn_start time_hour (SEQ09371) - 天候抽選条件_夜明け_開始時刻_時(SEQ09371)
    pub weatherLotConditionStart_DayBreak_Hour: u8,

    /// NAME: Weather lottery conditions_dawn_start time_minutes - 天候抽選条件_夜明け_開始時刻_分
    /// DESC: Weather lottery conditions_dawn_start time_minutes (SEQ09371) - 天候抽選条件_夜明け_開始時刻_分(SEQ09371)
    pub weatherLotConditionStart_DayBreak_Minute: u8,

    /// NAME: Weather lottery conditions_reservation - 天候抽選条件_予約
    /// DESC: (dummy8) - (dummy8)
    pub weatherLotCondition_reserved: [u8; 2],

    /// NAME: Player Light intensity scale change time zone_start time_hour - Playerライト強度スケール変更時間帯_開始時刻_時
    /// DESC: Player Light Intensity Scale Change Time Zone_Start Time_Hour (SEQ16562) - Playerライト強度スケール変更時間帯_開始時刻_時(SEQ16562)
    pub pclightScaleChangeStart_Hour: u8,

    /// NAME: Player Light intensity scale change time zone_start time_minute - Playerライト強度スケール変更時間帯_開始時刻_分
    /// DESC: Player Light Intensity Scale Change Time Zone_Start Time_Minute (SEQ16562) - Playerライト強度スケール変更時間帯_開始時刻_分(SEQ16562)
    pub pclightScaleChangeStart_Minute: u8,

    /// NAME: Player Light intensity scale change time zone_end time_hour - Playerライト強度スケール変更時間帯_終了時刻_時
    /// DESC: Player Light intensity scale change time zone_end time_hour (SEQ16562) - Playerライト強度スケール変更時間帯_終了時刻_時(SEQ16562)
    pub pclightScaleChangeEnd_Hour: u8,

    /// NAME: Player Light intensity scale change time zone_end time_minute - Playerライト強度スケール変更時間帯_終了時刻_分
    /// DESC: Player Light Intensity Scale Change Time Zone_End Time_Minute (SEQ16562) - Playerライト強度スケール変更時間帯_終了時刻_分(SEQ16562)
    pub pclightScaleChangeEnd_Minute: u8,

    /// NAME: Time zone Player light intensity scale change value - 時間帯Playerライト強度スケール変更値
    /// DESC: Time zone Player light intensity scale change value (SEQ16562) - 時間帯Playerライト強度スケール変更値(SEQ16562)
    pub pclightScaleByTimezone: f32,

    /// NAME: Large Rune: Greater Demon Buddy Summon Buddy Special Effect ID - 大ルーン：グレーターデーモンバディ召喚時バディ付与特殊効果ID
    /// DESC: Large Rune: Greater Demon Buddy Summon Buddy Special Effect ID - 大ルーン：グレーターデーモンバディ召喚時バディ付与特殊効果ID
    pub bigRuneGreaterDemon_SummonBuddySpecialEffectId_Buddy: i32,

    /// NAME: Large Rune: Special effect ID given to PC when summoning Greater Demon Buddy - 大ルーン：グレーターデーモンバディ召喚時PC付与特殊効果ID
    /// DESC: Large Rune: Special effect ID given to PC when summoning Greater Demon Buddy - 大ルーン：グレーターデーモンバディ召喚時PC付与特殊効果ID
    pub bigRuneGreaterDemon_SummonBuddySpecialEffectId_Pc: i32,

    /// NAME: Base Bonfire Warp ID - 拠点篝火ワープID
    /// DESC: Base bonfire bonfire warp parameter ID - 拠点篝火の篝火ワープパラメータID
    pub homeBonfireParamId: i32,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 11 White - レベルシンク適用判定最大武器強化レベル係数１１白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_11: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 12 White - レベルシンク適用判定最大武器強化レベル係数１２白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_12: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 13 White - レベルシンク適用判定最大武器強化レベル係数１３白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_13: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 14 White - レベルシンク適用判定最大武器強化レベル係数１４白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_14: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 15 White - レベルシンク適用判定最大武器強化レベル係数１５白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_15: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 16 White - レベルシンク適用判定最大武器強化レベル係数１６白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_16: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 17 White - レベルシンク適用判定最大武器強化レベル係数１７白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_17: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 18 White - レベルシンク適用判定最大武器強化レベル係数１８白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_18: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 19 White - レベルシンク適用判定最大武器強化レベル係数１９白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_19: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 20 White - レベルシンク適用判定最大武器強化レベル係数２０白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_20: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 21 White - レベルシンク適用判定最大武器強化レベル係数２１白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_21: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 22 White - レベルシンク適用判定最大武器強化レベル係数２２白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_22: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 23 White - レベルシンク適用判定最大武器強化レベル係数２３白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_23: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 24 White - レベルシンク適用判定最大武器強化レベル係数２４白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_24: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 25 White - レベルシンク適用判定最大武器強化レベル係数２５白
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperWhiteSign_25: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 11 Red - レベルシンク適用判定最大武器強化レベル係数１１赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_11: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 12 Red - レベルシンク適用判定最大武器強化レベル係数１２赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_12: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 13 Red - レベルシンク適用判定最大武器強化レベル係数１３赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_13: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 14 Red - レベルシンク適用判定最大武器強化レベル係数１４赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_14: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 15 Red - レベルシンク適用判定最大武器強化レベル係数１５赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_15: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 16 Red - レベルシンク適用判定最大武器強化レベル係数１６赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_16: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 17 Red - レベルシンク適用判定最大武器強化レベル係数１７赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_17: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 18 Red - レベルシンク適用判定最大武器強化レベル係数１８赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_18: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 19 Red - レベルシンク適用判定最大武器強化レベル係数１９赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_19: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 20 Red - レベルシンク適用判定最大武器強化レベル係数２０赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_20: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 21 Red - レベルシンク適用判定最大武器強化レベル係数２１赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_21: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 22 Red - レベルシンク適用判定最大武器強化レベル係数２２赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_22: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 23 Red - レベルシンク適用判定最大武器強化レベル係数２３赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_23: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 24 Red - レベルシンク適用判定最大武器強化レベル係数２４赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_24: u8,

    /// NAME: Level Sync Applicability Judgment Maximum Weapon Enhancement Level Coefficient 25 Red - レベルシンク適用判定最大武器強化レベル係数２５赤
    /// DESC: Maximum weapon enhancement level factor for whether to apply level sync - レベルシンク適用するかどうかの最大武器強化レベル係数
    pub machingWeaponLevelUpperRedSign_25: u8,

    /// NAME: Menu time zone display_morning_start time_hour - メニュー用時間帯表示_朝_開始時刻_時
    /// DESC: Menu time zone display_morning_start time_hour (SEQ22108) - メニュー用時間帯表示_朝_開始時刻_時(SEQ22108)
    pub menuTimezoneStart_Morning_Hour: u8,

    /// NAME: Menu time zone display_morning_start time_minute - メニュー用時間帯表示_朝_開始時刻_分
    /// DESC: Menu time zone display_morning_start time_minute (SEQ22108) - メニュー用時間帯表示_朝_開始時刻_分(SEQ22108)
    pub menuTimezoneStart_Morning_Minute: u8,

    /// NAME: Menu time zone display_day 1_start time_hour - メニュー用時間帯表示_昼1_開始時刻_時
    /// DESC: Menu time zone display_day 1_start time_hour (SEQ22108) - メニュー用時間帯表示_昼1_開始時刻_時(SEQ22108)
    pub menuTimezoneStart_Day1_Hour: u8,

    /// NAME: Time zone display for menu_Lunch 1_Start time_Minute - メニュー用時間帯表示_昼1_開始時刻_分
    /// DESC: Menu time zone display_day 1_start time_minute (SEQ22108) - メニュー用時間帯表示_昼1_開始時刻_分(SEQ22108)
    pub menuTimezoneStart_Day1_Minute: u8,

    /// NAME: Menu time zone display_day 2_start time_hour - メニュー用時間帯表示_昼2_開始時刻_時
    /// DESC: Menu time zone display_day 2_start time_hour (SEQ22108) - メニュー用時間帯表示_昼2_開始時刻_時(SEQ22108)
    pub menuTimezoneStart_Day2_Hour: u8,

    /// NAME: Time zone display for menu_Lunch 2_Start time_Minute - メニュー用時間帯表示_昼2_開始時刻_分
    /// DESC: Time zone display for menu_Lunch 2_Start time_Minute (SEQ22108) - メニュー用時間帯表示_昼2_開始時刻_分(SEQ22108)
    pub menuTimezoneStart_Day2_Minute: u8,

    /// NAME: Menu time zone display_evening_start time_hour - メニュー用時間帯表示_夕_開始時刻_時
    /// DESC: Menu time zone display_evening_start time_hour (SEQ22108) - メニュー用時間帯表示_夕_開始時刻_時(SEQ22108)
    pub menuTimezoneStart_Evening_Hour: u8,

    /// NAME: Menu time zone display_evening_start time_minute - メニュー用時間帯表示_夕_開始時刻_分
    /// DESC: Menu time zone display_evening_start time_minute (SEQ22108) - メニュー用時間帯表示_夕_開始時刻_分(SEQ22108)
    pub menuTimezoneStart_Evening_Minute: u8,

    /// NAME: Menu time zone display_night_start time_hour - メニュー用時間帯表示_夜_開始時刻_時
    /// DESC: Menu time zone display_night_start time_hour (SEQ22108) - メニュー用時間帯表示_夜_開始時刻_時(SEQ22108)
    pub menuTimezoneStart_Night_Hour: u8,

    /// NAME: Menu time zone display_night_start time_minute - メニュー用時間帯表示_夜_開始時刻_分
    /// DESC: Menu time zone display_night_start time_minute (SEQ22108) - メニュー用時間帯表示_夜_開始時刻_分(SEQ22108)
    pub menuTimezoneStart_Night_Minute: u8,

    /// NAME: Menu time zone display_midnight_start time_hour - メニュー用時間帯表示_深夜_開始時刻_時
    /// DESC: Menu time zone display_midnight_start time_hour (SEQ22108) - メニュー用時間帯表示_深夜_開始時刻_時(SEQ22108)
    pub menuTimezoneStart_Midnight_Hour: u8,

    /// NAME: Menu time zone display_midnight_start time_minute - メニュー用時間帯表示_深夜_開始時刻_分
    /// DESC: Menu time zone display_midnight_start time_minute (SEQ22108) - メニュー用時間帯表示_深夜_開始時刻_分(SEQ22108)
    pub menuTimezoneStart_Midnight_Minute: u8,

    /// NAME: Network PC threat level notification_threat level - ネットワークPC脅威度通知_脅威度
    /// DESC: Network PC Threat Level Notification_Threat Level (SEQ21950) - ネットワークPC脅威度通知_脅威度(SEQ21950)
    pub remotePlayerThreatLvNotify_ThreatLv: u16,

    /// NAME: Network PC threat level notification_notification distance [m] - ネットワークPC脅威度通知_通知距離[m]
    /// DESC: Network PC Threat Level Notification_Notification Distance [m] (SEQ21950) - ネットワークPC脅威度通知_通知距離[m](SEQ21950)
    pub remotePlayerThreatLvNotify_NotifyDist: f32,

    /// NAME: Network PC threat level notification_notification end distance [m] - ネットワークPC脅威度通知_通知終了距離[m]
    /// DESC: Network PC threat level notification_notification end distance [m] (SEQ21950) - ネットワークPC脅威度通知_通知終了距離[m](SEQ21950)
    pub remotePlayerThreatLvNotify_EndNotifyDist: f32,

    /// NAME: Default extended distance of map point discovery area [m] - 地図ポイント発見領域のデフォルト拡張距離[m]
    /// DESC: The default extended distance for the map point discovery area. When the "discovery area overwrite area" of the map point is invalid (-1), the discovery area is generated by expanding from its own area. Its extended distance - 地図ポイントの発見領域のデフォルトの拡張距離。地図ポイントの"発見領域 上書き領域"が無効(-1)のときに、自身の領域から拡張して発見領域が生成される。その拡張距離
    pub worldMapPointDiscoveryExpandRange: f32,

    /// NAME: Map point participation area default extended distance [m] - 地図ポイント出場領域のデフォルト拡張距離[m]
    /// DESC: The default extended distance for the map point's entry area. When the "participation area overwrite area" of the map point is invalid (-1), the participation area is generated by expanding from its own area. Its extended distance - 地図ポイントの出場領域のデフォルトの拡張距離。地図ポイントの"出場領域 上書き領域"が無効(-1)のときに、自身の領域から拡張して出場領域が生成される。その拡張距離
    pub worldMapPointReentryExpandRange: f32,

    /// NAME: Network PC threat level notification_Notification time [seconds] - ネットワークPC脅威度通知_通知時間[秒]
    /// DESC: Network PC Threat Level Notification_Notification Time [seconds] (SEQ21950) - ネットワークPC脅威度通知_通知時間[秒](SEQ21950)
    pub remotePlayerThreatLvNotify_NotifyTime: u16,

    /// NAME: Number of "re-invasion items" to be given at the time of intrusion: Intrusion item_Greater system (ID102) - 侵入時に付与する"再侵入アイテム"の付与個数：侵入アイテム_グレーター系（ID102）
    /// DESC: Number of "re-invasion items" to be given at the time of intrusion: Intrusion item_Greater system (ID102) - 侵入時に付与する"再侵入アイテム"の付与個数：侵入アイテム_グレーター系（ID102）
    pub breakIn_A_rebreakInGoodsNum: u16,

    /// NAME: Tool item ID of "re-invasion item" given at the time of intrusion: Invasion item_Greater system (ID102) - 侵入時に付与する"再侵入アイテム"の道具アイテムID：侵入アイテム_グレーター系（ID102）
    /// DESC: Tool item ID of "re-invasion item" given at the time of intrusion: Invasion item_Greater system (ID102) - 侵入時に付与する"再侵入アイテム"の道具アイテムID：侵入アイテム_グレーター系（ID102）
    pub breakIn_A_rebreakInGoodsId: i32,

    /// NAME: Big jump to get off _ sky SFXID - 降りる大ジャンプ_上空SFXID
    /// DESC: Big jump to get off _ sky SFXID - 降りる大ジャンプ_上空SFXID
    pub rideJumpoff_SfxId: i32,

    /// NAME: Big jump to get off_SFX base point offset in the sky - 降りる大ジャンプ_上空SFX基点オフセット
    /// DESC: Big jump to get off_SFX base point offset in the sky - 降りる大ジャンプ_上空SFX基点オフセット
    pub rideJumpoff_SfxHeightOffset: f32,

    /// NAME: In the big jump area to get off _ Special effect ID for PC horse - 降りる大ジャンプ領域内_PC馬にかかる特殊効果ID
    /// DESC: In the big jump area to get off _ Special effect ID for PC horse - 降りる大ジャンプ領域内_PC馬にかかる特殊効果ID
    pub rideJumpoff_SpEffectId: i32,

    /// NAME: Special effect ID applied to _PC in the large jump area to get off - 降りる大ジャンプ領域内_PCにかかる特殊効果ID
    /// DESC: Special effect ID for _PC in the big jump area to get off - 降りる大ジャンプ領域内_PCにかかる特殊効果ID
    pub rideJumpoff_SpEffectIdPc: i32,

    /// NAME: Main menu_Item creation_Open event flag - メインメニュー_アイテム作成_開放イベントフラグ
    /// DESC: Main menu → Event flag to unlock the item creation menu - メインメニュー→アイテム作成メニューをアンロックするイベントフラグ
    pub unlockExchangeMenuEventFlagId: u32,

    /// NAME: Main menu_message_open event flag - メインメニュー_メッセージ_開放イベントフラグ
    /// DESC: Main menu → Event flag to unlock the message menu - メインメニュー→メッセージメニューをアンロックするイベントフラグ
    pub unlockMessageMenuEventFlagId: u32,

    /// NAME: Number of "re-invasion items" to be given at the time of intrusion: Intrusion item_Greater disposable (ID111) - 侵入時に付与する"再侵入アイテム"の付与個数：侵入アイテム_グレーター系使い捨て（ID111）
    /// DESC: Number of "re-invasion items" to be given at the time of intrusion: Intrusion item_Greater disposable (ID111) - 侵入時に付与する"再侵入アイテム"の付与個数：侵入アイテム_グレーター系使い捨て（ID111）
    pub breakInOnce_A_rebreakInGoodsNum: u16,

    /// NAME: Number of "re-invasion items" to be given at the time of invasion: Invasion item_Volcano building system (ID112) - 侵入時に付与する"再侵入アイテム"の付与個数：侵入アイテム_火山館系（ID112）
    /// DESC: Number of "re-invasion items" to be given at the time of invasion: Invasion item_Volcano building system (ID112) - 侵入時に付与する"再侵入アイテム"の付与個数：侵入アイテム_火山館系（ID112）
    pub breakIn_B_rebreakInGoodsNum: u16,

    /// NAME: Tool item ID of "re-invasion item" given at the time of intrusion: Invasion item_Greater disposable (ID111) - 侵入時に付与する"再侵入アイテム"の道具アイテムID：侵入アイテム_グレーター系使い捨て（ID111）
    /// DESC: Tool item ID of "re-invasion item" given at the time of intrusion: Invasion item_Greater disposable (ID111) - 侵入時に付与する"再侵入アイテム"の道具アイテムID：侵入アイテム_グレーター系使い捨て（ID111）
    pub breakInOnce_A_rebreakInGoodsId: i32,

    /// NAME: Tool item ID of "re-invasion item" given at the time of invasion: Invasion item_Volcano building system (ID112) - 侵入時に付与する"再侵入アイテム"の道具アイテムID：侵入アイテム_火山館系（ID112）
    /// DESC: Tool item ID of "re-invasion item" given at the time of invasion: Invasion item_Volcano building system (ID112) - 侵入時に付与する"再侵入アイテム"の道具アイテムID：侵入アイテム_火山館系（ID112）
    pub breakIn_B_rebreakInGoodsId: i32,

    /// NAME: Time to disable action button operation by holding down the action button - アクションボタン押しっぱなしでアクションボタン操作を無効化する時間
    /// DESC: Time to disable action button operation by holding down the action button - アクションボタン押しっぱなしでアクションボタン操作を無効化する時間
    pub actionButtonInputCancelTime: f32,

    /// NAME: Clear bonus acquisition delay time after boss defeat processing - ボス撃破処理後クリアボーナス取得遅延時間
    /// DESC: Clear bonus acquisition delay time after boss defeat processing - ボス撃破処理後クリアボーナス取得遅延時間
    pub blockClearBonusDelayTime: f32,

    /// NAME: [Unused] (Refer to SEQ25048) Distance from the PC to determine the bonfire invalidation by the enemy [m] - 【未使用】(SEQ25048参照）敵による篝火無効化を判定するPCから篝火までの距離[m]
    /// DESC: [Unused] (Refer to SEQ25048) Distance from PC to bonfire that determines invalidation of bonfire by enemy [m] (0 or less: PC distance is not checked. Checked at all distances) - 【未使用】(SEQ25048参照）敵による篝火無効化を判定するPCから篝火までの距離[m](0以下：PC距離チェックしない。全距離でチェック)
    pub bonfireCheckEnemyRange: f32,

    /// NAME: 予約
    /// DESC: (dummy8)
    pub reserved_124: [u8; 128],
}

impl Paramdef for GAME_SYSTEM_COMMON_PARAM_ST {
    const NAME: &'static str = "GAME_SYSTEM_COMMON_PARAM_ST";
    const VERSION: u16 = 3;
}
