/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SP_EFFECT_PARAM_ST {

	/// NAME: Icon ID - アイコンID
	/// DESC: Icon ID (When -1, no icon is required) - アイコンID(-1の時は、アイコン必要なし)
	pub iconId:i32,

	/// NAME: Trigger condition: Remaining HP ratio [%] - 発動条件　残りHP比率[%]
	/// DESC: Set what percentage of maxHP the remaining HP will be activated - 残りHPが、maxHPの何%になったら発動するかを設定
	pub conditionHp:f32,

	/// NAME: Effect duration time [s] - 効果持続時間　時間[s]
	/// DESC: Change duration / -1 for permanent / 0 for one moment only - 変化持続時間　/-1で永続 /0で瞬間1回限り
	pub effectEndurance:f32,

	/// NAME: Activation interval [s] - 発動間隔[s]
	/// DESC: Set how many seconds it occurs - 何秒間隔で発生するのかを設定
	pub motionInterval:f32,

	/// NAME: Maximum HP magnification [%] - 最大HP倍率[%]
	/// DESC: Correct the maximum HP - 最大HPに補正をかける
	pub maxHpRate:f32,

	/// NAME: Maximum MP magnification [%] - 最大MP倍率[%]
	/// DESC: Correct the maximum MP - 最大MPに補正をかける
	pub maxMpRate:f32,

	/// NAME: Maximum stamina magnification [%] - 最大スタミナ倍率[%]
	/// DESC: Correct the maximum SP - 最大SPに補正をかける
	pub maxStaminaRate:f32,

	/// NAME: Defender: Slash damage multiplier - 防御側：斬撃ダメージ倍率
	/// DESC: Slash damage ratio: Correct the calculated damage by XX times. 1 is normal. - 斬撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub slashDamageCutRate:f32,

	/// NAME: Defender: Batter damage multiplier - 防御側：打撃ダメージ倍率
	/// DESC: Batter damage ratio: The calculated damage is corrected by XX times. 1 is normal. - 打撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub blowDamageCutRate:f32,

	/// NAME: Defender: Puncture damage multiplier - 防御側：刺突ダメージ倍率
	/// DESC: Puncture damage ratio: Correct the calculated damage by XX times. 1 is normal. - 刺突ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub thrustDamageCutRate:f32,

	/// NAME: Defender: Non-attribute damage multiplier - 防御側：無属性ダメージ倍率
	/// DESC: Non-attribute damage ratio: Correct the calculated damage by XX times. 1 is normal. - 無属性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub neutralDamageCutRate:f32,

	/// NAME: Defender: Magic damage multiplier - 防御側：魔法ダメージ倍率
	/// DESC: Magic damage multiplier: The calculated damage is corrected by XX times. 1 is normal. - 魔法ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub magicDamageCutRate:f32,

	/// NAME: Defender: Fire damage multiplier - 防御側：炎ダメージ倍率
	/// DESC: Flame damage ratio: Correct the calculated damage by XX times. 1 is normal. - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub fireDamageCutRate:f32,

	/// NAME: Defender: Electric shock damage multiplier - 防御側：電撃ダメージ倍率
	/// DESC: Electric shock damage ratio: Correct the calculated damage by XX times. 1 is normal. - 電撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub thunderDamageCutRate:f32,

	/// NAME: Attacker: Physical damage multiplier - 攻撃側：物理ダメージ倍率
	/// DESC: Physical damage ratio: Correct the calculated damage by XX times. 1 is normal. - 物理ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub physicsAttackRate:f32,

	/// NAME: Attacker: Magic damage multiplier - 攻撃側：魔法ダメージ倍率
	/// DESC: Magic damage multiplier: The calculated damage is corrected by XX times. 1 is normal. - 魔法ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub magicAttackRate:f32,

	/// NAME: Attacker: Fire damage multiplier - 攻撃側：炎ダメージ倍率
	/// DESC: Flame damage ratio: Correct the calculated damage by XX times. 1 is normal. - 炎ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub fireAttackRate:f32,

	/// NAME: Attack side: Electric shock damage ratio - 攻撃側：電撃ダメージ倍率
	/// DESC: Electric shock damage ratio: Correct the calculated damage by XX times. 1 is normal. - 電撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub thunderAttackRate:f32,

	/// NAME: Physical attack power multiplier - 物理攻撃力倍率
	/// DESC: Multiply the physical attack power by the set value - 物理攻撃力に設定した数値をかけます
	pub physicsAttackPowerRate:f32,

	/// NAME: Magic attack power multiplier - 魔法攻撃力倍率
	/// DESC: Multiply the magic attack power by the set value - 魔法攻撃力に設定した数値をかけます
	pub magicAttackPowerRate:f32,

	/// NAME: Fire attack power multiplier - 炎攻撃力倍率
	/// DESC: Multiply the fire attack power by the set value - 炎攻撃力に設定した数値をかけます
	pub fireAttackPowerRate:f32,

	/// NAME: Electric shock attack power multiplier - 電撃攻撃力倍率
	/// DESC: Multiply the electric shock attack power by the set value - 電撃攻撃力に設定した数値をかけます
	pub thunderAttackPowerRate:f32,

	/// NAME: Physical attack power [point] - 物理攻撃力[point]
	/// DESC: Add or subtract the value set for physical attack power - 物理攻撃力に設定した数値を加減算する
	pub physicsAttackPower:i32,

	/// NAME: Magic attack power [point] - 魔法攻撃力[point]
	/// DESC: Add or subtract the value set for the magic attack power - 魔法攻撃力に設定した数値を加減算する
	pub magicAttackPower:i32,

	/// NAME: Fire attack power [point] - 炎攻撃力[point]
	/// DESC: Add or subtract the value set for the flame attack power - 炎攻撃力に設定した数値を加減算する
	pub fireAttackPower:i32,

	/// NAME: Electric shock attack power [point] - 電撃攻撃力[point]
	/// DESC: Add or subtract the value set for the electric shock attack power - 電撃攻撃力に設定した数値を加減算する
	pub thunderAttackPower:i32,

	/// NAME: Physical defense power multiplier - 物理防御力倍率
	/// DESC: Multiply the set value for physical defense - 物理防御力に設定した数値をかけます
	pub physicsDiffenceRate:f32,

	/// NAME: Magic defense multiplier - 魔法防御力倍率
	/// DESC: Multiply the value set for magic defense - 魔法防御力に設定した数値をかけます
	pub magicDiffenceRate:f32,

	/// NAME: Fire defense multiplier - 炎防御力倍率
	/// DESC: Multiply the fire defense by the set value - 炎防御力に設定した数値をかけます
	pub fireDiffenceRate:f32,

	/// NAME: Electric shock defense power multiplier - 電撃防御力倍率
	/// DESC: Multiply the value set for the electric shock defense - 電撃防御力に設定した数値をかけます
	pub thunderDiffenceRate:f32,

	/// NAME: Physical defense [point] - 物理防御力[point]
	/// DESC: Add or subtract the value set for physical defense - 物理防御力に設定した数値を加減算する
	pub physicsDiffence:i32,

	/// NAME: Magic defense [point] - 魔法防御力[point]
	/// DESC: Add or subtract the value set for magic defense - 魔法防御力に設定した数値を加減算する
	pub magicDiffence:i32,

	/// NAME: Fire defense [point] - 炎防御力[point]
	/// DESC: Add or subtract the value set for the fire defense - 炎防御力に設定した数値を加減算する
	pub fireDiffence:i32,

	/// NAME: Electric shock defense [point] - 電撃防御力[point]
	/// DESC: Add or subtract the value set for the electric shock defense - 電撃防御力に設定した数値を加減算する
	pub thunderDiffence:i32,

	/// NAME: Gap damage ratio - 隙ダメージ倍率
	/// DESC: Replace the damage ratio at the time of the gap with the set value (set on the damage side) - 隙のときのダメージ倍率を、設定した数値に置き換える（ダメージ側に設定）
	pub NoGuardDamageRate:f32,

	/// NAME: Sweet spot magnification - スィートスポット倍率
	/// DESC: Replaces the damage calculation of the sweet spot with the specified value (key point damage correction) -Invalid at 1.0 - スィートスポットのダメージ計算を指定した数値に差し替える(急所ダメージ補正) -1.0で無効
	pub vitalSpotChangeRate:f32,

	/// NAME: Normal hit magnification - ノーマルヒット倍率
	/// DESC: Replaces normal hit damage calculation with the specified number -Invalid at 1.0 - ノーマルヒットのダメージ計算を指定した数値に差し替える  -1.0で無効
	pub normalSpotChangeRate:f32,

	/// NAME: LookAt Position Offset [m] - LookAt位置オフセット[m]
	/// DESC: Offset the target position when the enemy looks at. Set to crouch or mount on the side to be seen - 敵がLookAtする際に目標位置をオフセットする。見られる側のしゃがみや騎乗に設定する
	pub lookAtTargetPosOffset:f32,

	/// NAME: Action ID designation frame - 行動ID指定枠
	/// DESC: Specified -1 when dealing damage using action ID from special effects - 特殊効果から行動IDを使ってダメージを与える場合に指定-1で無効
	pub behaviorId:i32,

	/// NAME: HP damage amount [%] - HPダメージ量[%]
	/// DESC: Set what percentage of the maximum HP to subtract (or add) with one activation - 一度の発動で最大HPの何%分を減算（または加算）するかを設定
	pub changeHpRate:f32,

	/// NAME: HP damage [point] - HPダメージ[point]
	/// DESC: Set how many points to subtract (or add) with one activation - 一度の発動で何ポイント減算（または加算）するかを設定
	pub changeHpPoint:i32,

	/// NAME: MP damage amount [%] - MPダメージ量[%]
	/// DESC: Set what percentage of the maximum MP to subtract (or add) with one activation - 一度の発動で最大MPの何%分を減算（または加算）するかを設定
	pub changeMpRate:f32,

	/// NAME: MP damage [point] - MPダメージ[point]
	/// DESC: Set how many points to subtract (or add) with one activation - 一度の発動で何ポイント減算（または加算）するかを設定
	pub changeMpPoint:i32,

	/// NAME: MP recovery speed change [point] - MP回復速度変化[point]
	/// DESC: Change the recovery speed. Add or subtract to the standard recovery speed and initial recovery speed of the recovery formula. - 回復速度を変化させる。回復計算式の基準回復速度、初期回復速度に加減算する。
	pub mpRecoverChangeSpeed:i32,

	/// NAME: Stamina damage amount [%] - スタミナダメージ量[%]
	/// DESC: Set what percentage of the maximum stamina to subtract (or add) with one activation - 一度の発動で最大スタミナの何%分を減算（または加算）するかを設定
	pub changeStaminaRate:f32,

	/// NAME: Stamina damage [point] - スタミナダメージ[point]
	/// DESC: Set how many points to subtract (or add) with one activation - 一度の発動で何ポイント減算（または加算）するかを設定
	pub changeStaminaPoint:i32,

	/// NAME: Stamina recovery speed change [point] - スタミナ回復速度変化[point]
	/// DESC: Change the recovery speed. Add or subtract to the standard recovery speed and initial recovery speed of the recovery formula. - 回復速度を変化させる。回復計算式の基準回復速度、初期回復速度に加減算する。
	pub staminaRecoverChangeSpeed:i32,

	/// NAME: Magic effect time change - 魔法効果時間変化
	/// DESC: Add / subtract the time set for the effect duration only for magic that has the effect duration set to 0.1 seconds or more. - 効果持続時間に0.1秒以上設定されている魔法のみ、効果持続時間に設定されている時間を加減算する
	pub magicEffectTimeChange:f32,

	/// NAME: Durability change: Internal wear [point] - 耐久度変化：内部損耗度[point]
	/// DESC: Add or subtract the numerical value to the internal wear degree - 内部損耗度に数値分を加減算する
	pub insideDurability:i32,

	/// NAME: Durability change: Maximum wear change [point] - 耐久度変化：最大損耗度変化[point]
	/// DESC: Add the set value to the maximum value of the internal wear degree of durability. - 耐久度の内部損耗度の最大値に、設定された数値を加算する
	pub maxDurability:i32,

	/// NAME: Stamina attack power multiplier - スタミナ攻撃力倍率
	/// DESC: Multiply the stamina attack power by a factor (1.0 1 times 0.5 half) - スタミナ攻撃力に、倍率をかける(1.0 1倍 0.5 半分）
	pub staminaAttackRate:f32,

	/// NAME: Poison resistance attack power [point] - 毒耐性攻撃力[point]
	/// DESC: A value to be added to the target's [poison resistance value] when it hits - ヒットした時に、対象の【毒耐性値】に加算する数値
	pub poizonAttackPower:i32,

	/// NAME: Epidemic resistance attack power [point] - 疫病耐性攻撃力[point]
	/// DESC: Numerical value to be added to the target [Plague resistance value] when hit - ヒットした時に、対象の【疫病耐性値】に加算する数値
	pub diseaseAttackPower:i32,

	/// NAME: Bleeding resistance attack power [point] - 出血耐性攻撃力[point]
	/// DESC: A value to be added to the target's [bleeding resistance value] when it hits - ヒットした時に、対象の【出血耐性値】に加算する数値
	pub bloodAttackPower:i32,

	/// NAME: Curse resistance attack power [point] - 呪耐性攻撃力[point]
	/// DESC: A number to be added to the target [curse resistance value] when hit - ヒットした時に、対象の【呪耐性値】に加算する数値
	pub curseAttackPower:i32,

	/// NAME: Fall damage ratio - 落下ダメージ倍率
	/// DESC: Multiply the damage calculation when falling - 落下時のダメージ計算に倍率をかける
	pub fallDamageRate:f32,

	/// NAME: Get Soul Magnification - 取得ソウル倍率
	/// DESC: The amount of soul acquired when defeating an enemy is added by the specified multiple. - 敵を倒した時の取得ソウル量が、指定倍数分上乗せされる
	pub soulRate:f32,

	/// NAME: Equipment weight change rate - 装備重量変化倍率
	/// DESC: Multiply the maximum equipment weight by the set magnification - 最大装備重量に、設定された倍率をかける
	pub equipWeightChangeRate:f32,

	/// NAME: Possession weight change rate - 所持重量変化倍率
	/// DESC: Multiply the maximum weight you have by the set magnification - 最大所持重量に、設定された倍率をかける
	pub allItemWeightChangeRate:f32,

	/// NAME: Soul addition - ソウル加算
	/// DESC: Add the set value to the possessed soul - 所持ソウルに、設定値を加算する
	pub soul:i32,

	/// NAME: Anime ID offset (invalid-1) - アニメIDオフセット(無効-1)
	/// DESC: Anime ID offset (invalid-1) - アニメIDオフセット(無効-1)
	pub animIdOffset:i32,

	/// NAME: Possession soul rate - 所持ソウル率
	/// DESC: For enemy lap effect. It is applied when the soul goes out from the set character. - 敵周回効果用。設定されているキャラから外にソウルが出て行く時に適用されます。
	pub haveSoulRate:f32,

	/// NAME: Target priority addition - ターゲット優先度加算分
	/// DESC: During multiplayer, the enemy will give priority to being targeted as a target. Addition of priority. 0 is the default. It will be often targeted with a positive value. Minus is up to -1. - マルチプレイ時、敵から優先的にターゲットとして狙われるようになる。プライオリティの加算。０がデフォルト。プラス値でよく狙われるようになる。マイナスは、－１まで。
	pub targetPriority:f32,

	/// NAME: Those who can see: Visual magnification - 見られる方：視覚倍率
	/// DESC: Correct the ease of finding by a magnification - 見つかりやすさを倍率で補正する
	pub sightSearchEnemyRate:f32,

	/// NAME: Who can hear: AI sound radius magnification - 聞かせる方：AI音半径倍率
	/// DESC: Correct the loudness of the AI sound emitted by the magnification - 発するAI音の大きさを倍率で補正する
	pub hearingSearchEnemyRate:f32,

	/// NAME: Gravity rate - グラビティ率
	/// DESC: Gravity rate - グラビティ率
	pub grabityRate:f32,

	/// NAME: Poison resistance change rate - 毒耐性変化倍率
	/// DESC: Multiply the poison resistance value by the set multiplier - 毒耐性値に設定された倍率をかける
	pub registPoizonChangeRate:f32,

	/// NAME: Epidemic resistance change rate - 疫病耐性変化倍率
	/// DESC: Multiply the plague resistance value by the set magnification - 疫病耐性値に設定された倍率をかける
	pub registDiseaseChangeRate:f32,

	/// NAME: Bleeding resistance change rate - 出血耐性変化倍率
	/// DESC: Multiply the bleeding resistance value by the set magnification - 出血耐性値に設定された倍率をかける
	pub registBloodChangeRate:f32,

	/// NAME: Curse resistance change rate - 呪耐性変化倍率
	/// DESC: Multiply the spell resistance value by the set multiplier - 呪耐性値に設定された倍率をかける
	pub registCurseChangeRate:f32,

	/// NAME: Soul Steel Coefficient - ソウルスティール係数
	/// DESC: Defense against HP robbed by NPCs in Soul Steel - NPCがソウルスティールで奪われるHPに対する防御力
	pub soulStealRate:f32,

	/// NAME: Defense: Life factor - 防御：寿命係数
	pub lifeReductionRate:f32,

	/// NAME: HP recovery coefficient - HP回復量係数
	/// DESC: It doesn't work when HP decreases. - HPが減るときは、効かない。
	pub hpRecoverRate:f32,

	/// NAME: Special effects to replace - 差し替える特殊効果
	/// DESC: Special effect ID added at the end of life (-1 is ignored) - 寿命が尽きた時に追加される特殊効果ID(-1は無視)
	pub replaceSpEffectId:i32,

	/// NAME: Periodic special effects - 周期発生特殊効果
	/// DESC: Special effect ID that occurs in each activation cycle (-1 is ignored) - 発動周期毎に発生する特殊効果ID(-1は無視)
	pub cycleOccurrenceSpEffectId:i32,

	/// NAME: Attack occurrence special effect - 攻撃発生特殊効果
	/// DESC: Special effect ID that occurs when hitting an attack (-1 is ignored) - 攻撃Hit時に発生する特殊効果ID(-1は無視)
	pub atkOccurrenceSpEffectId:i32,

	/// NAME: When guarding, the defense power up rate - ガード時はじき防御力アップ倍率
	/// DESC: Repellent defense correction value when guarding - ガード時のはじき防御力補正値
	pub guardDefFlickPowerRate:f32,

	/// NAME: Stamina cut magnification when guarding - ガード時スタミナカット倍率
	/// DESC: Stamina cut rate correction value when guarding - ガード時のスタミナカット率補正値
	pub guardStaminaCutRate:f32,

	/// NAME: Passing the line of sight: Activation time [ms] - 視線通過：発動時間[ms]
	/// DESC: Passing the line of sight: Activation time [ms] (for evil eye) - 視線通過：発動時間[ms]（邪眼用）
	pub rayCastPassedTime:i16,

	/// NAME: Vs to subcategory parameter change 1 - 対サブカテゴリパラメータ変化1
	/// DESC: Vs to subcategory parameter change 1 - 対サブカテゴリパラメータ変化1
	pub magicSubCategoryChange1:u8,

	/// NAME: Vs to subcategory parameter change 2 - 対サブカテゴリパラメータ変化2
	/// DESC: Vs to subcategory parameter change 2 - 対サブカテゴリパラメータ変化2
	pub magicSubCategoryChange2:u8,

	/// NAME: Bow distance correction [%] - 弓飛距離補正[％]
	/// DESC: Correction value added to the flight distance correction of the weapon - 武器の飛距離補正に加算される補正値
	pub bowDistRate:i16,

	/// NAME: Special effects category - 特殊効果カテゴリ
	/// DESC: Categories that determine behavior such as overwriting special effects - 特殊効果の上書きなどの挙動を決めるカテゴリ
	pub spCategory:u16,

	/// NAME: In-category priority - カテゴリ内優先度
	/// DESC: Priority within the same category (lower one has priority) - 同一カテゴリ内での優先度（低い方が優先）
	pub categoryPriority:u8,

	/// NAME: Save category - 保存カテゴリ
	/// DESC: Category to save special effects - 特殊効果を保存するカテゴリ
	pub saveCategory:i8,

	/// NAME: Magic registration frame change Magic slot - 魔法登録枠変化　魔法スロット
	/// DESC: You can increase the specified number of magic registration slots - 魔法登録枠を指定数増やすことが出来る
	pub changeMagicSlot:u8,

	/// NAME: Miracle registration frame change Miracle slot - 奇跡登録枠変化　奇跡スロット
	/// DESC: You can increase the specified number of trajectory registration frames. - 軌跡登録枠を指定数増やすことが出来る
	pub changeMiracleSlot:u8,

	/// NAME: Human damage value - 人間性ダメージ値
	/// DESC: Damage value given to human nature value - 人間性値に与えるダメージ値
	pub heroPointDamage:i8,

	/// NAME: Repellent Defense_Overwrite - はじき防御力_上書き
	/// DESC: Set a value that overwrites the repelling defense - はじき防御力を上書きする値を設定
	pub defFlickPower:u8,

	/// NAME: Damage attenuation rate when repelling [%] _ Overwrite - はじき時ダメージ減衰率[%]_上書き
	/// DESC: Set a value that overwrites the damage attenuation rate at the time of repelling - はじき時のダメージ減衰率を上書きする値を設定
	pub flickDamageCutRate:u8,

	/// NAME: Bleeding damage correction factor - 出血ダメージ補正倍率
	/// DESC: Point damage of state change type [bleeding], correction value used only when% damage - 状態変化タイプ[出血]のPointダメージ、％ダメージの時のみ使用される補正値
	pub bloodDamageRate:u8,

	/// NAME: DL_No damage (0) - DL_ダメージなし（0）
	/// DESC: Specify the type to replace the damage Lv0 - ダメージLv0を差し替えるタイプを指定
	pub dmgLv_None:i8,

	/// NAME: DL_Small (1) - DL_小（1）
	/// DESC: Specify the type to replace the damage Lv1 - ダメージLv1を差し替えるタイプを指定
	pub dmgLv_S:i8,

	/// NAME: DL_Medium (2) - DL_中（2）
	/// DESC: Specify the type to replace the damage Lv2 - ダメージLv2を差し替えるタイプを指定
	pub dmgLv_M:i8,

	/// NAME: DL_Large (3) - DL_大（3）
	/// DESC: Specify the type to replace the damage Lv3 - ダメージLv3を差し替えるタイプを指定
	pub dmgLv_L:i8,

	/// NAME: DL_ Blow-off (4) - DL_吹っ飛び（4）
	/// DESC: Specify the type to replace the damage Lv4 - ダメージLv4を差し替えるタイプを指定
	pub dmgLv_BlowM:i8,

	/// NAME: DL_push (5) - DL_プッシュ（5）
	/// DESC: Specify the type to replace the damage Lv5 - ダメージLv5を差し替えるタイプを指定
	pub dmgLv_Push:i8,

	/// NAME: DL_Slamming (6) - DL_叩きつけ（6）
	/// DESC: Specify the type to replace the damage Lv6 - ダメージLv6を差し替えるタイプを指定
	pub dmgLv_Strike:i8,

	/// NAME: DL_Small blow (7) - DL_小吹っ飛び（7）
	/// DESC: Specify the type to replace the damage Lv7 - ダメージLv7を差し替えるタイプを指定
	pub dmgLv_BlowS:i8,

	/// NAME: DL_minimal (8) - DL_極小（8）
	/// DESC: Specify the type to replace the damage Lv8 - ダメージLv8を差し替えるタイプを指定
	pub dmgLv_Min:i8,

	/// NAME: DL_ Launch (9) - DL_打ち上げ（9）
	/// DESC: Specify the type to replace the damage Lv9 - ダメージLv9を差し替えるタイプを指定
	pub dmgLv_Uppercut:i8,

	/// NAME: DL_ Oversized Blowout (10) - DL_特大吹っ飛び(10)
	/// DESC: Specify the type to replace the damage Lv10 - ダメージLv10を差し替えるタイプを指定
	pub dmgLv_BlowLL:i8,

	/// NAME: DL_Breath (11) - DL_ブレス(11)
	/// DESC: Specify the type to replace the damage Lv11 - ダメージLv11を差し替えるタイプを指定
	pub dmgLv_Breath:i8,

	/// NAME: Physical attributes - 物理属性
	/// DESC: Physical attributes to set for special effects - 特殊効果に設定する物理属性
	pub atkAttribute:u8,

	/// NAME: Special attributes - 特殊属性
	/// DESC: Special attributes to set for special effects - 特殊効果に設定する特殊属性
	pub spAttribute:u8,

	/// NAME: State change type - 状態変化タイプ
	/// DESC: State change judgment flag - 状態変化の判定フラグ
	pub stateInfo:u16,

	/// NAME: Weapon parameter change - 対武器パラメータ変化
	/// DESC: Specify which weapon is effective. If there is no limit, all attacks and defenses including enemies are targeted - どの武器に対して効果を発揮するかを指定する。制限無しの場合は敵も含めた全ての攻撃・防御が対象
	pub wepParamChange:u8,

	/// NAME: Movement type - 移動タイプ
	/// DESC: Movement type. Change the movement speed. - 移動タイプ。移動速度を変更する。
	pub moveType:u8,

	/// NAME: Defense: Life reduction type - 防御：寿命減少タイプ
	pub lifeReductionType:u16,

	/// NAME: Throwing conditions - 投げ条件
	/// DESC: Throwing conditions. Affects the throwing mask. - 投げ条件。投げマスクに影響する。
	pub throwCondition:u8,

	/// NAME: Condition value to be added to the action judgment ID - 行動判定IDに加算する条件値
	/// DESC: Condition value to add a value to the action judgment ID (Def: -1) - 行動判定ＩＤに値を加算する条件値(Def:-1)
	pub addBehaviorJudgeId_condition:i8,

	/// NAME: Cold damage correction factor - 冷気ダメージ補正倍率
	/// DESC: Correction value used only for point damage and% damage of state change type [cold air] - 状態変化タイプ[冷気]のPointダメージ、％ダメージの時のみ使用される補正値
	pub freezeDamageRate:u8,

	/// NAME: Effect target: Affiliation - 効果対象：所属　自分
	/// DESC: Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	pub Bitfield1:u8,

	/// NAME: Madness invalid - 発狂無効
	/// DESC: With this effect, you won't go mad - この効果がかかっていると発狂にかからなくなる
	pub Bitfield2:u8,

	/// NAME: Anti-miracle parameter change - 対奇跡パラメータ変化
	/// DESC: Set whether or not it is effective against miracles - 奇跡に対して効果を発揮するかしないかを設定する
	pub Bitfield3:u8,

	/// NAME: Undecidable judgment flag - 成仏不可　判定フラグ
	/// DESC: Whether you can be corpse. With this check, you will not be dead - 死体状態になれるかどうか。このチェックが付いていると、死亡状態にならない
	pub Bitfield4:u8,

	/// NAME: False target invalid_beast system - 偽ターゲット無効_獣系
	/// DESC: You will not be caught by the fake target of the beast system that occurred - 発生した獣系の偽ターゲットに引っかからなくなる
	pub Bitfield5:u8,

	/// NAME: Do you want to correct your strength? - 筋力補正するか？
	/// DESC: Do you want to correct your strength? - 筋力補正するか？
	pub Bitfield6:u8,

	/// NAME: Pledge 0 - 誓約0
	/// DESC: Pledge 0 - 誓約0
	pub Bitfield7:u8,

	/// NAME: Pledge 8 - 誓約8
	/// DESC: Pledge 8 - 誓約8
	pub Bitfield8:u8,

	/// NAME: Attack side damage level replacement - 攻撃側ダメージレベル差し替え
	/// DESC: The damage level of the attacking side changes to this value - 攻撃側のダメージレベルがこの値に指し換わる
	pub repAtkDmgLv:i8,

	/// NAME: Viewer: Visual magnification - 見る方：視覚倍率
	/// DESC: Correct the ease of finding with a magnification - 見つけやすさを倍率で補正する
	pub sightSearchRate:f32,

	/// NAME: Effect target: ● Hostile - 効果対象：●敵対
	/// DESC: Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	pub Bitfield9:u8,

	/// NAME: Team type change - チームタイプ変更
	/// DESC: Overwrites the specified team type - 指定したチームタイプに上書きする
	pub changeTeamType:i8,

	/// NAME: Damipoli ID - ダミポリID
	/// DESC: Damipoli ID. Damipoli ID range is 0 to 999.1000, 10000 is the category number. - ダミポリID。ダミポリID範囲は0～999.1000,10000の位はカテゴリ番号.
	pub dmypolyId:i16,

	/// NAME: Special effect VfxId_0 - 特殊効果VfxId_０
	/// DESC: Special effect VfxId (-1 disabled) - 特殊効果VfxId(-1無効)
	pub vfxId:i32,

	/// NAME: Special effect Id activated at the upper limit of the spirit ball - 元気玉上限時発動特殊効果Id
	/// DESC: Special effect Id activated at the upper limit of the spirit ball - 元気玉上限時発動特殊効果Id
	pub accumuOverFireId:i32,

	/// NAME: Genkitama upper limit - 元気玉上限値
	/// DESC: Genkitama upper limit - 元気玉上限値
	pub accumuOverVal:i32,

	/// NAME: Special effect Id activated at the lower limit of the spirit ball - 元気玉下限時発動特殊効果Id
	/// DESC: Special effect Id activated at the lower limit of the spirit ball - 元気玉下限時発動特殊効果Id
	pub accumuUnderFireId:i32,

	/// NAME: Genkitama lower limit - 元気玉下限値
	/// DESC: Genkitama lower limit - 元気玉下限値
	pub accumuUnderVal:i32,

	/// NAME: Genkitama accumulation value - 元気玉蓄積値
	/// DESC: Genkitama accumulation value - 元気玉蓄積値
	pub accumuVal:i32,

	/// NAME: How to see: Overwrite visual angle (height) [deg] - 見る方：視覚角度（高さ）上書き[deg]
	/// DESC: Override the findability angle - 見つけやすさの角度を上書きする
	pub eye_angX:u8,

	/// NAME: How to see: Overwrite visual angle (width) [deg] - 見る方：視覚角度（幅）上書き[deg]
	/// DESC: Override the findability angle - 見つけやすさの角度を上書きする
	pub eye_angY:u8,

	/// NAME: Change the degree of death - 亡者度 変更
	/// DESC: Add this value to the degree of death - この値分亡者度を加算する
	pub addDeceasedLv:i16,

	/// NAME: Special effect VfxId_1 - 特殊効果VfxId_１
	/// DESC: Special effect VfxId1 (-1 invalid) - 特殊効果VfxId１(-1無効)
	pub vfxId1:i32,

	/// NAME: Special effect VfxId_2 - 特殊効果VfxId_２
	/// DESC: Special effect VfxId2 (-1 invalid) - 特殊効果VfxId２(-1無効)
	pub vfxId2:i32,

	/// NAME: Special effect VfxId_3 - 特殊効果VfxId_３
	/// DESC: Special effect VfxId3 (-1 invalid) - 特殊効果VfxId３(-1無効)
	pub vfxId3:i32,

	/// NAME: Special effect VfxId_4 - 特殊効果VfxId_４
	/// DESC: Special effect VfxId4 (-1 invalid) - 特殊効果VfxId４(-1無効)
	pub vfxId4:i32,

	/// NAME: Special effect VfxId_5 - 特殊効果VfxId_５
	/// DESC: Special effect VfxId5 (-1 invalid) - 特殊効果VfxId５(-1無効)
	pub vfxId5:i32,

	/// NAME: Special effect VfxId_6 - 特殊効果VfxId_６
	/// DESC: Special effect VfxId6 (-1 invalid) - 特殊効果VfxId６(-1無効)
	pub vfxId6:i32,

	/// NAME: Special effect VfxId_7 - 特殊効果VfxId_７
	/// DESC: Special effect VfxId7 (-1 invalid) - 特殊効果VfxId７(-1無効)
	pub vfxId7:i32,

	/// NAME: Cold resistance attack power [point] - 冷気耐性攻撃力[point]
	/// DESC: Numerical value to be added to the target [Cold air resistance value] when hit - ヒットした時に、対象の【冷気耐性値】に加算する数値
	pub freezeAttackPower:i32,

	/// NAME: Generated AI sound ID - 発生AI音ID
	/// DESC: Generates AI sound parameters with set values - 設定された値のAI音パラメータを発生させる
	pub AppearAiSoundId:i32,

	/// NAME: Additional foot effect identifier - 追加フットエフェクト識別子
	/// DESC: The identifier of the foot effect that is additionally generated during special effects. XYYZZZ ZZZ - 特殊効果時に追加で発生させるフットエフェクトの識別子。XYYZZZのZZZ
	pub addFootEffectSfxId:i16,

	/// NAME: Virtual status for skill cancellation - 技量キャンセル用仮想ステータス
	/// DESC: Add this value when calculating the end timing of the TAE flag of "Skill Cancel". - 「技量キャンセル」のTAEフラグの終了タイミングを計算する時に、この値を追加して計算する
	pub dexterityCancelSystemOnlyAddDexterity:i8,

	/// NAME: Team attack influence_overwrite - チーム攻撃影響力_上書き
	/// DESC: Overwrite and change the target [Team Attack Influence] value. Do not change the default value (-1). - 対象の【チーム攻撃影響力】の値を、上書きして変更する。デフォルト値（-1）のときは変更しない。
	pub teamOffenseEffectivity:i8,

	/// NAME: Toughness Damage multiplier - 強靭度 被ダメージ倍率
	/// DESC: Toughness version cut rate - 強靭度版カット率
	pub toughnessDamageCutRate:f32,

	/// NAME: Special attack A damage multiplier correction - 特攻Aダメージ倍率補正
	/// DESC: Special Attack A Damage multiplier is corrected. 1 is normal. - 特攻Aダメージ倍率に補正をかけます。１が通常。
	pub weakDmgRateA:f32,

	/// NAME: Special attack B damage multiplier correction - 特攻Bダメージ倍率補正
	/// DESC: Special attack B Damage multiplier is corrected. 1 is normal. - 特攻Bダメージ倍率に補正をかけます。１が通常。
	pub weakDmgRateB:f32,

	/// NAME: Special attack C damage multiplier correction - 特攻Cダメージ倍率補正
	/// DESC: Special attack C Damage multiplier is corrected. 1 is normal. - 特攻Cダメージ倍率に補正をかけます。１が通常。
	pub weakDmgRateC:f32,

	/// NAME: Special attack D damage multiplier correction - 特攻Dダメージ倍率補正
	/// DESC: Special attack D Damage multiplier is corrected. 1 is normal. - 特攻Dダメージ倍率に補正をかけます。１が通常。
	pub weakDmgRateD:f32,

	/// NAME: Special attack E damage multiplier correction - 特攻Eダメージ倍率補正
	/// DESC: Special attack E Damage multiplier is corrected. 1 is normal. - 特攻Eダメージ倍率に補正をかけます。１が通常。
	pub weakDmgRateE:f32,

	/// NAME: Special attack F damage multiplier correction - 特攻Fダメージ倍率補正
	/// DESC: Special attack F Damage multiplier is corrected. 1 is normal. - 特攻Fダメージ倍率に補正をかけます。１が通常。
	pub weakDmgRateF:f32,

	/// NAME: Defender: Dark damage multiplier - 防御側：闇ダメージ倍率
	/// DESC: Dark damage multiplier: Correct the calculated damage by XX times. 1 is normal. - 闇ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub darkDamageCutRate:f32,

	/// NAME: Dark defense multiplier - 闇防御力倍率
	/// DESC: Multiply the darkness defense by the set value - 闇防御力に設定した数値をかけます
	pub darkDiffenceRate:f32,

	/// NAME: Dark defense [point] - 闇防御力[point]
	/// DESC: Add or subtract the value set for darkness defense - 闇防御力に設定した数値を加減算する
	pub darkDiffence:i32,

	/// NAME: Attacker: Dark damage multiplier - 攻撃側：闇ダメージ倍率
	/// DESC: Dark damage multiplier: Correct the calculated damage by XX times. 1 is normal. - 闇ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub darkAttackRate:f32,

	/// NAME: Dark attack power multiplier - 闇攻撃力倍率
	/// DESC: Multiply the dark attack power by the set value - 闇攻撃力に設定した数値をかけます
	pub darkAttackPowerRate:f32,

	/// NAME: Dark attack power [point] - 闇攻撃力[point]
	/// DESC: Add or subtract the value set for the dark attack power - 闇攻撃力に設定した数値を加減算する
	pub darkAttackPower:i32,

	/// NAME: Radius of full view of darkness [m] - 暗闇丸見え半径[m]
	/// DESC: Radius of full view of darkness [m]. If you are within this distance, you will be able to see at normal distance even in the dark. - 暗闇丸見え半径[m]。この距離内にいる場合は暗所でも通常距離で見えるようになります
	pub antiDarkSightRadius:f32,

	/// NAME: Damipoli ID with full view of darkness - 暗闇丸見えダミポリID
	/// DESC: Damipoli ID (-1: Master) with full view of darkness. Create a full view area around this Damipoli - 暗闇丸見えダミポリID(-1:マスター)。このダミポリを中心に丸見え領域を作成します
	pub antiDarkSightDmypolyId:i32,

	/// NAME: Trigger condition: Remaining HP ratio is above a certain level [%] - 発動条件　残りHP比率が一定以上[%]
	/// DESC: Activates only when you have HP above the specified value - 指定された値以上のHPを持っている時にしか発動しない
	pub conditionHpRate:f32,

	/// NAME: Consumption stamina magnification - 消費スタミナ倍率
	/// DESC: Multiply by multiplying the consumption stamina value of the behavior parameter - 行動パラメータの消費スタミナの値にかける倍率
	pub consumeStaminaRate:f32,

	/// NAME: Item drop correction - アイテムドロップ補正
	/// DESC: The set value is added to [Item Drop Correction] - 設定された値が【アイテムドロップ補正】に加算される 
	pub itemDropRate:f32,

	/// NAME: Poison resistance change [point] - 毒耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changePoisonResistPoint:i32,

	/// NAME: Epidemic resistance change [point] - 疫病耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changeDiseaseResistPoint:i32,

	/// NAME: Bleeding resistance change [point] - 出血耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changeBloodResistPoint:i32,

	/// NAME: Curse resistance change [point] - 呪耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changeCurseResistPoint:i32,

	/// NAME: Change in cold tolerance [point] - 冷気耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changeFreezeResistPoint:i32,

	/// NAME: Attacker: Slash damage multiplier - 攻撃側：斬撃ダメージ倍率
	/// DESC: Slash damage ratio: Correct the calculated damage by XX times. 1 is normal. - 斬撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub slashAttackRate:f32,

	/// NAME: Attacker: Batter damage multiplier - 攻撃側：打撃ダメージ倍率
	/// DESC: Batter damage ratio: The calculated damage is corrected by XX times. 1 is normal. - 打撃ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub blowAttackRate:f32,

	/// NAME: Attack side: piercing damage multiplier - 攻撃側：刺突ダメージ倍率
	/// DESC: Puncture damage ratio: Correct the calculated damage by XX times. 1 is normal. - 刺突ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub thrustAttackRate:f32,

	/// NAME: Attacker: Non-attribute damage multiplier - 攻撃側：無属性ダメージ倍率
	/// DESC: Non-attribute damage ratio: Correct the calculated damage by XX times. 1 is normal. - 無属性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub neutralAttackRate:f32,

	/// NAME: Slash attack power multiplier - 斬撃攻撃力倍率
	/// DESC: Multiply the slashing attack power by the set value - 斬撃攻撃力に設定した数値をかけます
	pub slashAttackPowerRate:f32,

	/// NAME: Batter attack power multiplier - 打撃攻撃力倍率
	/// DESC: Multiply the hit attack power by the set value - 打撃攻撃力に設定した数値をかけます
	pub blowAttackPowerRate:f32,

	/// NAME: Puncture attack power multiplier - 刺突攻撃力倍率
	/// DESC: Multiply the piercing attack power by the set value - 刺突攻撃力に設定した数値をかけます
	pub thrustAttackPowerRate:f32,

	/// NAME: Non-attribute attack power multiplier - 無属性攻撃力倍率
	/// DESC: Multiply the non-attribute attack power by the set value - 無属性攻撃力に設定した数値をかけます
	pub neutralAttackPowerRate:f32,

	/// NAME: Slash attack power [point] - 斬撃攻撃力[point]
	/// DESC: Add or subtract the value set for the slashing attack power - 斬撃攻撃力に設定した数値を加減算する
	pub slashAttackPower:i32,

	/// NAME: Batter attack power [point] - 打撃攻撃力[point]
	/// DESC: Add or subtract the value set for the batter attack power - 打撃攻撃力に設定した数値を加減算する
	pub blowAttackPower:i32,

	/// NAME: Puncture attack power [point] - 刺突攻撃力[point]
	/// DESC: Add or subtract the value set for the piercing attack power - 刺突攻撃力に設定した数値を加減算する
	pub thrustAttackPower:i32,

	/// NAME: Non-attribute attack power [point] - 無属性攻撃力[point]
	/// DESC: Add or subtract the value set for non-attribute attack power - 無属性攻撃力に設定した数値を加減算する
	pub neutralAttackPower:i32,

	/// NAME: Strength correction change [point] - 筋力補正変化[point]
	/// DESC: Add or subtract the correction value of the weapon - 武器の補正値を加減算する
	pub changeStrengthPoint:i32,

	/// NAME: Agility correction change [point] - 俊敏補正変化[point]
	/// DESC: Add or subtract the correction value of the weapon - 武器の補正値を加減算する
	pub changeAgilityPoint:i32,

	/// NAME: Magic correction change [point] - 魔力補正変化[point]
	/// DESC: Add or subtract the correction value of the weapon - 武器の補正値を加減算する
	pub changeMagicPoint:i32,

	/// NAME: Faith correction change [point] - 信仰補正変化[point]
	/// DESC: Add or subtract the correction value of the weapon - 武器の補正値を加減算する
	pub changeFaithPoint:i32,

	/// NAME: Luck correction change [point] - 運補正変化[point]
	/// DESC: Add or subtract the correction value of the weapon - 武器の補正値を加減算する
	pub changeLuckPoint:i32,

	/// NAME: Arts point recovery Strength system - アーツポイント回復 筋力系
	/// DESC: Arts Point Restores strength - アーツポイント筋力を回復させる
	pub recoverArtsPoint_Str:i8,

	/// NAME: Arts point recovery workmanship system - アーツポイント回復 技量系
	/// DESC: Restores arts point workmanship - アーツポイント技量を回復させる
	pub recoverArtsPoint_Dex:i8,

	/// NAME: Arts point recovery magic system - アーツポイント回復 魔法系
	/// DESC: Restores arts point magic - アーツポイント魔法を回復させる
	pub recoverArtsPoint_Magic:i8,

	/// NAME: Arts point recovery Miracle system - アーツポイント回復 奇跡系
	/// DESC: Recover arts point miracles - アーツポイント奇跡を回復させる
	pub recoverArtsPoint_Miracle:i8,

	/// NAME: Mad damage correction factor - 発狂ダメージ補正倍率
	/// DESC: Correction value used only for point damage and% damage of state change type [madness] - 状態変化タイプ[発狂]のPointダメージ、％ダメージの時のみ使用される補正値
	pub madnessDamageRate:u8,

	/// NAME: Whether to apply the abnormal state attack power multiplier correction - 状態異常攻撃力倍率補正を適応するか
	/// DESC: If ○, the abnormal state attack power multiplier correction of the attack para is applied. - ○なら攻撃パラの状態異常攻撃力倍率補正を適応します。
	pub Bitfield10:u8,

	/// NAME: Addition value to be added to the action judgment ID - 行動判定IDに加算する加算値
	/// DESC: If the addition value of the action judgment ID is 0, the action is not switched and the action is stopped. - 行動判定IDの加算値 ０の場合は行動を切り替えるのではなく、行動しなくなります。
	pub addBehaviorJudgeId_add:u16,

	/// NAME: SA value_damage ratio - SA値_被ダメージ倍率
	/// DESC: Multiplier for SA damage - SAダメージかかる倍率
	pub saReceiveDamageRate:f32,

	/// NAME: Defending player Physical damage compensation factor - 防御側 プレイヤー 物理ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the player. - プレイヤーから受けるダメージ値に対するダメージ補正。
	pub defPlayerDmgCorrectRate_Physics:f32,

	/// NAME: Defending player magic damage correction multiplier - 防御側 プレイヤー 魔法ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the player. - プレイヤーから受けるダメージ値に対するダメージ補正。
	pub defPlayerDmgCorrectRate_Magic:f32,

	/// NAME: Defending player Fire damage compensation factor - 防御側 プレイヤー 炎ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the player. - プレイヤーから受けるダメージ値に対するダメージ補正。
	pub defPlayerDmgCorrectRate_Fire:f32,

	/// NAME: Defending player Lightning damage compensation factor - 防御側 プレイヤー 雷ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the player. - プレイヤーから受けるダメージ値に対するダメージ補正。
	pub defPlayerDmgCorrectRate_Thunder:f32,

	/// NAME: Defending player Dark damage correction factor - 防御側 プレイヤー 闇ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the player. - プレイヤーから受けるダメージ値に対するダメージ補正。
	pub defPlayerDmgCorrectRate_Dark:f32,

	/// NAME: Defender Enemy Physical Damage Compensation Magnification - 防御側 敵 物理ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the enemy. - 敵から受けるダメージ値に対するダメージ補正。
	pub defEnemyDmgCorrectRate_Physics:f32,

	/// NAME: Defender enemy magic damage correction factor - 防御側 敵 魔法ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the enemy. - 敵から受けるダメージ値に対するダメージ補正。
	pub defEnemyDmgCorrectRate_Magic:f32,

	/// NAME: Defender enemy flame damage correction factor - 防御側 敵 炎ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the enemy. - 敵から受けるダメージ値に対するダメージ補正。
	pub defEnemyDmgCorrectRate_Fire:f32,

	/// NAME: Defender enemy lightning damage correction factor - 防御側 敵 雷ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the enemy. - 敵から受けるダメージ値に対するダメージ補正。
	pub defEnemyDmgCorrectRate_Thunder:f32,

	/// NAME: Defender enemy darkness damage correction factor - 防御側 敵 闇ダメージ補正倍率
	/// DESC: Damage correction for the damage value received from the enemy. - 敵から受けるダメージ値に対するダメージ補正。
	pub defEnemyDmgCorrectRate_Dark:f32,

	/// NAME: Defender Object Damage Compensation Magnification - 防御側 オブジェクトダメージ補正倍率
	/// DESC: Damage correction for the damage value received from OBJ. - OBJから受けるダメージ値に対するダメージ補正。
	pub defObjDmgCorrectRate:f32,

	/// NAME: Attacking player Physical damage correction factor - 攻撃側 プレイヤー 物理ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the player. - プレイヤーに与えるダメージ値に対するダメージ補正。
	pub atkPlayerDmgCorrectRate_Physics:f32,

	/// NAME: Attacking player Magic damage compensation factor - 攻撃側 プレイヤー 魔法ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the player. - プレイヤーに与えるダメージ値に対するダメージ補正。
	pub atkPlayerDmgCorrectRate_Magic:f32,

	/// NAME: Attacker Player Flame Damage Compensation Multiplier - 攻撃側 プレイヤー 炎ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the player. - プレイヤーに与えるダメージ値に対するダメージ補正。
	pub atkPlayerDmgCorrectRate_Fire:f32,

	/// NAME: Attacking player Lightning damage compensation factor - 攻撃側 プレイヤー 雷ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the player. - プレイヤーに与えるダメージ値に対するダメージ補正。
	pub atkPlayerDmgCorrectRate_Thunder:f32,

	/// NAME: Attacking player Dark damage correction factor - 攻撃側 プレイヤー 闇ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the player. - プレイヤーに与えるダメージ値に対するダメージ補正。
	pub atkPlayerDmgCorrectRate_Dark:f32,

	/// NAME: Attacker Enemy Physical Damage Compensation Magnification - 攻撃側 敵 物理ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the enemy. - 敵に与えるダメージ値に対するダメージ補正。
	pub atkEnemyDmgCorrectRate_Physics:f32,

	/// NAME: Attacking side Enemy magic damage correction factor - 攻撃側 敵 魔法ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the enemy. - 敵に与えるダメージ値に対するダメージ補正。
	pub atkEnemyDmgCorrectRate_Magic:f32,

	/// NAME: Attacking side Enemy flame damage correction factor - 攻撃側 敵 炎ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the enemy. - 敵に与えるダメージ値に対するダメージ補正。
	pub atkEnemyDmgCorrectRate_Fire:f32,

	/// NAME: Attacking side Enemy lightning damage correction factor - 攻撃側 敵 雷ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the enemy. - 敵に与えるダメージ値に対するダメージ補正。
	pub atkEnemyDmgCorrectRate_Thunder:f32,

	/// NAME: Attacking side Enemy darkness damage correction factor - 攻撃側 敵 闇ダメージ補正倍率
	/// DESC: Damage correction for the damage value given to the enemy. - 敵に与えるダメージ値に対するダメージ補正。
	pub atkEnemyDmgCorrectRate_Dark:f32,

	/// NAME: Cold resistance change rate - 冷気耐性変化倍率
	/// DESC: Multiply the cold resistance value by the set magnification - 冷気耐性値に設定された倍率をかける
	pub registFreezeChangeRate:f32,

	/// NAME: Trigger condition state change type 1 - 発動条件状態変化タイプ1
	/// DESC: Trigger condition state change type 1 - 発動条件状態変化タイプ1
	pub invocationConditionsStateChange1:u16,

	/// NAME: Trigger condition state change type 2 - 発動条件状態変化タイプ2
	/// DESC: Trigger condition state change type 2 - 発動条件状態変化タイプ2
	pub invocationConditionsStateChange2:u16,

	/// NAME: Trigger condition state change type 3 - 発動条件状態変化タイプ3
	/// DESC: Trigger condition state change type 3 - 発動条件状態変化タイプ3
	pub invocationConditionsStateChange3:u16,

	/// NAME: Listener: Overwrite audible AI sound level - 聞く方：可聴AI音レベル上書き
	/// DESC: Overwrite how good your ears are - どれくらい耳が良いのかを上書きする
	pub hearingAiSoundLevel:i16,

	/// NAME: Capsule size magnification - カプセルサイズ倍率
	/// DESC: Magnification over the height of the character capsule - キャラカプセルの高さに掛かる倍率
	pub chrProxyHeightRate:f32,

	/// NAME: Search side addition correction_viewer - 索敵度加算補正_見る側
	/// DESC: Search side addition correction_viewer - 索敵度加算補正_見る側
	pub addAwarePointCorrectValue_forMe:f32,

	/// NAME: Searching degree addition correction _ side to be seen - 索敵度加算補正_見られる側
	/// DESC: Searching degree addition correction _ side to be seen - 索敵度加算補正_見られる側
	pub addAwarePointCorrectValue_forTarget:f32,

	/// NAME: Those who can see: Visual addition - 見られる方：視覚加算
	/// DESC: Correct the ease of finding with a real number - 見つかりやすさを実数で補正する
	pub sightSearchEnemyAdd:f32,

	/// NAME: How to see: Visual addition - 見る方：視覚加算
	/// DESC: Correct the ease of finding with a real number - 見つけやすさを実数で補正する
	pub sightSearchAdd:f32,

	/// NAME: Listener: AI sound radius addition - 聞く方：AI音半径加算
	/// DESC: Correct the hearing of AI sounds with real numbers - AI音の聞こえ具合を実数で補正する
	pub hearingSearchAdd:f32,

	/// NAME: Listener: AI sound radius magnification - 聞く方：AI音半径倍率
	/// DESC: Correct the audibility of AI sound by magnification - AI音の聞こえ具合を倍率で補正する
	pub hearingSearchRate:f32,

	/// NAME: Those who can hear: AI sound radius addition - 聞かせる方：AI音半径加算
	/// DESC: Correct the loudness of the emitted AI sound with a real number - 発するAI音の大きさを実数で補正する
	pub hearingSearchEnemyAdd:f32,

	/// NAME: Sales price correction: Magnification - 販売価格補正：倍率
	/// DESC: Sales price correction: Magnification - 販売価格補正：倍率
	pub value_Magnification:f32,

	/// NAME: Arts MP consumption magnification - アーツ消費MP倍率
	/// DESC: Arts MP consumption multiplier [%] - アーツ消費MP倍率[%]
	pub artsConsumptionRate:f32,

	/// NAME: Magic consumption MP magnification - 魔法消費MP倍率
	/// DESC: Magic consumption MP multiplier [%] - 魔法消費MP倍率[%]
	pub magicConsumptionRate:f32,

	/// NAME: Magic consumption MP multiplier - 呪術消費MP倍率
	/// DESC: Magic consumption MP multiplier [%] - 呪術消費MP倍率[%]
	pub shamanConsumptionRate:f32,

	/// NAME: Miracle consumption MP magnification - 奇跡消費MP倍率
	/// DESC: Miracle consumption MP magnification [%] - 奇跡消費MP倍率[%]
	pub miracleConsumptionRate:f32,

	/// NAME: Est bottle HP damage amount [%] - エスト瓶HPダメージ量[%]
	/// DESC: Set what percentage of the maximum HP to add (or subtract) with one activation - 一度の発動で最大HPの何%分を加算（または減算）するかを設定
	pub changeHpEstusFlaskRate:i32,

	/// NAME: Est bottle HP damage amount [point] - エスト瓶HPダメージ量[point]
	/// DESC: Set how many points to add (or subtract) with one activation - 一度の発動で何ポイント加算（または減算）するかを設定
	pub changeHpEstusFlaskPoint:i32,

	/// NAME: Est Bottle MP Damage Amount [%] - エスト瓶MPダメージ量[%] 
	/// DESC: Set what percentage of the maximum MP to add (or subtract) with one activation - 一度の発動で最大MPの何%分を加算（または減算）するかを設定
	pub changeMpEstusFlaskRate:i32,

	/// NAME: Est Bottle MP Damage Amount [point] - エスト瓶MPダメージ量[point] 
	/// DESC: Set how many points to add (or subtract) with one activation - 一度の発動で何ポイント加算（または減算）するかを設定
	pub changeMpEstusFlaskPoint:i32,

	/// NAME: Est bottle HP damage multiplier - エスト瓶HPダメージ倍率 
	/// DESC: Correct the damage amount of the HP Est bottle - HPエスト瓶のダメージ量に対して補正をかける
	pub changeHpEstusFlaskCorrectRate:f32,

	/// NAME: Est bottle MP damage multiplier - エスト瓶MPダメージ倍率 
	/// DESC: Correct the damage amount of MP Est Bottle - MPエスト瓶のダメージ量に対して補正をかける
	pub changeMpEstusFlaskCorrectRate:f32,

	/// NAME: HP drain activation special effect - HPドレイン発動特殊効果
	/// DESC: When the special effect of the state change type "HP drain" is enabled, when the enemy is defeated, the special effect ID set in the "HP drain activation special effect" of the same special effect is called (0: ignore). - 状態変化タイプ「HPドレイン」の特殊効果が有効の時に、敵を倒した際に同じ特殊効果の「HPドレイン発動特殊効果」に設定されている特殊効果IDを呼び出す(0：無視)
	pub applyIdOnGetSoul:i32,

	/// NAME: Life extension factor - 寿命延長倍率
	/// DESC: Extension coefficient of state change type "life extension" - 状態変化タイプ「寿命延長」の延長係数
	pub extendLifeRate:f32,

	/// NAME: Life shortening ratio - 寿命短縮倍率
	/// DESC: Shortening coefficient of state change type "life shortening" - 状態変化タイプ「寿命短縮」の短縮係数
	pub contractLifeRate:f32,

	/// NAME: Damaged object attack power multiplier - 被ダメージ オブジェクト攻撃力倍率
	/// DESC: Corrects the attack power against the damage received from OBJ. (Not damage compensation) - OBJから受けるダメージに対して攻撃力を補正する。（ダメージ補正ではない）
	pub defObjectAttackPowerRate:f32,

	/// NAME: Group ID that deletes the character's paint decal when the special effect disappears - 特殊効果消失時にキャラのペイントデカールを削除するグループID
	/// DESC: When the special effect disappears (lifetime / overwritten / erased ... etc.), the paint decal is deleted if the special effect of the same group ID is not applied. - 特殊効果が消失した時（寿命/何かに上書きされる/消される…など）に、同じグループIDの特殊効果がかかっていなければペイントデカールを削除する。
	pub effectEndDeleteDecalGroupId:i16,

	/// NAME: Vitality additional value - 生命力追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addLifeForceStatus:i8,

	/// NAME: Mental strength additional value - 精神力追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addWillpowerStatus:i8,

	/// NAME: Endurance additional value - 持久力追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addEndureStatus:i8,

	/// NAME: Additional physical strength - 体力追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addVitalityStatus:i8,

	/// NAME: Strength addition value - 筋力追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addStrengthStatus:i8,

	/// NAME: Workmanship addition value - 技量追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addDexterityStatus:i8,

	/// NAME: Force addition value - 理力追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addMagicStatus:i8,

	/// NAME: Faith additional value - 信仰追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addFaithStatus:i8,

	/// NAME: Luck addition value - 運追加値
	/// DESC: Add value to growth status - 成長ステータスに値を加える
	pub addLuckStatus:i8,

	/// NAME: Deletion condition damage - 削除条件ダメージ
	/// DESC: Reason for damage under the condition to remove special effects - 特殊効果を削除する条件のダメージ理由
	pub deleteCriteriaDamage:u8,

	/// NAME: Vs to subcategory parameter change 3 - 対サブカテゴリパラメータ変化3
	/// DESC: Vs to subcategory parameter change 3 - 対サブカテゴリパラメータ変化3
	pub magicSubCategoryChange3:u8,

	/// NAME: Special attribute variation value - 特殊属性バリエーション値
	/// DESC: This value is used to give variation to abnormal state SFX, SE, etc. in combination with the special attribute set for the special effect. SEQ16473 - 特殊効果に設定する特殊属性と組み合わせて状態異常SFX,SEなどにバリエーションを持たせるために使用する値です。SEQ16473
	pub spAttributeVariationValue:u8,

	/// NAME: Repelling attack power_overwrite - はじき攻撃力_上書き
	/// DESC: Set a value that overwrites the repelling attack power - はじき攻撃力を上書きする値を設定
	pub atkFlickPower:u8,

	/// NAME: Water level setting for wet conditions - 濡れる条件の水位設定
	/// DESC: TimeAct Determines whether special effects are applied in combination with "at what water level you get wet" - TimeAct「どの水位で濡れるか」と組み合わせて特殊効果に掛かるかどうかを判定する
	pub wetConditionDepth:u8,

	/// NAME: SA recovery speed change - SA回復速度変化
	/// DESC: Change the recovery speed of SA durability - SA耐久度の回復速度を変化させる
	pub changeSaRecoveryVelocity:f32,

	/// NAME: Regain magnification - リゲイン倍率
	/// DESC: Regain magnification - リゲイン倍率
	pub regainRate:f32,

	/// NAME: SA attack power multiplier - SA攻撃力倍率
	/// DESC: SA attack power multiplier - SA攻撃力倍率
	pub saAttackPowerRate:f32,

	/// NAME: Sleep tolerance attack power [point] - 睡眠耐性攻撃力[point]
	/// DESC: Numerical value to be added to the target's [Sleep tolerance] when hit - ヒットした時に、対象の【睡眠耐性値】に加算する数値
	pub sleepAttackPower:i32,

	/// NAME: Madness resistance attack power [point] - 発狂耐性攻撃力[point]
	/// DESC: A number to be added to the target's [madness resistance value] when it hits - ヒットした時に、対象の【発狂耐性値】に加算する数値
	pub madnessAttackPower:i32,

	/// NAME: Sleep tolerance change rate - 睡眠耐性変化倍率
	/// DESC: Multiply the sleep tolerance value by the set magnification - 睡眠耐性値に設定された倍率をかける
	pub registSleepChangeRate:f32,

	/// NAME: Madness resistance change rate - 発狂耐性変化倍率
	/// DESC: Multiply the madness resistance value by the set multiplier - 発狂耐性値に設定された倍率をかける
	pub registMadnessChangeRate:f32,

	/// NAME: Changes in sleep tolerance [point] - 睡眠耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changeSleepResistPoint:i32,

	/// NAME: Madness resistance change [point] - 発狂耐性変化[point]
	/// DESC: Increase or decrease the state resistance value - 状態耐性値を増減させる
	pub changeMadnessResistPoint:i32,

	/// NAME: Sleep damage correction factor - 睡眠ダメージ補正倍率
	/// DESC: Point damage of state change type [Sleep], correction value used only when% damage - 状態変化タイプ[睡眠]のPointダメージ、％ダメージの時のみ使用される補正値
	pub sleepDamageRate:u8,

	/// NAME: Changes in site parameters - 対部位パラメータ変化
	/// DESC: The effect is limited by the part where the attack hits. Only defensive items in damage calculation are subject to restriction - 攻撃がヒットした部位によって効果を制限する。ダメージ計算の防御系の項目のみ制限対象となる
	pub applyPartsGroup:u8,

	/// NAME: Clear the target - ターゲットクリア
	/// DESC: Does not recognize the target while the special effect is applied (excluding the riding target) - 特殊効果が掛かっている間ターゲットを認識しない（騎乗ターゲット除く
	pub Bitfield11:u8,

	/// NAME: pad - pad
	pub pad2:[u8;1],

	/// NAME: Maximum SA addition value [point] - 最大SA加算値[point]
	/// DESC: Value to add to the super armor value - スーパーアーマー値に加算する値
	pub changeSuperArmorPoint:f32,

	/// NAME: SA damage amount [point] - SAダメージ量[point]
	/// DESC: Set how many points to subtract (or add) with one activation - 一度の発動で何ポイント減算（または加算）するかを設定
	pub changeSaPoint:f32,

	/// NAME: Giant enemy lift height overwrite [m] - 巨大敵持ち上げ高さ上書き[m]
	/// DESC: Giant enemy lift height overwrite [m] - 巨大敵持ち上げ高さ上書き[m]
	pub hugeEnemyPickupHeightOverwrite:f32,

	/// NAME: Defender: Poison resistance damage multiplier - 防御側：毒耐性ダメージ倍率
	/// DESC: Poison resistance damage ratio: Correct the calculated damage by XX times. 1 is normal. - 毒耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub poisonDefDamageRate:f32,

	/// NAME: Defender: Epidemic resistance damage multiplier - 防御側：疫病耐性ダメージ倍率
	/// DESC: Epidemic resistance damage ratio: Correct the calculated damage by XX times. 1 is normal. - 疫病耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub diseaseDefDamageRate:f32,

	/// NAME: Defender: Bleeding resistance damage multiplier - 防御側：出血耐性ダメージ倍率
	/// DESC: Bleeding resistance damage ratio: Correct the calculated damage by XX times. 1 is normal. - 出血耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub bloodDefDamageRate:f32,

	/// NAME: Defender: Curse resistance damage multiplier - 防御側：呪耐性ダメージ倍率
	/// DESC: Curse resistance damage multiplier: Corrects the calculated damage by XX times. 1 is normal. - 呪耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub curseDefDamageRate:f32,

	/// NAME: Defender: Cold resistance damage multiplier - 防御側：冷気耐性ダメージ倍率
	/// DESC: Cold resistance damage ratio: Correct the calculated damage by XX times. 1 is normal. - 冷気耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub freezeDefDamageRate:f32,

	/// NAME: Defender: Sleep resistance damage multiplier - 防御側：睡眠耐性ダメージ倍率
	/// DESC: Sleep resistance damage ratio: Correct the calculated damage by XX times. 1 is normal. - 睡眠耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub sleepDefDamageRate:f32,

	/// NAME: Defender: Madness resistance damage multiplier - 防御側：発狂耐性ダメージ倍率
	/// DESC: Madness resistance damage multiplier: Correct the calculated damage by XX times. 1 is normal. - 発狂耐性ダメージ倍率：算出したダメージに×○倍で補正をかける。１が通常。
	pub madnessDefDamageRate:f32,

	/// NAME: Distance to go home no matter what [m] _ overwrite - 何があっても帰宅する距離[m]_上書き
	/// DESC: Distance to go home no matter what [m] _ overwrite - 何があっても帰宅する距離[m]_上書き
	pub overwrite_maxBackhomeDist:u16,

	/// NAME: Distance to return home while fighting [m] _ Overwrite - 戦闘しつつ帰宅する距離[m]_上書き
	/// DESC: Distance to return home while fighting [m] _ Overwrite - 戦闘しつつ帰宅する距離[m]_上書き
	pub overwrite_backhomeDist:u16,

	/// NAME: Distance to give up and fight to return to the nest [m] _ overwrite - 巣に帰るのをあきらめて戦闘する距離[m]_上書き
	/// DESC: Distance to give up and fight to return to the nest [m] _ overwrite - 巣に帰るのをあきらめて戦闘する距離[m]_上書き 
	pub overwrite_backhomeBattleDist:u16,

	/// NAME: When returning home: Distance to see the target [m] _ Overwrite - 帰宅時：ターゲットを見ている距離[m]_上書き
	/// DESC: When returning home: Distance to see the target [m] _ Overwrite - 帰宅時：ターゲットを見ている距離[m]_上書き
	pub overwrite_BackHome_LookTargetDist:u16,

	/// NAME: Item consumption MP magnification - アイテム消費MP倍率
	/// DESC: Item consumption MP multiplier - アイテム消費MP倍率
	pub goodsConsumptionRate:f32,

	/// NAME: pad
	pub unk2:f32,

	/// NAME: pad
	pub unk3:[u8;4],
}

impl Paramdef for SP_EFFECT_PARAM_ST {
const NAME: &'static str = "SP_EFFECT_PARAM_ST";
const VERSION: u16 = 4;
}
impl SP_EFFECT_PARAM_ST {
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetSelf(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetSelf(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetFriend(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetFriend(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetEnemy(&self) -> bool {
		&self.Bitfield1 & 0x4 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetEnemy(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x4
		} else {
			self.Bitfield1 &= 0xFB
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetPlayer(&self) -> bool {
		&self.Bitfield1 & 0x8 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetPlayer(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x8
		} else {
			self.Bitfield1 &= 0xF7
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetAI(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetAI(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xEF
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetLive(&self) -> bool {
		&self.Bitfield1 & 0x20 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetLive(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x20
		} else {
			self.Bitfield1 &= 0xDF
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield1
	pub fn get_effectTargetGhost(&self) -> bool {
		&self.Bitfield1 & 0x40 != 0
	}

	/// Bitfield1
	pub fn set_effectTargetGhost(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40
		} else {
			self.Bitfield1 &= 0xBF
		}
	}
	/// If this effect is applied, you will not sleep - この効果がかかっていると睡眠にかからなくなる
	/// Bitfield1
	pub fn get_disableSleep(&self) -> bool {
		&self.Bitfield1 & 0x80 != 0
	}

	/// Bitfield1
	pub fn set_disableSleep(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80
		} else {
			self.Bitfield1 &= 0x7F
		}
	}
	/// With this effect, you won't go mad - この効果がかかっていると発狂にかからなくなる
	/// Bitfield2
	pub fn get_disableMadness(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_disableMadness(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Apply special effects to attackers after damage (cannot enter defenders) - ダメージ後に攻撃者に特殊効果を適用（防御側には入れない）
	/// Bitfield2
	pub fn get_effectTargetAttacker(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_effectTargetAttacker(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// The icon is displayed even in the state of waiting for activation. - 発動待ちの状態でもアイコンを表示する。
	/// Bitfield2
	pub fn get_dispIconNonactive(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_dispIconNonactive(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Whether to generate a regain gauge - リゲインゲージを発生させるか
	/// Bitfield2
	pub fn get_regainGaugeDamage(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_regainGaugeDamage(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Do you want to correct the magic power? - 魔力補正するか？
	/// Bitfield2
	pub fn get_bAdjustMagicAblity(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_bAdjustMagicAblity(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Do you correct your faith? - 信仰補正するか？
	/// Bitfield2
	pub fn get_bAdjustFaithAblity(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_bAdjustFaithAblity(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Whether it is for the game clear lap bonus. - ゲームクリア周回ボーナス用かどうか。
	/// Bitfield2
	pub fn get_bGameClearBonus(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_bGameClearBonus(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Set whether or not it is effective against magic - 魔法に対して効果を発揮するかしないかを設定する
	/// Bitfield2
	pub fn get_magParamChange(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_magParamChange(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Set whether or not it is effective against miracles - 奇跡に対して効果を発揮するかしないかを設定する
	/// Bitfield3
	pub fn get_miracleParamChange(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_miracleParamChange(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Possession soul becomes 0. - 所持ソウルが0になります。
	/// Bitfield3
	pub fn get_clearSoul(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_clearSoul(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// If checked, issue an SOS sign request when activated - チェックが付いている場合、発動時にSOSサイン要求を発行
	/// Bitfield3
	pub fn get_requestSOS(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_requestSOS(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// If checked, issue a black SOS sign request when activated - チェックが付いている場合、発動時にブラックSOSサイン要求を発行
	/// Bitfield3
	pub fn get_requestBlackSOS(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_requestBlackSOS(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// If checked, issue an intrusion_A request when activated - チェックが付いている場合、発動時に侵入_Aリクエストを発行
	/// Bitfield3
	pub fn get_requestForceJoinBlackSOS(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_requestForceJoinBlackSOS(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// If checked, a kick request will be issued at the time of activation. - チェックが付いている場合、発動時にキック要求を発行
	/// Bitfield3
	pub fn get_requestKickSession(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_requestKickSession(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// If checked, an exit request will be issued at the time of activation. - チェックが付いている場合、発動時に退出要求を発行
	/// Bitfield3
	pub fn get_requestLeaveSession(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_requestLeaveSession(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// If checked, an intrusion request to the NPC will be issued at the time of activation. - チェックが付いている場合、発動時にNPCへの侵入要求を発行
	/// Bitfield3
	pub fn get_requestNpcInveda(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_requestNpcInveda(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Whether you can be corpse. With this check, you will not be dead - 死体状態になれるかどうか。このチェックが付いていると、死亡状態にならない
	/// Bitfield4
	pub fn get_noDead(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_noDead(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Does HP now affect even if the maximum HP increases or decreases? - 最大HPが増減しても、現在HPは影響しないか？
	/// Bitfield4
	pub fn get_bCurrHPIndependeMaxHP(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_bCurrHPIndependeMaxHP(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// [State change type] ignores [Durability] decrease due to [Corrosion] - 【状態変化タイプ】が【腐食】による【耐久度】減少を無視する
	/// Bitfield4
	pub fn get_corrosionIgnore(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_corrosionIgnore(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Ignore visual search invalidity - 視覚索敵無効を無視する
	/// Bitfield4
	pub fn get_sightSearchCutIgnore(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_sightSearchCutIgnore(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Ignore auditory search invalidity - 聴覚索敵無効を無視する
	/// Bitfield4
	pub fn get_hearingSearchCutIgnore(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_hearingSearchCutIgnore(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// You can use magic even in the anti-magic range - アンチマジック範囲でも魔法を使用できる
	/// Bitfield4
	pub fn get_antiMagicIgnore(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_antiMagicIgnore(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Don't get caught in the fake target that occurred - 発生した偽ターゲットに引っかからなくなる
	/// Bitfield4
	pub fn get_fakeTargetIgnore(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_fakeTargetIgnore(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// You will not be caught by the fake target of the human system that occurred - 発生した人系の偽ターゲットに引っかからなくなる
	/// Bitfield4
	pub fn get_fakeTargetIgnoreUndead(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_fakeTargetIgnoreUndead(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// You will not be caught by the fake target of the beast system that occurred - 発生した獣系の偽ターゲットに引っかからなくなる
	/// Bitfield5
	pub fn get_fakeTargetIgnoreAnimal(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_fakeTargetIgnoreAnimal(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// Gravity effect disabled - グラビティ効果無効
	/// Bitfield5
	pub fn get_grabityIgnore(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_grabityIgnore(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// If this effect is applied, it will not be poisoned. - この効果がかかっていると毒にかからなくなる
	/// Bitfield5
	pub fn get_disablePoison(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_disablePoison(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// If this effect is applied, you will not get plague - この効果がかかっていると疫病にかからなくなる
	/// Bitfield5
	pub fn get_disableDisease(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_disableDisease(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// With this effect, you won't get bleeding - この効果がかかっていると出血にかからなくなる
	/// Bitfield5
	pub fn get_disableBlood(&self) -> bool {
		&self.Bitfield5 & 0x10 != 0
	}

	/// Bitfield5
	pub fn set_disableBlood(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x10
		} else {
			self.Bitfield5 &= 0xEF
		}
	}
	/// With this effect, you won't be cursed - この効果がかかっていると呪いにかからなくなる
	/// Bitfield5
	pub fn get_disableCurse(&self) -> bool {
		&self.Bitfield5 & 0x20 != 0
	}

	/// Bitfield5
	pub fn set_disableCurse(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x20
		} else {
			self.Bitfield5 &= 0xDF
		}
	}
	/// If this effect is applied, you will be fascinated. - この効果がかかっていると魅了にかかるようになる
	/// Bitfield5
	pub fn get_enableCharm(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_enableCharm(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// Will the life be extended when the flag is set by TAE? - TAEによるフラグ設定時に寿命延長するか？
	/// Bitfield5
	pub fn get_enableLifeTime(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_enableLifeTime(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Do you want to correct your strength? - 筋力補正するか？
	/// Bitfield6
	pub fn get_bAdjustStrengthAblity(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_bAdjustStrengthAblity(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Do you want to correct your workmanship? - 技量補正するか？
	/// Bitfield6
	pub fn get_bAdjustAgilityAblity(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_bAdjustAgilityAblity(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Will it be extinguished by bonfire recovery? - 篝火回復で消えるか
	/// Bitfield6
	pub fn get_eraseOnBonfireRecover(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_eraseOnBonfireRecover(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// Set whether or not it is effective against throwing attacks - 投げ攻撃に対して効果を発揮するかしないかを設定する
	/// Bitfield6
	pub fn get_throwAttackParamChange(&self) -> bool {
		&self.Bitfield6 & 0x8 != 0
	}

	/// Bitfield6
	pub fn set_throwAttackParamChange(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x8
		} else {
			self.Bitfield6 &= 0xF7
		}
	}
	/// If checked, a request to leave the arena will be issued at the time of activation. - チェックが付いている場合、発動時に闘技場退出要求を発行
	/// Bitfield6
	pub fn get_requestLeaveColiseumSession(&self) -> bool {
		&self.Bitfield6 & 0x10 != 0
	}

	/// Bitfield6
	pub fn set_requestLeaveColiseumSession(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x10
		} else {
			self.Bitfield6 &= 0xEF
		}
	}
	/// Whether to be eligible for extension when the life extension effect is applied - 寿命延長効果が掛かっている時に延長対象になるかどうか
	/// Bitfield6
	pub fn get_isExtendSpEffectLife(&self) -> bool {
		&self.Bitfield6 & 0x20 != 0
	}

	/// Bitfield6
	pub fn set_isExtendSpEffectLife(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x20
		} else {
			self.Bitfield6 &= 0xDF
		}
	}
	/// Do you know the enemy? : [Activation condition] (for evil eye users) - 敵を把握しているか？：[発動条件](邪眼使用者用)
	/// Bitfield6
	pub fn get_hasTarget(&self) -> bool {
		&self.Bitfield6 & 0x40 != 0
	}

	/// Bitfield6
	pub fn set_hasTarget(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x40
		} else {
			self.Bitfield6 &= 0xBF
		}
	}
	/// Whether to replan at the time of activation - 発動時リプランニングするか
	/// Bitfield6
	pub fn get_replanningOnFire(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_replanningOnFire(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Pledge 0 - 誓約0
	/// Bitfield7
	pub fn get_vowType0(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_vowType0(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// Pledge 1 - 誓約1
	/// Bitfield7
	pub fn get_vowType1(&self) -> bool {
		&self.Bitfield7 & 0x2 != 0
	}

	/// Bitfield7
	pub fn set_vowType1(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x2
		} else {
			self.Bitfield7 &= 0xFD
		}
	}
	/// Pledge 2 - 誓約2
	/// Bitfield7
	pub fn get_vowType2(&self) -> bool {
		&self.Bitfield7 & 0x4 != 0
	}

	/// Bitfield7
	pub fn set_vowType2(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x4
		} else {
			self.Bitfield7 &= 0xFB
		}
	}
	/// Pledge 3 - 誓約3
	/// Bitfield7
	pub fn get_vowType3(&self) -> bool {
		&self.Bitfield7 & 0x8 != 0
	}

	/// Bitfield7
	pub fn set_vowType3(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x8
		} else {
			self.Bitfield7 &= 0xF7
		}
	}
	/// Pledge 4 - 誓約4
	/// Bitfield7
	pub fn get_vowType4(&self) -> bool {
		&self.Bitfield7 & 0x10 != 0
	}

	/// Bitfield7
	pub fn set_vowType4(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x10
		} else {
			self.Bitfield7 &= 0xEF
		}
	}
	/// Pledge 5 - 誓約5
	/// Bitfield7
	pub fn get_vowType5(&self) -> bool {
		&self.Bitfield7 & 0x20 != 0
	}

	/// Bitfield7
	pub fn set_vowType5(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x20
		} else {
			self.Bitfield7 &= 0xDF
		}
	}
	/// Pledge 6 - 誓約6
	/// Bitfield7
	pub fn get_vowType6(&self) -> bool {
		&self.Bitfield7 & 0x40 != 0
	}

	/// Bitfield7
	pub fn set_vowType6(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x40
		} else {
			self.Bitfield7 &= 0xBF
		}
	}
	/// Pledge 7 - 誓約7
	/// Bitfield7
	pub fn get_vowType7(&self) -> bool {
		&self.Bitfield7 & 0x80 != 0
	}

	/// Bitfield7
	pub fn set_vowType7(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x80
		} else {
			self.Bitfield7 &= 0x7F
		}
	}
	/// Pledge 8 - 誓約8
	/// Bitfield8
	pub fn get_vowType8(&self) -> bool {
		&self.Bitfield8 & 0x1 != 0
	}

	/// Bitfield8
	pub fn set_vowType8(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x1
		} else {
			self.Bitfield8 &= 0xFE
		}
	}
	/// Pledge 9 - 誓約9
	/// Bitfield8
	pub fn get_vowType9(&self) -> bool {
		&self.Bitfield8 & 0x2 != 0
	}

	/// Bitfield8
	pub fn set_vowType9(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x2
		} else {
			self.Bitfield8 &= 0xFD
		}
	}
	/// Pledge 10 - 誓約10
	/// Bitfield8
	pub fn get_vowType10(&self) -> bool {
		&self.Bitfield8 & 0x4 != 0
	}

	/// Bitfield8
	pub fn set_vowType10(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x4
		} else {
			self.Bitfield8 &= 0xFB
		}
	}
	/// Pledge 11 - 誓約11
	/// Bitfield8
	pub fn get_vowType11(&self) -> bool {
		&self.Bitfield8 & 0x8 != 0
	}

	/// Bitfield8
	pub fn set_vowType11(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x8
		} else {
			self.Bitfield8 &= 0xF7
		}
	}
	/// Pledge 12 - 誓約12
	/// Bitfield8
	pub fn get_vowType12(&self) -> bool {
		&self.Bitfield8 & 0x10 != 0
	}

	/// Bitfield8
	pub fn set_vowType12(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x10
		} else {
			self.Bitfield8 &= 0xEF
		}
	}
	/// Pledge 13 - 誓約13
	/// Bitfield8
	pub fn get_vowType13(&self) -> bool {
		&self.Bitfield8 & 0x20 != 0
	}

	/// Bitfield8
	pub fn set_vowType13(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x20
		} else {
			self.Bitfield8 &= 0xDF
		}
	}
	/// Pledge 14 - 誓約14
	/// Bitfield8
	pub fn get_vowType14(&self) -> bool {
		&self.Bitfield8 & 0x40 != 0
	}

	/// Bitfield8
	pub fn set_vowType14(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x40
		} else {
			self.Bitfield8 &= 0xBF
		}
	}
	/// Pledge 15 - 誓約15
	/// Bitfield8
	pub fn get_vowType15(&self) -> bool {
		&self.Bitfield8 & 0x80 != 0
	}

	/// Bitfield8
	pub fn set_vowType15(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x80
		} else {
			self.Bitfield8 &= 0x7F
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield9
	pub fn get_effectTargetOpposeTarget(&self) -> bool {
		&self.Bitfield9 & 0x1 != 0
	}

	/// Bitfield9
	pub fn set_effectTargetOpposeTarget(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x1
		} else {
			self.Bitfield9 &= 0xFE
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield9
	pub fn get_effectTargetFriendlyTarget(&self) -> bool {
		&self.Bitfield9 & 0x2 != 0
	}

	/// Bitfield9
	pub fn set_effectTargetFriendlyTarget(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x2
		} else {
			self.Bitfield9 &= 0xFD
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield9
	pub fn get_effectTargetSelfTarget(&self) -> bool {
		&self.Bitfield9 & 0x4 != 0
	}

	/// Bitfield9
	pub fn set_effectTargetSelfTarget(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x4
		} else {
			self.Bitfield9 &= 0xFB
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield9
	pub fn get_effectTargetPcHorse(&self) -> bool {
		&self.Bitfield9 & 0x8 != 0
	}

	/// Bitfield9
	pub fn set_effectTargetPcHorse(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x8
		} else {
			self.Bitfield9 &= 0xF7
		}
	}
	/// Only the target for which this judgment is checked is effective, the default is × - この判定にチェックが入っている対象のみ効果を発揮する、デフォルトは×
	/// Bitfield9
	pub fn get_effectTargetPcDeceased(&self) -> bool {
		&self.Bitfield9 & 0x10 != 0
	}

	/// Bitfield9
	pub fn set_effectTargetPcDeceased(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x10
		} else {
			self.Bitfield9 &= 0xEF
		}
	}
	/// Whether it will be shortened when the life shortening effect is applied - 寿命短縮効果が掛かっている時に短縮対象になるかどうか
	/// Bitfield9
	pub fn get_isContractSpEffectLife(&self) -> bool {
		&self.Bitfield9 & 0x20 != 0
	}

	/// Bitfield9
	pub fn set_isContractSpEffectLife(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x20
		} else {
			self.Bitfield9 &= 0xDF
		}
	}
	/// Do you want to delete it the moment you are in a waiting state? - 待ち状態になった瞬間に削除するか？
	/// Bitfield9
	pub fn get_isWaitModeDelete(&self) -> bool {
		&self.Bitfield9 & 0x40 != 0
	}

	/// Bitfield9
	pub fn set_isWaitModeDelete(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x40
		} else {
			self.Bitfield9 &= 0xBF
		}
	}
	/// Whether to apply the damage from this special effect even in the invincible state only when the state change type "Apply the activation function even when invincible" is applied. - 状態変化タイプ「無敵時でも発動機能を適応」が掛かっているときのみ、無敵状態でもこの特殊効果からのダメージを適応するか
	/// Bitfield9
	pub fn get_isIgnoreNoDamage(&self) -> bool {
		&self.Bitfield9 & 0x80 != 0
	}

	/// Bitfield9
	pub fn set_isIgnoreNoDamage(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x80
		} else {
			self.Bitfield9 &= 0x7F
		}
	}
	/// If ○, the abnormal state attack power multiplier correction of the attack para is applied. - ○なら攻撃パラの状態異常攻撃力倍率補正を適応します。
	/// Bitfield10
	pub fn get_isUseStatusAilmentAtkPowerCorrect(&self) -> bool {
		&self.Bitfield10 & 0x1 != 0
	}

	/// Bitfield10
	pub fn set_isUseStatusAilmentAtkPowerCorrect(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x1
		} else {
			self.Bitfield10 &= 0xFE
		}
	}
	/// If ○, the attack power multiplier correction of the attack para is applied. - ○なら攻撃パラの攻撃力倍率補正を適応します。
	/// Bitfield10
	pub fn get_isUseAtkParamAtkPowerCorrect(&self) -> bool {
		&self.Bitfield10 & 0x2 != 0
	}

	/// Bitfield10
	pub fn set_isUseAtkParamAtkPowerCorrect(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x2
		} else {
			self.Bitfield10 &= 0xFD
		}
	}
	/// If it is ○, it will not be deleted even if the character dies. Mainly used for death effects. - ○ならキャラが死亡しても削除しません。主に死亡エフェクトに使います。
	/// Bitfield10
	pub fn get_dontDeleteOnDead(&self) -> bool {
		&self.Bitfield10 & 0x4 != 0
	}

	/// Bitfield10
	pub fn set_dontDeleteOnDead(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x4
		} else {
			self.Bitfield10 &= 0xFB
		}
	}
	/// When this effect is applied, it will not be cold - この効果がかかっていると冷気にかからなくなる
	/// Bitfield10
	pub fn get_disableFreeze(&self) -> bool {
		&self.Bitfield10 & 0x8 != 0
	}

	/// Bitfield10
	pub fn set_disableFreeze(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x8
		} else {
			self.Bitfield10 &= 0xF7
		}
	}
	/// Do not synchronize with the net. It does not mean that you will be able to call it locally, but simply do not send it online. For example, a remote character does not activate locally, so nothing happens in that case. - ネット同期しない。ローカルに掛けるようになる、という意味ではなく、単にネット送信しない。例えばリモートキャラはローカル発動しないので、その場合何も起こらない。
	/// Bitfield10
	pub fn get_isDisableNetSync(&self) -> bool {
		&self.Bitfield10 & 0x10 != 0
	}

	/// Bitfield10
	pub fn set_isDisableNetSync(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x10
		} else {
			self.Bitfield10 &= 0xEF
		}
	}
	/// Set whether or not it is effective against spells - 呪術に対して効果を発揮するかしないかを設定する
	/// Bitfield10
	pub fn get_shamanParamChange(&self) -> bool {
		&self.Bitfield10 & 0x20 != 0
	}

	/// Bitfield10
	pub fn set_shamanParamChange(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x20
		} else {
			self.Bitfield10 &= 0xDF
		}
	}
	/// Whether to stop notifications targeting your army (used by EventMaker decisions and buddy platoons) - 自軍をターゲットしている通知を停止するかどうか(EventMakerでの判定やバディ小隊で使用)
	/// Bitfield10
	pub fn get_isStopSearchedNotify(&self) -> bool {
		&self.Bitfield10 & 0x40 != 0
	}

	/// Bitfield10
	pub fn set_isStopSearchedNotify(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x40
		} else {
			self.Bitfield10 &= 0xBF
		}
	}
	/// If it is ○, it will not be applied when it is judged to be shielded (× is always applied) - ○なら遮蔽判定されているときは掛からない（×は常に掛かる）
	/// Bitfield10
	pub fn get_isCheckAboveShadowTest(&self) -> bool {
		&self.Bitfield10 & 0x80 != 0
	}

	/// Bitfield10
	pub fn set_isCheckAboveShadowTest(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x80
		} else {
			self.Bitfield10 &= 0x7F
		}
	}
	/// Does not recognize the target while the special effect is applied (excluding the riding target) - 特殊効果が掛かっている間ターゲットを認識しない（騎乗ターゲット除く
	/// Bitfield11
	pub fn get_clearTarget(&self) -> bool {
		&self.Bitfield11 & 0x1 != 0
	}

	/// Bitfield11
	pub fn set_clearTarget(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x1
		} else {
			self.Bitfield11 &= 0xFE
		}
	}
	/// You will not be caught by the fake target of the subhuman system that occurred - 発生した亜人系の偽ターゲットに引っかからなくなる
	/// Bitfield11
	pub fn get_fakeTargetIgnoreAjin(&self) -> bool {
		&self.Bitfield11 & 0x2 != 0
	}

	/// Bitfield11
	pub fn set_fakeTargetIgnoreAjin(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x2
		} else {
			self.Bitfield11 &= 0xFD
		}
	}
	/// You will not be caught by the fake target of the phantom arts system that occurred - 発生した幻影アーツ系の偽ターゲットに引っかからなくなる
	/// Bitfield11
	pub fn get_fakeTargetIgnoreMirageArts(&self) -> bool {
		&self.Bitfield11 & 0x4 != 0
	}

	/// Bitfield11
	pub fn set_fakeTargetIgnoreMirageArts(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x4
		} else {
			self.Bitfield11 &= 0xFB
		}
	}
	/// If checked, issue an intrusion_B request when activated - チェックが付いている場合、発動時に侵入_Bリクエストを発行
	/// Bitfield11
	pub fn get_requestForceJoinBlackSOS_B(&self) -> bool {
		&self.Bitfield11 & 0x8 != 0
	}

	/// Bitfield11
	pub fn set_requestForceJoinBlackSOS_B(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x8
		} else {
			self.Bitfield11 &= 0xF7
		}
	}
	/// 
	/// Bitfield11
	pub fn get_unk353_4(&self) -> bool {
		&self.Bitfield11 & 0x10 != 0
	}

	/// Bitfield11
	pub fn set_unk353_4(&mut self, state: bool) {
		if state {
			self.Bitfield11 |= 0x10
		} else {
			self.Bitfield11 &= 0xEF
		}
	}

}
