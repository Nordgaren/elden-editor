/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 10
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NETWORK_PARAM_ST {

	/// NAME: Sign height offset [m] - サイン高さオフセット[m]
	pub signVerticalOffset:f32,

	/// NAME: Sign position correction maximum distance [m] - サイン位置補正最大距離[m]
	pub maxSignPosCorrectionRange:f32,

	/// NAME: Summon desired timeout time [seconds] - 召喚希望タイムアウト時間[秒]
	pub summonTimeoutTime:f32,

	/// NAME: reserve - 予約
	pub pad_0:[u8;4],

	/// NAME: Message display interval [seconds] during registration of signature accumulation - サイン溜まり登録中メッセージ表示間隔[秒]
	pub signPuddleActiveMessageIntervalSec:f32,

	/// NAME: Key guide vertical range [m] - キーガイド垂直範囲[m]
	pub keyGuideHeight_0:f32,

	/// NAME: Waiting time for reacquisition of summoning sign (during depopulation) [seconds] - 召喚サイン再取得待機時間(過疎時)[秒]
	pub reloadSignIntervalTime1:f32,

	/// NAME: Summon sign reacquisition waiting time [seconds] - 召喚サイン再取得待機時間[秒]
	pub reloadSignIntervalTime2:f32,

	/// NAME: Maximum number of summon signs you can have (overall) - 召喚サイン所持可能数上限(全体)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadSignTotalCount_0:u32,

	/// NAME: Maximum number of summon signs you can have (cell) - 召喚サイン所持可能数上限(セル)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadSignCellCount_0:u32,

	/// NAME: Summon sign update waiting time [seconds] - 召喚サイン更新待機時間[秒]
	pub updateSignIntervalTime:f32,

	/// NAME: Exclusive horizontal range for drawing between summon signs [m] - 召喚サイン間描画排他水平範囲[m]
	pub basicExclusiveRange_0:f32,

	/// NAME: Exclusive vertical range for drawing between summon signs [m] - 召喚サイン間描画排他垂直範囲[m]
	pub basicExclusiveHeight_0:f32,

	/// NAME: Summon sign character model drawing waiting time [seconds] - 召喚サインキャラモデル描画待機時間[秒]
	pub previewChrWaitingTime:f32,

	/// NAME: Summon sign PC drawing distance [m] - 召喚サインPC間描画距離[m]
	pub signVisibleRange_0:f32,

	/// NAME: Summon sign acquisition cell range (horizontal) - 召喚サイン取得セル範囲(水平)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupHorizontalRange_0:u32,

	/// NAME: Summon sign acquisition cell range (upward) - 召喚サイン取得セル範囲(上方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupTopRange_0:u32,

	/// NAME: Summon sign acquisition cell range (downward) - 召喚サイン取得セル範囲(下方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupBottomRange_0:u32,

	/// NAME: White spirit sign display time limit lower limit magnification - 白霊サイン表示制限時間下限倍率
	pub minWhitePhantomLimitTimeScale:f32,

	/// NAME: Small spirit sign display time limit lower limit magnification - 小霊サイン表示制限時間下限倍率
	pub minSmallPhantomLimitTimeScale:f32,

	/// NAME: White spirit sign keyword extension magnification - 白霊サインキーワード延長倍率
	pub whiteKeywordLimitTimeScale:f32,

	/// NAME: Ghost sign keyword extension magnification - 小霊サインキーワード延長倍率
	pub smallKeywordLimitTimeScale:f32,

	/// NAME: Dark Spirit Sign Keyword Extension Magnification - 闇霊サインキーワード延長倍率
	pub blackKeywordLimitTimeScale:f32,

	/// NAME: Dragon Spirit Sign Keyword Extension Magnification - 竜霊サインキーワード延長倍率
	pub dragonKeywordLimitTimeScale:f32,

	/// NAME: Sign acquisition limit - サイン取得上限
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub singGetMax:u32,

	/// NAME: Sign download span - サインダウンロードスパン
	pub signDownloadSpan:f32,

	/// NAME: Sign upload span - サインアップロードスパン
	pub signUpdateSpan:f32,

	/// NAME: reserve - 予約
	pub signPad:[u8;4],

	/// NAME: Number of intruders acquired - 乱入先取得数
	pub maxBreakInTargetListCount:u32,

	/// NAME: Intrusion request interval [seconds] - 乱入リクエスト間隔[秒]
	pub breakInRequestIntervalTimeSec:f32,

	/// NAME: Intrusion request timeout time [seconds] - 乱入リクエストタイムアウト時間[秒]
	pub breakInRequestTimeOutSec:f32,

	/// NAME: reserve - 予約
	pub pad_1:[u8;4],

	/// NAME: Key guide horizontal range [m] - キーガイド水平範囲[m]
	pub keyGuideRange:f32,

	/// NAME: Key guide vertical range [m] - キーガイド垂直範囲[m]
	pub keyGuideHeight_1:f32,

	/// NAME: Number of blood characters acquired (overall) - 血文字取得数(全体)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadSignTotalCount_1:u32,

	/// NAME: Number of blood characters acquired (cell, new order) - 血文字取得数(セル、新順)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadNewSignCellCount:u32,

	/// NAME: Number of blood characters acquired (cell, random) - 血文字取得数(セル、ランダム )
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadRandomSignCellCount:u32,

	/// NAME: Upper limit of the number of blood characters that can be possessed (overall) - 血文字所持可能数上限(全体)
	/// DESC: Actually u16 is enough - 本当はu16くらいで十分
	pub maxSignTotalCount_0:u32,

	/// NAME: Upper limit of the number of blood characters that can be possessed (cell) - 血文字所持可能数上限(セル)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub maxSignCellCount_0:u32,

	/// NAME: Blood character drawing exclusive horizontal range [m] - 血文字間描画排他水平範囲[m]
	pub basicExclusiveRange_1:f32,

	/// NAME: Blood character drawing exclusive vertical range [m] - 血文字間描画排他垂直範囲[m]
	pub basicExclusiveHeight_1:f32,

	/// NAME: Blood character PC drawing distance [m] - 血文字PC間描画距離[m]
	pub signVisibleRange_1:f32,

	/// NAME: Maximum number of written blood characters history - 書いた血文字履歴件数上限
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub maxWriteSignCount:u32,

	/// NAME: Maximum number of blood characters read - 読んだ血文字履歴件数上限
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub maxReadSignCount:u32,

	/// NAME: Blood character reacquisition waiting time [seconds] - 血文字再取得待機時間[秒]
	pub reloadSignIntervalTime_0:f32,

	/// NAME: Blood character acquisition cell range (horizontal) - 血文字取得セル範囲(水平)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupHorizontalRange_1:u32,

	/// NAME: Blood character acquisition cell range (upward) - 血文字取得セル範囲(上方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupTopRange_1:u32,

	/// NAME: Blood character acquisition cell range (downward) - 血文字取得セル範囲(下方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupBottomRange_1:u32,

	/// NAME: Blood character data retention period upper limit [seconds] - 血文字データ保持期間上限[秒]
	/// DESC: Actually u16 is enough - 本当はu16くらいで十分
	pub lifeTime_0:u32,

	/// NAME: Blood character download interval - 血文字ダウンロード間隔
	pub downloadSpan_0:f32,

	/// NAME: Blood character evaluation number Download interval - 血文字評価数ダウンロード間隔
	pub downloadEvaluationSpan:f32,

	/// NAME: reserve - 予約
	pub pad_2:[u8;4],

	/// NAME: Allowable distance between bloodstain position and illusion start position [m] - 血痕位置と幻影開始位置間の許容距離[m]
	/// DESC: If the distance between the bloodstain position and the illusion start position is farther than this value, the server will not be registered. - 血痕位置と幻影開始位置の間の距離がこの値より離れている場合はサーバの登録を行わない
	pub deadingGhostStartPosThreshold:f32,

	/// NAME: Key guide vertical range [m] - キーガイド垂直範囲[m]
	pub keyGuideHeight_2:f32,

	/// NAME: Player Bloodstain Key Guide Horizontal Range [m] - プレイヤー血痕キーガイド水平範囲[m]
	pub keyGuideRangePlayer:f32,

	/// NAME: Player Bloodstain Key Guide Vertical Range [m] - プレイヤー血痕キーガイド垂直範囲[m]
	pub keyGuideHeightPlayer:f32,

	/// NAME: Number of blood stains acquired (overall) - 血痕取得数(全体)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadSignTotalCount_2:u32,

	/// NAME: Number of blood stains obtained (cell) - 血痕取得数(セル)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadSignCellCount_1:u32,

	/// NAME: Upper limit of the number of blood stains that can be possessed (overall) - 血痕所持可能数上限(全体)
	/// DESC: Actually u16 is enough - 本当はu16くらいで十分
	pub maxSignTotalCount_1:u32,

	/// NAME: Maximum number of blood stains (cell) - 血痕所持可能数上限(セル)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub maxSignCellCount_1:u32,

	/// NAME: Waiting time for reacquisition of blood stains [seconds] - 血痕再取得待機時間[秒]
	pub reloadSignIntervalTime_1:f32,

	/// NAME: Bloodstain PC drawing distance [m] - 血痕PC間描画距離[m]
	pub signVisibleRange_2:f32,

	/// NAME: Exclusive horizontal range for drawing between blood stains [m] - 血痕間描画排他水平範囲[m]
	pub basicExclusiveRange_2:f32,

	/// NAME: Exclusive vertical range for drawing between blood stains [m] - 血痕間描画排他垂直範囲[m]
	pub basicExclusiveHeight_2:f32,

	/// NAME: Blood stain acquisition cell range (horizontal) - 血痕取得セル範囲(水平)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupHorizontalRange_2:u32,

	/// NAME: Blood stain acquisition cell range (upward) - 血痕取得セル範囲(上方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupTopRange_2:u32,

	/// NAME: Blood stain acquisition cell range (downward) - 血痕取得セル範囲(下方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupBottomRange_2:u32,

	/// NAME: Bloodstain data retention period upper limit [seconds] - 血痕データ保持期間上限[秒]
	/// DESC: Actually u16 is enough - 本当はu16くらいで十分
	pub lifeTime_1:u32,

	/// NAME: Death illusion record total time [seconds] - 死亡幻影記録合計時間[秒]
	pub recordDeadingGhostTotalTime:f32,

	/// NAME: Minimum recording time of death illusion [seconds] - 死亡幻影の最低記録時間[秒]
	/// DESC: Death illusions less than this recording time will not register the server - この記録時間未満の死亡幻影はサーバの登録を行わない
	pub recordDeadingGhostMinTime:f32,

	/// NAME: Bloodstain download interval - 血痕ダウンロード間隔
	pub downloadSpan_1:f32,

	/// NAME: Petrified blood stain drawing limit distance [m] - 石化血痕描画制限距離[m]
	/// DESC: For open fields. When a stone statue is generated, it can be generated if the distance between the PC and the generation position is greater than or equal to this value. - オープンフィールド用。石像生成時にPC～生成位置間の距離がこの値以上ならば生成できる
	pub statueCreatableDistance:f32,

	/// NAME: Number of illusions acquired (overall) - 幻影取得数(全体)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadGhostTotalCount:u32,

	/// NAME: Number of illusions acquired (cell) - 幻影取得数(セル)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub reloadGhostCellCount:u32,

	/// NAME: Maximum number of illusions you can have (overall) - 幻影所持可能数上限(全体)
	/// DESC: Actually u16 is enough - 本当はu16くらいで十分
	pub maxGhostTotalCount:u32,

	/// NAME: Hostile PC replay recording start distance [m] - 敵対PCリプレイ記録開始距離[m]
	pub distanceOfBeginRecordVersus:f32,

	/// NAME: Hostile PC replay recording end distance [m] - 敵対PCリプレイ記録終了距離[m]
	pub distanceOfEndRecordVersus:f32,

	/// NAME: Wandering illusion upload interval [seconds] - 徘徊幻影アップロード間隔[秒]
	pub updateWanderGhostIntervalTime:f32,

	/// NAME: Battle illusion upload interval [seconds] - 対戦幻影アップロード間隔[秒]
	pub updateVersusGhostIntervalTime:f32,

	/// NAME: Phantom recording time [seconds] - 幻影記録時間[秒]
	pub recordWanderingGhostTime:f32,

	/// NAME: Minimum recording time of wandering illusion [seconds] - 徘徊幻影の最低記録時間[秒]
	/// DESC: Wandering illusions less than this recording time do not register the server - この記録時間未満の徘徊幻影はサーバの登録を行わない
	pub recordWanderingGhostMinTime:f32,

	/// NAME: Kagaribi illusion upload interval [seconds] - 篝火幻影アップロード間隔[秒]
	pub updateBonfireGhostIntervalTime:f32,

	/// NAME: Phantom reproduction distance (in the field of view) [seconds] - 幻影再生距離（視野内）[秒]
	pub replayGhostRangeOnView:f32,

	/// NAME: Phantom playback distance (out of field of view) [seconds] - 幻影再生距離（視野外）[秒]
	pub replayGhostRangeOutView:f32,

	/// NAME: Kagaribi illusion Playing time [seconds] - 篝火幻影再生時間[秒]
	pub replayBonfireGhostTime:f32,

	/// NAME: Kagaribi illusion placement minimum distance [seconds] - 篝火幻影配置最小距離[秒]
	/// DESC: Do not place bonfire illusions less than this distance from the bonfire - 篝火からこの距離未満の場所には篝火幻影を配置しない
	pub minBonfireGhostValidRange:f32,

	/// NAME: Kagaribi illusion placement maximum distance [seconds] - 篝火幻影配置最大距離[秒]
	/// DESC: Do not place bonfire illusions beyond this distance from the bonfire - 篝火からこの距離以上の場所には篝火幻影を配置しない
	pub maxBonfireGhostValidRange:f32,

	/// NAME: Phantom playback interval lower limit [seconds] - 幻影再生間隔下限[秒]
	pub minReplayIntervalTime:f32,

	/// NAME: Phantom playback interval upper limit [seconds] - 幻影再生間隔上限[秒]
	pub maxReplayIntervalTime:f32,

	/// NAME: Phantom periodic acquisition interval [seconds] - 幻影定期取得間隔[秒]
	pub reloadGhostIntervalTime:f32,

	/// NAME: Phantom acquisition cell range (horizontal) - 幻影取得セル範囲(水平)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupHorizontalRange_3:u32,

	/// NAME: Phantom acquisition cell range (upward) - 幻影取得セル範囲(上方向)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub cellGroupTopRange_3:u32,

	/// NAME: Phantom bonfire mode phantom parameter ID (codename) - 幻影篝火モードファントムパラメータID(コードネーム)
	/// DESC: Phantom bonfire mode phantom parameter ID used when codename matches - コードネーム一致のときに使われる幻影篝火モードファントムパラメータID
	pub replayBonfirePhantomParamIdForCodename:i32,

	/// NAME: Phantom bonfire mode playback applicable distance - 幻影篝火モード再生適用距離
	pub replayBonfireModeRange:f32,

	/// NAME: Phantom bonfire mode phantom parameter ID - 幻影篝火モードファントムパラメータID
	/// DESC: Phantom bonfire mode phantom parameter ID - 幻影篝火モードファントムパラメータID
	pub replayBonfirePhantomParamId:i32,

	/// NAME: reserve - 予約
	pub ghostpad:[u8;4],

	/// NAME: Ring search interval [seconds] - 指輪検索間隔[秒]
	pub reloadVisitListCoolTime:f32,

	/// NAME: Maximum number of rescue blue spirits appearing - 救援青霊出現数上限
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub maxCoopBlueSummonCount:u32,

	/// NAME: Upper limit of the number of bell guard ghosts that appear - 鐘守灰霊出現数上限
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub maxBellGuardSummonCount:u32,

	/// NAME: Number of ring search destinations acquired - 指輪検索先取得数
	pub maxVisitListCount:u32,

	/// NAME: Relief Blue Spirit reload time minimum [sec] - 救援青霊リロード時間　最小[sec]
	pub reloadSearch_CoopBlue_Min:f32,

	/// NAME: Relief Blue Spirit reload time maximum [sec] - 救援青霊リロード時間　最大[sec]
	pub reloadSearch_CoopBlue_Max:f32,

	/// NAME: Kanemori reload time minimum [sec] - 鐘守リロード時間　最小[sec]
	pub reloadSearch_BellGuard_Min:f32,

	/// NAME: Kanemori reload time maximum [sec] - 鐘守リロード時間　最大[sec]
	pub reloadSearch_BellGuard_Max:f32,

	/// NAME: Mouse King Reload Time Minimum [sec] - ネズミの王リロード時間　最小[sec]
	pub reloadSearch_RatKing_Min:f32,

	/// NAME: Mouse King Reload Time Maximum [sec] - ネズミの王リロード時間　最大[sec]
	pub reloadSearch_RatKing_Max:f32,

	/// NAME: reserve - 予約
	pub visitpad00:[u8;8],

	/// NAME: SRTT upper limit [milliseconds] - SRTT上限[ミリ秒]
	pub srttMaxLimit:f32,

	/// NAME: SRTT upper limit (when stable) [milliseconds] - SRTT上限(安定時)[ミリ秒]
	pub srttMeanLimit:f32,

	/// NAME: RTT mean deviation upper limit [milliseconds] - RTT平均偏差上限[ミリ秒]
	pub srttMeanDeviationLimit:f32,

	/// NAME: Dark spirit time limit Acceleration time [seconds] - 闇霊制限時間加速時間[秒]
	pub darkPhantomLimitBoostTime:f32,

	/// NAME: Dark spirit time limit acceleration time magnification - 闇霊制限時間加速時倍率
	pub darkPhantomLimitBoostScale:f32,

	/// NAME: Multiplayer invalidation life - マルチプレイ無効化寿命
	pub multiplayDisableLifeTime:f32,

	/// NAME: Abyss Spirit Multiplayer Count - 深淵霊マルチプレイ回数
	/// DESC: The number of times the abyss spirit enters the host in the abyss area - 深淵エリアで、深淵霊がホストに入ってこれる回数
	pub abyssMultiplayLimit:u8,

	/// NAME: Minimum time for a ghost to warp [seconds] - 霊体がワープするまでの最低時間[秒]
	pub phantomWarpMinimumTime:u8,

	/// NAME: Delay time until returning after using black crystal [seconds] - 黒水晶使用後に帰還するまでのディレイ時間[秒]
	pub phantomReturnDelayTime:u8,

	/// NAME: Time-out time waiting for disconnection - 切断待ちのタイムアウト時間
	pub terminateTimeoutTime:u8,

	/// NAME: Penalty addition value without LAN - LAN抜きによるペナルティ加算値
	pub penaltyPointLanDisconnect:u16,

	/// NAME: Penalty addition value for sign-out - サインアウトによるペナルティ加算値
	pub penaltyPointSignout:u16,

	/// NAME: Penalty addition value due to power failure - 電源断によるペナルティ加算値
	pub penaltyPointReboot:u16,

	/// NAME: Penalty value that activates the penalty - ペナルティが発動するペナルティ値
	pub penaltyPointBeginPenalize:u16,

	/// NAME: Sales time limit for "Line Reason" [seconds] - 「線の理」の販売制限時間[秒]
	pub penaltyForgiveItemLimitTime:f32,

	/// NAME: Area search rate: Relief Blue Spirit [0-100] - 全域検索率：救援青霊[0-100]
	/// DESC: Percentage of searching for intrusion targets from all areas (%) - 全域から侵入対象を検索する割合（％）
	pub allAreaSearchRate_CoopBlue:u8,

	/// NAME: Area search rate: Retribution Blue Spirit [0-100] - 全域検索率：報復青霊[0-100]
	/// DESC: Percentage of searching for intrusion targets from all areas (%) - 全域から侵入対象を検索する割合（％）
	pub allAreaSearchRate_VsBlue:u8,

	/// NAME: Area search rate: Kanemori Ashrei [0-100] - 全域検索率：鐘守灰霊[0-100]
	/// DESC: Percentage of searching for intrusion targets from all areas (%) - 全域から侵入対象を検索する割合（％）
	pub allAreaSearchRate_BellGuard:u8,

	/// NAME: HP recovery rate at the time of blood character evaluation [0-100] - 血文字評価時のHP回復割合[0-100]
	pub bloodMessageEvalHealRate:u8,

	/// NAME: Kogane Rei Success Return Host Reward ID - 小金霊成功帰還ホスト報酬ID
	pub smallGoldSuccessHostRewardId:u32,

	/// NAME: Play area invalidation distance near the door [m] - ドア付近プレイ領域無効化距離[m]
	/// DESC: The area around the black door that separates the multiplayer area is set to the systematically invalid play area (-1). At that time, make the invalid area thicker with this parameter on the thinner side of the OBJ's bounding box with a black door. - マルチプレイ領域を区切る黒扉の周辺を、システム的に無効なプレイ領域（-1）にします。その際、無効領域を黒扉のOBJのバウンディングボックスの薄い方を、このパラメータで太らせます。
	pub doorInvalidPlayAreaExtents:f32,

	/// NAME: Maximum number of simultaneous displays of signs - サイン最大同時表示数
	pub signDisplayMax:u8,

	/// NAME: Maximum number of blood stains displayed at the same time - 血痕最大同時表示数
	pub bloodStainDisplayMax:u8,

	/// NAME: Maximum number of blood characters displayed at the same time - 血文字最大同時表示数
	pub bloodMessageDisplayMax:u8,

	/// NAME: reserve - 予約
	pub pad00:[u8;9],

	/// NAME: reserve - 予約
	pub pad10:[u8;32],

	/// NAME: Summon message is displayed at interval [seconds] - 召喚メッセージが表示間隔[秒]
	pub summonMessageInterval:f32,

	/// NAME: Host periodic update request interval [seconds] - ホスト定期更新リクエスト間隔[秒]
	pub hostRegisterUpdateTime:f32,

	/// NAME: Host guest join wait timeout time [seconds] - ホストのゲスト参加待ちタイムアウト時間[秒]
	pub hostTimeOutTime:f32,

	/// NAME: Authentication wait timeout time from guest host [seconds] - ゲストのホストからの認証待ちタイムアウト時間[秒]
	pub guestUpdateTime:f32,

	/// NAME: Guest PlayNo Sync wait timeout time [seconds] - ゲストPlayNo同期待ちタイムアウト時間[秒]
	pub guestPlayerNoTimeOutTime:f32,

	/// NAME: Host PlayNo Sync wait timeout time [seconds] - ホストPlayNo同期待ちタイムアウト時間[秒]
	pub hostPlayerNoTimeOutTime:f32,

	/// NAME: RequestSearchQuickMatch limit value - RequestSearchQuickMatchのlimit値
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub requestSearchQuickMatchLimit:u32,

	/// NAME: Maximum number of people when searching for avatar battles (unused) - アバター戦検索時の最大人数(未使用)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub AvatarMatchSearchMax:u32,

	/// NAME: Minimum number of people when searching for battle royale (unused) - バトルロイヤル戦検索時の最少人数(未使用)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub BattleRoyalMatchSearchMin:u32,

	/// NAME: Maximum number of people when searching for battle royale (unused) - バトルロイヤル戦検索時の最大人数(未使用)
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub BattleRoyalMatchSearchMax:u32,

	/// NAME: reserve - 予約
	pub pad11:[u8;8],

	/// NAME: Maximum number of people to get a list of visitors - 訪問対象者リスト取得最大値
	/// DESC: Actually u8 is enough - 本当はu8くらいで十分
	pub VisitorListMax:u32,

	/// NAME: Time-out waiting for visit - 訪問待ちタイムアウト
	pub VisitorTimeOutTime:f32,

	/// NAME: Visitor list download interval [seconds] - 訪問者リストダウンロード間隔[秒]
	pub DownloadSpan:f32,

	/// NAME: Visit search message display interval [seconds] - 訪問先検索メッセージ表示間隔[秒]
	/// DESC: Display interval [seconds] of messages sent by visiting guests while searching for a destination - 訪問ゲストが訪問先を探してる間に出すメッセージの表示間隔[秒]
	pub VisitorGuestRequestMessageIntervalSec:f32,

	/// NAME: Wandering illusion life - 徘徊幻影寿命
	/// DESC: Wandering illusion life - 徘徊幻影寿命
	pub wanderGhostIntervalLifeTime:f32,

	/// NAME: reserve - 予約
	/// DESC: reserve - 予約
	pub pad13:[u8;12],

	/// NAME: Yellow robe's old man waiting time out - 黄衣の翁待ちタイムアウト
	pub YellowMonkTimeOutTime:f32,

	/// NAME: Yellow robe list download interval - 黄衣の翁リストダウンロード間隔
	pub YellowMonkDownloadSpan:f32,

	/// NAME: Yellow robe's whole flow time-out - 黄衣の翁全体フロータイムアウト
	pub YellowMonkOverallFlowTimeOutTime:f32,

	/// NAME: reserve - 予約
	pub pad14_0:[u8;4],

	/// NAME: reserve - 予約
	pub pad14_1:[u8;8],
}

impl Paramdef for NETWORK_PARAM_ST {
const NAME: &'static str = "NETWORK_PARAM_ST";
const VERSION: u16 = 10;
}
