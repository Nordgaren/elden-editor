/* This file was automatically generated from XML paramdefs. */
/// Data Version: 9
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NPC_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Behavior variation ID - 行動バリエーションID
	/// DESC: Variation ID used when calculating the action ID. - 行動IDを算出するときに使用するバリエーションID.
	pub behaviorVariationId:i32,

	/// NAME: Poison resistance correction rule ID - 毒耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the set value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_poison:i32,

	/// NAME: NPC name ID - NPC名ID
	/// DESC: NPC Name Message Parameter ID - NPC名メッセージパラメータ用ID
	pub nameId:i32,

	/// NAME: Turning speed [deg / sec] - 旋回速度[deg/sec]
	/// DESC: Rotational speed [degrees / second] that can turn in 1 second. - 1秒間に旋回できる回転速度[度/秒].
	pub turnVellocity:f32,

	/// NAME: Height per map [m] - 対マップあたりの高さ[m]
	/// DESC: The height of the capsule per character. - 対キャラ当たりカプセルの高さ.
	pub hitHeight:f32,

	/// NAME: Radius per map [m] - 対マップあたりの半径[m]
	/// DESC: Radius of capsule per character. - 対キャラ当たりカプセルの半径.
	pub hitRadius:f32,

	/// NAME: Weight [kg] - 重量[kg]
	/// DESC: weight. - 重量.
	pub weight:u32,

	/// NAME: Display position Y offset [m] - 表示位置Yオフセット[m]
	/// DESC: Offset of the model display position in the Y (height) direction. It can be floated from the hit position. - モデル表示位置のY（高さ）方向のオフセット。あたり位置より浮かせることができる。
	pub hitYOffset:f32,

	/// NAME: HP - ＨＰ
	/// DESC: Death grace. - 死亡猶予.
	pub hp:u32,

	/// NAME: MP - ＭＰ
	/// DESC: Magic usage. - 魔法使用量.
	pub mp:u32,

	/// NAME: Seoul - ソウル
	/// DESC: The amount of soul that a character can get at the time of death. - 死亡時に、キャラクターが取得できるソウル量.
	pub getSoul:u32,

	/// NAME: Item lottery ID_for enemies - アイテム抽選ID_エネミー用
	/// DESC: Specify the lottery ID_for enemies of the item to be acquired at the time of death. Please set only one of them. - 死亡時に取得するアイテムの抽選ID_エネミー用を指定。どちらか片方のみ設定してください。
	pub itemLotId_enemy:i32,

	/// NAME: Item lottery ID_for map - アイテム抽選ID_マップ用
	/// DESC: Specify for the lottery ID_map of the item to be acquired at the time of death. Please set only one of them. - 死亡時に取得するアイテムの抽選ID_マップ用を指定。どちらか片方のみ設定してください。
	pub itemLotId_map:i32,

	/// NAME: FootIK Ankle limit angle_roll - FootIK足首の制限角度_ロール
	/// DESC: FootIK Ankle roll limit angle (-1: no limit) - FootIK足首のロールの制限角度（-1：制限なし）
	pub maxAnkleRollAngle:f32,

	/// NAME: Per group and used navigation mesh - あたりグループと使用ナビメッシュ
	/// DESC: Set the hit judgment with other characters (If you hit the ragdoll, other characters will hit the ragdoll) - 他のキャラとのあたり判定を設定（ラグドールあたりにすると他のキャラがラグドールに当たるようになる）
	pub chrHitGroupAndNavimesh:u8,

	/// NAME: NPC face image ID - NPC顔画像ID
	/// DESC: NPC face image ID (0: invalid value (default)). Specify the ID of the face image to be displayed in the "Sign browsing menu", "Kick menu", etc. If it is an invalid value, the dress-up model is displayed. - NPC顔画像ID(0:無効値(デフォルト))。「サイン閲覧メニュー」「キックメニュー」などで表示する顔画像のIDを指定する。無効値なら着せ替えモデルを表示する
	pub faceIconId:u8,

	/// NAME: Deactivate distance setting [m] - ディアクティベート距離設定[m]
	/// DESC: Distance at which the character is deactivated (valid only for open placement characters) - キャラがディアクティベートされる距離（オープン配置キャラのみ有効）
	pub deactivateDist:i16,

	/// NAME: Character appearance condition Para - キャラ出現条件パラ
	/// DESC: Character appearance condition parameter ID - キャラ出現条件パラメータID
	pub chrActivateConditionParamId:u32,

	/// NAME: FootIK Appearance up / down limit - FootIK見た目の上下制限値
	/// DESC: FootIK Appearance up / down limit - FootIK見た目の上下制限値
	pub footIkErrorHeightLimit:f32,

	/// NAME: Human nature lottery ID - 人間性抽選ID
	/// DESC: Specify the lottery ID of human nature to be acquired at the time of death - 死亡時に取得する人間性の抽選IDを指定
	pub humanityLotId:i32,

	/// NAME: Resident special effect 0 - 常駐特殊効果0
	/// DESC: Resident special effect 0 - 常駐特殊効果0
	pub spEffectID0:i32,

	/// NAME: Resident special effect 1 - 常駐特殊効果1
	/// DESC: Resident special effect 1 - 常駐特殊効果1
	pub spEffectID1:i32,

	/// NAME: Resident special effect 2 - 常駐特殊効果2
	/// DESC: Resident special effect 2 - 常駐特殊効果2
	pub spEffectID2:i32,

	/// NAME: Resident special effect 3 - 常駐特殊効果3
	/// DESC: Resident special effect 3 - 常駐特殊効果3
	pub spEffectID3:i32,

	/// NAME: Resident special effect 4 - 常駐特殊効果4
	/// DESC: Resident special effect 4 - 常駐特殊効果4
	pub spEffectID4:i32,

	/// NAME: Resident special effect 5 - 常駐特殊効果5
	/// DESC: Resident special effect 5 - 常駐特殊効果5
	pub spEffectID5:i32,

	/// NAME: Resident special effect 6 - 常駐特殊効果6
	/// DESC: Resident special effect 6 - 常駐特殊効果6
	pub spEffectID6:i32,

	/// NAME: Resident special effect 7 - 常駐特殊効果7
	/// DESC: Resident special effect 7 - 常駐特殊効果7
	pub spEffectID7:i32,

	/// NAME: Special effect ID for lap bonus - 周回ボーナス用特殊効果ＩＤ
	/// DESC: Special effect ID for lap bonus - 周回ボーナス用特殊効果ＩＤ
	pub GameClearSpEffectID:i32,

	/// NAME: Physical attack cut rate [%] - 物理攻撃カット率[％]
	/// DESC: Set the damage cut rate when guarding for each attack - ガード時のダメージカット率を各攻撃ごとに設定
	pub physGuardCutRate:f32,

	/// NAME: Magic attack cut rate [%] - 魔法攻撃カット率[％]
	/// DESC: If it is not a guard attack, enter 0 - ガード攻撃でない場合は、0を入れる
	pub magGuardCutRate:f32,

	/// NAME: Flame attack power cut rate [%] - 炎攻撃力カット率[％]
	/// DESC: How much to cut the fire attack? - 炎攻撃をどれだけカットするか？
	pub fireGuardCutRate:f32,

	/// NAME: Electric shock attack power cut rate [%] - 電撃攻撃力カット率[％]
	/// DESC: How much to cut the electric shock attack? - 電撃攻撃をどれだけカットするか？
	pub thunGuardCutRate:f32,

	/// NAME: Anime ID offset 1 - アニメIDオフセット1
	/// DESC: All animations will be played with IDs shifted by this number. If not, the original anime ID is referenced. - すべてのアニメをこの数だけずらしたIDで再生します。なければ元のアニメIDを参照します。
	pub animIdOffset:i32,

	/// NAME: Gaze point of Rock Damipoli 0 - ロックダミポリ0の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint0:i16,

	/// NAME: Gaze point of Rock Damipoli 1 - ロックダミポリ1の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint1:i16,

	/// NAME: Gaze point of Rock Damipoli 2 - ロックダミポリ2の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint2:i16,

	/// NAME: Gaze point of Rock Damipoli 3 - ロックダミポリ3の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint3:i16,

	/// NAME: Gaze point of Rock Damipoli 4 - ロックダミポリ4の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint4:i16,

	/// NAME: Gaze point of Rock Damipoli 5 - ロックダミポリ5の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint5:i16,

	/// NAME: Network warp judgment distance [m / sec] - ネットワークワープ判定距離[m/秒]
	/// DESC: Distance to warp instead of complementary movement in network synchronization. Faster people (ex dragons) need to be longer. - ネットワークの同期で、補完移動でなくワープさせる距離。スピードの速い人（exドラゴン)は長めにしてあげる必要がある。
	pub networkWarpDist:f32,

	/// NAME: R1 - R1
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorR1:i32,

	/// NAME: L1 - L1
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorL1:i32,

	/// NAME: R2 - R2
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorR2:i32,

	/// NAME: L2 - L2
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorL2:i32,

	/// NAME: □ - □
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorRL:i32,

	/// NAME: ○ ○ - ○
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorRR:i32,

	/// NAME: × - ×
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorRD:i32,

	/// NAME: △ - △
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorRU:i32,

	/// NAME: ← - ←
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorLL:i32,

	/// NAME: → - →
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorLR:i32,

	/// NAME: ↓ - ↓
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorLD:i32,

	/// NAME: ↑ - ↑
	/// DESC: Register the ID from the action parameter tool and specify the action. - 行動パラメータツールからIDを登録し、行動を指定する.
	pub dbgBehaviorLU:i32,

	/// NAME: Anime ID offset 2 - アニメIDオフセット2
	/// DESC: All animations will be played with IDs shifted by this number. If not, the animation ID of the animation ID offset 1 is referred to. - すべてのアニメをこの数だけずらしたIDで再生します。なければアニメIDオフセット1のアニメIDを参照します。
	pub animIdOffset2:i32,

	/// NAME: Damage group 1 Damage multiplier - ダメージグループ1ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 1 - 部位1に対するダメージ処理に適応する倍率
	pub partsDamageRate1:f32,

	/// NAME: Damage group 2 Damage multiplier - ダメージグループ2ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 2 - 部位2に対するダメージ処理に適応する倍率
	pub partsDamageRate2:f32,

	/// NAME: Damage group 3 Damage multiplier - ダメージグループ3ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 3 - 部位3に対するダメージ処理に適応する倍率
	pub partsDamageRate3:f32,

	/// NAME: Damage group 4 damage multiplier - ダメージグループ4ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 4 - 部位4に対するダメージ処理に適応する倍率
	pub partsDamageRate4:f32,

	/// NAME: Damage group 5 damage multiplier - ダメージグループ5ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 5 - 部位5に対するダメージ処理に適応する倍率
	pub partsDamageRate5:f32,

	/// NAME: Damage group 6 damage multiplier - ダメージグループ6ダメージ倍率
	/// DESC: Magnification to adapt to damage processing for part 6 - 部位6に対するダメージ処理に適応する倍率
	pub partsDamageRate6:f32,

	/// NAME: Damage group 7 Damage multiplier - ダメージグループ7ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 7 - 部位7に対するダメージ処理に適応する倍率
	pub partsDamageRate7:f32,

	/// NAME: Damage group 8 damage multiplier - ダメージグループ8ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for part 8 - 部位8に対するダメージ処理に適応する倍率
	pub partsDamageRate8:f32,

	/// NAME: Weak point damage ratio - 弱点部位ダメージ倍率
	/// DESC: Magnification that adapts to damage processing for weak points - 弱点部位に対するダメージ処理に適応する倍率
	pub weakPartsDamageRate:f32,

	/// NAME: SA recovery time correction value - SA回復時間補正値
	/// DESC: Correction value for super armor recovery time - スーパーアーマー回復時間用の補正値
	pub superArmorRecoverCorrection:f32,

	/// NAME: Knockback distance at SA break - SAブレイク時ノックバック距離
	/// DESC: Knockback distance that can be used only at the time of SA break - SAブレイクの時だけに使えるノックバック距離
	pub superArmorBrakeKnockbackDist:f32,

	/// NAME: stamina - スタミナ
	/// DESC: Total amount of stamina. - スタミナ総量.
	pub stamina:u16,

	/// NAME: Stamina recovery basic speed [point / s] - スタミナ回復基本速度[point/s]
	/// DESC: Stamina recovery basic speed [point / s] - スタミナ回復基本速度[point/s]
	pub staminaRecoverBaseVel:u16,

	/// NAME: Physical defense - 物理防御力
	/// DESC: Damage reduction base value for physical attacks. - 物理攻撃に対するダメージ減少基本値.
	pub def_phys:u16,

	/// NAME: Slash defense [%] - 斬撃防御力[％]
	/// DESC: Look at the attack attribute, and if it is a slash attribute, reduce the defense power. - 攻撃属性を見て、斬撃属性のときは、防御力を減少させる.
	pub def_slash:i16,

	/// NAME: Blow defense [%] - 打撃防御力[％]
	/// DESC: Look at the attack attribute, and if it is a hit attribute, reduce the defense power. - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
	pub def_blow:i16,

	/// NAME: Puncture defense [%] - 刺突防御力[％]
	/// DESC: Look at the attack attribute, and if it is a piercing attribute, reduce the defense power. - 攻撃属性を見て、刺突属性のときは、防御力を減少させる.
	pub def_thrust:i16,

	/// NAME: Magic defense - 魔法防御力
	/// DESC: Damage reduction base value for magic attacks. - 魔法攻撃に対するダメージ減少基本値.
	pub def_mag:u16,

	/// NAME: Fire defense - 炎防御力
	/// DESC: Damage reduction base value against fire attack. - 炎攻撃に対するダメージ減少基本値.
	pub def_fire:u16,

	/// NAME: Electric shock defense - 電撃防御力
	/// DESC: Damage reduction base value against electric shock attack. - 電撃攻撃に対するダメージ減少基本値.
	pub def_thunder:u16,

	/// NAME: Repellent defense - はじき防御力
	/// DESC: Used to determine the repelling of enemy attacks. // It is intended to be repelled by normal attacks other than guards. // Enemies with a hard skin can be repelled without doing anything ... It doesn't matter if it is a normal enemy. - 敵の攻撃のはじき判定に使用。//ガード以外の通常攻撃でもはじけるようにするためのものです.//硬い表皮の敵は、何もしなくてもはじかれることがある…みたいな感じ通常の敵なら関係ないです.
	pub defFlickPower:u16,

	/// NAME: Poison resistance - 毒耐性
	/// DESC: Difficulty in getting poisonous - 毒状態異常へのかかりにくさ
	pub resist_poison:u16,

	/// NAME: Epidemic resistance - 疫病耐性
	/// DESC: Difficulty in getting sick - 疫病状態異常へのかかりにくさ
	pub resist_desease:u16,

	/// NAME: Bleeding resistance - 出血耐性
	/// DESC: Difficulty in getting bleeding abnormalities - 出血状態異常へのかかりにくさ
	pub resist_blood:u16,

	/// NAME: Curse resistance - 呪耐性
	/// DESC: Difficulty in dealing with abnormal curse conditions - 呪状態異常へのかかりにくさ
	pub resist_curse:u16,

	/// NAME: Replacement model ID for wandering ghost - 徘徊ゴースト時差し替えモデルID
	/// DESC: Replacement model when wandering ghost, texture ID - 徘徊ゴースト化したときの差し替えモデル、テクスチャID
	pub ghostModelId:i16,

	/// NAME: Normal replacement resource ID - 通常時差し替えリソースID
	/// DESC: Replace resource ID during normal operation (do not use it unnecessarily) - 通常時のリソースID差し替え（むやみに使わないこと）
	pub normalChangeResouceId:i16,

	/// NAME: Guard range [deg] - ガード範囲[deg]
	/// DESC: Defense range angle when guarding weapons. Pending - 武器のガード時の防御発生範囲角度.保留中
	pub guardAngle:i16,

	/// NAME: Slash attack cut rate [%] - 斬撃攻撃カット率[％]
	/// DESC: Looking at the attack type, what percentage of the damage of the slashing attribute is cut? Specify - 攻撃タイプを見て、斬撃属性のダメージを何％カットするか？を指定
	pub slashGuardCutRate:i16,

	/// NAME: Batter attack cut rate [%] - 打撃攻撃カット率[％]
	/// DESC: Looking at the attack type, what percentage of the damage of the hit attribute is cut? Specify - 攻撃タイプを見て、打撃属性のダメージを何％カットするか？を指定
	pub blowGuardCutRate:i16,

	/// NAME: Puncture attack cut rate [%] - 刺突攻撃カット率[％]
	/// DESC: Looking at the attack type, what percentage of the damage of the piercing attribute is cut? Specify - 攻撃タイプを見て、刺突属性のダメージを何％カットするか？を指定
	pub thrustGuardCutRate:i16,

	/// NAME: Gaze point of Rock Damipoli 6 - ロックダミポリ6の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint6:i16,

	/// NAME: Normal replacement texture character ID - 通常時差し替えテクスチャキャラID
	/// DESC: Normal replacement texture character ID (do not use it unnecessarily) - 通常時差し替えテクスチャキャラID（むやみに使わないこと）
	pub normalChangeTexChrId:i16,

	/// NAME: Display format of drop items - ドロップアイテムの表示形式
	/// DESC: Display method when dropping an item (corpse emission or item display) - アイテムドロップ時の表示方法(死体発光orアイテム表示)
	pub dropType:u16,

	/// NAME: Knockback cut rate [%] - ノックバックカット率[％]
	/// DESC: Decrease value when receiving knockback damage / Specifically, cut the initial knockback speed of the attacking side - ノックバックダメージを受けたときの減少値／具体的には、攻撃側のノックバック初速度をカットする
	pub knockbackRate:u8,

	/// NAME: Knockback parameter ID - ノックバックパラメータID
	/// DESC: Set the parameter ID used for knockback - ノックバック時に使用するパラメータIDを設定
	pub knockbackParamId:u8,

	/// NAME: Fall damage reduction correction [%] - 落下ダメージ軽減補正[％]
	/// DESC: Fall damage reduction correction [%] - 落下ダメージ軽減補正[％]
	pub fallDamageDump:u8,

	/// NAME: Stamina attack cut rate [%] - スタミナ攻撃カット率[％]
	/// DESC: Defense against enemy stamina attacks when guarding successfully - ガード成功時に、敵のスタミナ攻撃に対する防御力
	pub staminaGuardDef:u8,

	/// NAME: Sleep tolerance - 睡眠耐性
	/// DESC: Difficulty in getting sleep abnormalities - 睡眠状態異常へのかかりにくさ
	pub resist_sleep:u16,

	/// NAME: Madness resistance - 発狂耐性
	/// DESC: Difficulty in getting mad - 発狂状態異常へのかかりにくさ
	pub resist_madness:u16,

	/// NAME: Sleep attack cut rate [%] - 睡眠攻撃カット率[％]
	/// DESC: How much to cut the attack power against sleep (set as a special effect parameter) - 睡眠に対する攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub sleepGuardResist:i8,

	/// NAME: Mad attack cut rate [%] - 発狂攻撃カット率[％]
	/// DESC: How much to cut the attack power against madness (set as a special effect parameter) - 発狂に対する攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub madnessGuardResist:i8,

	/// NAME: Gaze point of Rock Damipoli 7 - ロックダミポリ7の注視点
	/// DESC: Gaze at the Damipoli specified when locking the Lock-on Damipoly 22X (-1: Invalid) - ロックオンダミポリ22Xをロックしている際に指定したダミポリを注視する（-1：無効）
	pub lockGazePoint7:i16,

	/// NAME: MP recovery basic speed [% / s] - MP回復基本速度[％/s]
	/// DESC: MP recovery basic speed [% / s] - MP回復基本速度[％/s]
	pub mpRecoverBaseVel:u8,

	/// NAME: Damage attenuation rate when repelling [%] - はじき時ダメージ減衰率[%]
	/// DESC: Set a value to attenuate damage when repelling an attack - 攻撃をはじいた時にダメージを減衰する値を設定
	pub flickDamageCutRate:u8,

	/// NAME: Default LOD Param ID - デフォルトLODパラムID
	/// DESC: Default LOD Param ID (-1: None) - デフォルトLODパラムID(-1：なし)
	pub defaultLodParamId:i8,

	/// NAME: Drawing type - 描画タイプ
	/// DESC: Drawing type - 描画タイプ
	pub drawType:i8,

	/// NAME: NPC type - NPCタイプ
	/// DESC: NPC type. OK if Zako enemies / boss enemies are distinguished - NPCの種類.ザコ敵/ボス敵が区別されていればOK
	pub npcType:u8,

	/// NAME: Team type - チームタイプ
	/// DESC: NPC attack hit / not hit, aim / not aim setting - NPCの攻撃が当たる/当たらない、狙う/狙わない設定
	pub teamType:u8,

	/// NAME: Movement type - 移動タイプ
	/// DESC: Moving method. This changes the control. - 移動方法。これにより制御が変更される.
	pub moveType:u8,

	/// NAME: Lock distance - ロック距離
	/// DESC: Lock-on distance [m] - ロックオンできる距離[m]
	pub lockDist:u8,

	/// NAME: Weakness defense material 1 [SE] - 弱点防御材質1【SE】
	/// DESC: Determines the SE that sounds when the weak point is damaged. 1 - 弱点部位ダメージを受けた時に鳴らすSEを判定する。1
	pub materialSe_Weak1:u16,

	/// NAME: Weakness protection material 1 [SFX] - 弱点防御材質1【SFX】
	/// DESC: Determines the SFX that occurs when the weak point is damaged. 1 - 弱点部位ダメージを受けた時に発生するSFXを判定する。1
	pub materialSfx_Weak1:u16,

	/// NAME: Part damage application attack - 部位ダメージ適用攻撃
	/// DESC: Set the attack type to apply site damage - 部位ダメージを適用する攻撃タイプを設定する
	pub partsDamageType:u8,

	/// NAME: Pledge - 誓約
	/// DESC: Pledge type - 誓約タイプ
	pub vowType:u8,

	/// NAME: Guard level - ガードレベル
	/// DESC: When guarding, which guard motion will the enemy attack? Decide - ガードしたとき、敵の攻撃をどのガードモーションで受けるか？を決める
	pub guardLevel:i8,

	/// NAME: Combustion SFX type - 燃焼SFXタイプ
	/// DESC: SFX type at the time of combustion - 燃焼時のSFXタイプ
	pub burnSfxType:u8,

	/// NAME: Poison resistance cut rate [%] - 毒耐性カット率[％]
	/// DESC: How much to cut the attack power to poison (set to the special effect parameter) - 毒にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub poisonGuardResist:i8,

	/// NAME: Epidemic attack cut rate [%] - 疫病攻撃カット率[％]
	/// DESC: How much to cut the attack power (set as a special effect parameter) to make it a plague - 疫病にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub diseaseGuardResist:i8,

	/// NAME: Bleeding attack cut rate [%] - 出血攻撃カット率[％]
	/// DESC: How much to cut the attack power (set as a special effect parameter) to make bleeding - 出血にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub bloodGuardResist:i8,

	/// NAME: Curse attack cut rate [%] - 呪攻撃カット率[％]
	/// DESC: How much to cut the attack power (set as a special effect parameter) to make a curse - 呪にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub curseGuardResist:i8,

	/// NAME: Parry attack power - パリィ攻撃力
	/// DESC: Parry attack power. Used by the parrying side - パリィ攻撃力。パリィする側が使用
	pub parryAttack:u8,

	/// NAME: Parry defense - パリィ防御力
	/// DESC: Parry defense. Used by the parried side. - パリィ防御力。パリィされる側が使用。
	pub parryDefence:u8,

	/// NAME: SFX size - SFXサイズ
	/// DESC: SFX size - SFXサイズ
	pub sfxSize:u8,

	/// NAME: Camera extrusion area radius [m] - カメラ押し出し領域半径[m]
	/// DESC: Camera extrusion area radius [m] - カメラ押し出し領域半径[m]
	pub pushOutCamRegionRadius:u8,

	/// NAME: Do you want to hit stop - ヒットストップするか
	/// DESC: Setting whether to perform hit stop processing - ヒットストップ処理を行うかどうかの設定
	pub hitStopType:u8,

	/// NAME: Ladder top termination offset [1 / 10m] - はしご上終端オフセット[1/10m]
	/// DESC: Upper side of the ladder end judgment offset - はしご終端判定用オフセット上側
	pub ladderEndChkOffsetTop:u8,

	/// NAME: Ladder bottom termination offset [1 / 10m] - はしご下終端オフセット[1/10m]
	/// DESC: Lower side of ladder end judgment offset - はしご終端判定用オフセット下側
	pub ladderEndChkOffsetLow:u8,

	/// NAME: Ragdoll per camera hit - カメラヒットあたりラグドール
	/// DESC: Does the camera hit the enemy ragdoll? (Valid only when hitting the player) - 敵のラグドールにカメラがあたるか。(プレイヤにも当たるときのみ有効)
	pub Bitfield2:u8,

	/// NAME: Model display mask 0 - モデル表示マスク0
	/// DESC: Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	pub Bitfield3:u8,

	/// NAME: Model display mask 8 - モデル表示マスク8
	/// DESC: Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	pub Bitfield4:u8,

	/// NAME: Whether to enable swinging - 首振り有効にするか
	/// DESC: Do you want to enable the swing set in Param Weaver? - パラムウィーバで設定された首振りを有効にするか。
	pub Bitfield5:u8,

	/// NAME: Is it a riding special attack? - 騎乗特攻か
	/// DESC: Will you be the target of a riding special attack (if you are riding)? - （騎乗中であれば）騎乗特攻の対象になるか
	pub Bitfield6:u8,

	/// NAME: Model display mask 16 - モデル表示マスク16
	/// DESC: Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	pub Bitfield7:u8,

	/// NAME: Model display mask 24 - モデル表示マスク24
	/// DESC: Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	pub Bitfield8:u8,

	/// NAME: Drop item radius correction - ドロップアイテム半径補正
	/// DESC: Radius to be added as a correction to the cylinder radius of normal Item search judgment (applicable to enemy drop items. Used for large characters etc.) - 通常のItem検索判定の円柱半径に、補正として足し合わせる半径(敵ドロップアイテムに適用。大きなキャラなどで使用する)
	pub itemSearchRadius:f32,

	/// NAME: Height per character [m] - 対キャラあたりの高さ[m]
	/// DESC: The height of the capsule per character. - 対キャラ当たりカプセルの高さ.
	pub chrHitHeight:f32,

	/// NAME: Radius per character [m] - 対キャラあたりの半径[m]
	/// DESC: Radius of capsule per character. - 対キャラ当たりカプセルの半径.
	pub chrHitRadius:f32,

	/// NAME: Special turning type - 特殊旋回のタイプ
	/// DESC: Special turning type - 特殊旋回のタイプ
	pub specialTurnType:u8,

	/// NAME: Do you get a boss in soul? - ソウルはボス入手か
	/// DESC: Do you get a boss in soul? - ソウルはボス入手か
	pub Bitfield9:u8,

	/// NAME: Dark defense - 闇防御力
	/// DESC: Damage reduction base value for dark attacks. - 闇攻撃に対するダメージ減少基本値.
	pub def_dark:u16,

	/// NAME: Threat level - 脅威度
	/// DESC: Threat level. If it is 0, even if the PC is found, "FE that seems to be found" is not displayed. - 脅威度。0ならPCが見つかっても「見つかりそうFE」を表示しない
	pub threatLv:u32,

	/// NAME: Threshold for special turning distance [m] - 特殊旋回の使用距離の閾値[m]
	/// DESC: Make a special turn when the distance to the target is greater than or equal to the set threshold - ターゲットとの距離が設定された閾値以上の場合に、特殊旋回を行う
	pub specialTurnDistanceThreshold:f32,

	/// NAME: Foot effect identifier - フットエフェクト識別子
	/// DESC: The SFX identifier used in the automatic foot effect. (ZZZ of XYYZZZ) - 自動フットエフェクトで使用するSFX識別子。（XYYZZZのZZZ）
	pub autoFootEffectSfxId:i32,

	/// NAME: Defensive material 1 [SE] - 防御材質1【SE】
	/// DESC: Determine the SE that sounds when damaged. 1. It is OK to set it by appearance. - ダメージを受けたときに鳴らすＳＥを判定する。1.見た目で設定してＯＫ.
	pub materialSe1:u16,

	/// NAME: Defensive material 1 [SFX] - 防御材質1【SFX】
	/// DESC: Determine the SFX that occurs when you take damage. 1. It is OK to set it by appearance. - ダメージを受けたときに発生するSFXを判定する。1.見た目で設定してＯＫ.
	pub materialSfx1:u16,

	/// NAME: Weakness defense material 2 [SE] - 弱点防御材質2【SE】
	/// DESC: Determines the SE that sounds when the weak point is damaged. 2 - 弱点部位ダメージを受けた時に鳴らすSEを判定する。2
	pub materialSe_Weak2:u16,

	/// NAME: Weakness protection material 2 [SFX] - 弱点防御材質2【SFX】
	/// DESC: Determines the SFX that occurs when the weak point is damaged. 2 - 弱点部位ダメージを受けた時に発生するSFXを判定する。2
	pub materialSfx_Weak2:u16,

	/// NAME: Defensive material 2 [SE] - 防御材質2【SE】
	/// DESC: Determine the SE that sounds when damaged. 2. It is OK to set it by appearance. - ダメージを受けたときに鳴らすＳＥを判定する。2.見た目で設定してＯＫ.
	pub materialSe2:u16,

	/// NAME: Defensive material 2 [SFX] - 防御材質2【SFX】
	/// DESC: Determine the SFX that occurs when you take damage. 2. It is OK to set it by appearance. - ダメージを受けたときに発生するSFXを判定する。2.見た目で設定してＯＫ.
	pub materialSfx2:u16,

	/// NAME: Resident special effect 8 - 常駐特殊効果8
	/// DESC: Resident special effect 8 - 常駐特殊効果8
	pub spEffectID8:i32,

	/// NAME: Resident special effect 9 - 常駐特殊効果9
	/// DESC: Resident special effect 9 - 常駐特殊効果9
	pub spEffectID9:i32,

	/// NAME: Resident special effect 10 - 常駐特殊効果10
	/// DESC: Resident special effect 10 - 常駐特殊効果10
	pub spEffectID10:i32,

	/// NAME: Resident special effects 11 - 常駐特殊効果11
	/// DESC: Resident special effects 11 - 常駐特殊効果11
	pub spEffectID11:i32,

	/// NAME: Resident special effect 12 - 常駐特殊効果12
	/// DESC: Resident special effect 12 - 常駐特殊効果12
	pub spEffectID12:i32,

	/// NAME: Resident special effect 13 - 常駐特殊効果13
	/// DESC: Resident special effect 13 - 常駐特殊効果13
	pub spEffectID13:i32,

	/// NAME: Resident special effect 14 - 常駐特殊効果14
	/// DESC: Resident special effect 14 - 常駐特殊効果14
	pub spEffectID14:i32,

	/// NAME: Resident special effects 15 - 常駐特殊効果15
	/// DESC: Resident special effects 15 - 常駐特殊効果15
	pub spEffectID15:i32,

	/// NAME: Foot decal identifier 1 - フットデカール識別子1
	/// DESC: Decal to be attached when a foot effect occurs. Floor material is also taken into consideration - フットエフェクト発生時に貼られるデカール。床材質も考慮される
	pub autoFootEffectDecalBaseId1:i32,

	/// NAME: Toughness - 強靭度
	/// DESC: Basic value of toughness - 強靭度の基本値
	pub toughness:u32,

	/// NAME: Toughness recovery time correction value - 強靭度 回復時間補正値
	/// DESC: Correction value for toughness recovery time - 強靭度の回復時間用の補正値
	pub toughnessRecoverCorrection:f32,

	/// NAME: Non-attribute damage multiplier - 無属性ダメージ倍率
	/// DESC: Non-attribute damage multiplier. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 無属性ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub neutralDamageCutRate:f32,

	/// NAME: Slash damage multiplier - 斬撃ダメージ倍率
	/// DESC: Slash damage multiplier. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 斬撃ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub slashDamageCutRate:f32,

	/// NAME: Batter damage multiplier - 打撃ダメージ倍率
	/// DESC: Batter damage multiplier. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 打撃ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub blowDamageCutRate:f32,

	/// NAME: Puncture damage ratio - 刺突ダメージ倍率
	/// DESC: Puncture damage ratio. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 刺突ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub thrustDamageCutRate:f32,

	/// NAME: Magic damage multiplier - 魔法ダメージ倍率
	/// DESC: Magic damage multiplier. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 魔法ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub magicDamageCutRate:f32,

	/// NAME: Flame damage multiplier - 火炎ダメージ倍率
	/// DESC: Flame damage multiplier. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 火炎ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub fireDamageCutRate:f32,

	/// NAME: Electric shock damage ratio - 電撃ダメージ倍率
	/// DESC: Electric shock damage ratio. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 電撃ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub thunderDamageCutRate:f32,

	/// NAME: Dark damage multiplier - 闇ダメージ倍率
	/// DESC: Dark damage multiplier. The final damage value is the value obtained by multiplying the damage calculation result by this value. - 闇ダメージ倍率。ダメージ計算結果にこの値をかけた値が最終ダメージ値になります。
	pub darkDamageCutRate:f32,

	/// NAME: Dark attack power cut rate [%] - 闇攻撃力カット率[％]
	/// DESC: How much to cut the dark attack? - 闇攻撃をどれだけカットするか？
	pub darkGuardCutRate:f32,

	/// NAME: Cross update priority offset [m] - クロス更新優先度オフセット[m]
	/// DESC: Cross update priority offset [m] - クロス更新優先度オフセット[m]
	pub clothUpdateOffset:i8,

	/// NAME: Weight setting for NPC players - NPCプレイヤー時重量設定
	/// DESC: Equipment weight type applied for NPC players - NPCプレイヤーのときに適用される装備重量タイプ
	pub npcPlayerWeightType:u8,

	/// NAME: Normal time replacement model ID - 通常時差し替えモデルID
	/// DESC: Normal replacement model, texture ID - 通常時の差し替えモデル、テクスチャID
	pub normalChangeModelId:i16,

	/// NAME: Normal replacement anime character ID - 通常時差し替えアニメキャラID
	/// DESC: Replace the target animation with the specified ID Anibnd - 対象のアニメを指定IDのAnibndで差し替える
	pub normalChangeAnimChrId:i16,

	/// NAME: Paint render target size [pix] - ペイントレンダーターゲットサイズ[pix]
	/// DESC: Paint render target size [pix] - ペイントレンダーターゲットサイズ[pix]
	pub paintRenderTargetSize:u16,

	/// NAME: Epidemic resistance correction rule ID - 疫病耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the set value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_disease:i32,

	/// NAME: Applicable shader ID - 適用シェーダーID
	/// DESC: ID of the phantom parameter .xlsm to apply - 適用するファントムパラメータ.xlsmのID
	pub phantomShaderId:i32,

	/// NAME: Multiplayer correction parameter ID - マルチプレイ補正パラメータID
	/// DESC: Multiplayer correction parameter ID - マルチプレイ補正パラメータID
	pub multiPlayCorrectionParamId:i32,

	/// NAME: FootIK Ankle limit angle_pitch - FootIK足首の制限角度_ピッチ
	/// DESC: FootIK Ankle pitch limit angle (-1: no limit). If you have not set Foot End L S in HAT, this angle is used in common with rolls. - FootIK足首のピッチの制限角度（-1：制限なし）。HATでFoot End L Sを設定していない場合はこの角度がロールと共通で使用される。
	pub maxAnklePitchAngle:f32,

	/// NAME: Cold resistance - 冷気耐性
	/// DESC: Difficulty in getting cold air condition abnormal - 冷気状態異常へのかかりにくさ
	pub resist_freeze:u16,

	/// NAME: Cold attack cut rate [%] - 冷気攻撃カット率[％]
	/// DESC: How much to cut the attack power against cold air (set as a special effect parameter) - 冷気に対する攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub freezeGuardResist:i8,

	/// NAME: pad - pad
	pub pad1:[u8;1],

	/// NAME: Lock camera parameter ID - ロックカメラパラメータID
	/// DESC: The ID of the lock camera parameter applied to the camera when locked on. Highest priority. Unused if -1 - ロックオンされた際にカメラに適用させるロックカメラパラメータのID。最も優先度が高い。-1なら未使用
	pub lockCameraParamId:i32,

	/// NAME: Resident special effects 16 - 常駐特殊効果16
	/// DESC: Resident special effects 16 - 常駐特殊効果16
	pub spEffectID16:i32,

	/// NAME: Resident special effects 17 - 常駐特殊効果17
	/// DESC: Resident special effects 17 - 常駐特殊効果17
	pub spEffectID17:i32,

	/// NAME: Resident special effects 18 - 常駐特殊効果18
	/// DESC: Resident special effects 18 - 常駐特殊効果18
	pub spEffectID18:i32,

	/// NAME: Resident special effects 19 - 常駐特殊効果19
	/// DESC: Resident special effects 19 - 常駐特殊効果19
	pub spEffectID19:i32,

	/// NAME: Resident special effect 20 - 常駐特殊効果20
	/// DESC: Resident special effect 20 - 常駐特殊効果20
	pub spEffectID20:i32,

	/// NAME: Resident special effect 21 - 常駐特殊効果21
	/// DESC: Resident special effect 21 - 常駐特殊効果21
	pub spEffectID21:i32,

	/// NAME: Resident special effect 22 - 常駐特殊効果22
	/// DESC: Resident special effect 22 - 常駐特殊効果22
	pub spEffectID22:i32,

	/// NAME: Resident special effect 23 - 常駐特殊効果23
	/// DESC: Resident special effect 23 - 常駐特殊効果23
	pub spEffectID23:i32,

	/// NAME: Resident special effect 24 - 常駐特殊効果24
	/// DESC: Resident special effect 24 - 常駐特殊効果24
	pub spEffectID24:i32,

	/// NAME: Resident special effects 25 - 常駐特殊効果25
	/// DESC: Resident special effects 25 - 常駐特殊効果25
	pub spEffectID25:i32,

	/// NAME: Resident special effect 26 - 常駐特殊効果26
	/// DESC: Resident special effect 26 - 常駐特殊効果26
	pub spEffectID26:i32,

	/// NAME: Resident special effect 27 - 常駐特殊効果27
	/// DESC: Resident special effect 27 - 常駐特殊効果27
	pub spEffectID27:i32,

	/// NAME: Resident special effects 28 - 常駐特殊効果28
	/// DESC: Resident special effects 28 - 常駐特殊効果28
	pub spEffectID28:i32,

	/// NAME: Resident special effect 29 - 常駐特殊効果29
	/// DESC: Resident special effect 29 - 常駐特殊効果29
	pub spEffectID29:i32,

	/// NAME: Resident special effect 30 - 常駐特殊効果30
	/// DESC: Resident special effect 30 - 常駐特殊効果30
	pub spEffectID30:i32,

	/// NAME: Resident special effects 31 - 常駐特殊効果31
	/// DESC: Resident special effects 31 - 常駐特殊効果31
	pub spEffectID31:i32,

	/// NAME: Central angle of lockable area [deg] - ロック不可領域の中心角[deg]
	/// DESC: Create a conical lock-on non-lockable area beneath the enemy. The angle of the size of the cone. Can be changed temporarily from TAE - 敵の真下に円錐状のロックオン不可領域を作る。円錐の広さの角度。TAEから一時的に変更可能
	pub disableLockOnAng:f32,

	/// NAME: Cross OffLOD level - クロスOffLODレベル
	/// DESC: Set the LOD level to turn off cross processing - クロスの処理を切るLODレベルを設定する
	pub clothOffLodLevel:i8,

	/// NAME: Whether to use FootIK results to match undulations - 起伏にあわせるのにFootIK結果を用いるか
	/// DESC: Do you use the FootIK result to match your character to the undulations of the ground? Cannot be used for flying characters - キャラを地面の起伏に合わせる際に、FootIK結果を用いるか。飛行キャラの場合は使用不可
	pub Bitfield10:u8,

	/// NAME: HP Est Bottle / MP Est Bottle Recovery Number Parameter ID - HPエスト瓶／MPエスト瓶回復数パラメータID
	/// DESC: When the character dies, the data ID of the est usage count recovery parameter .xlsm, which is the same as the value, is acquired and the est bottle is recovered. Unused if -1 - キャラクター死亡時に値と同じ エスト使用回数回復パラメータ.xlsm　のデータIDを取得してエスト瓶を回復させる。 -1なら未使用
	pub estusFlaskRecoveryParamId:i16,

	/// NAME: Role name text ID - ロール名テキストID
	/// DESC: Specify the role name at the time of summoning. -1: Use the default role name of the target spirit body. 0: No display. 1 or more: Used as a text ID. - 召喚時のロール名を指定する。-1:対象霊体のデフォルトロール名を使用。0:表示なし。1以上:テキストＩＤとして利用。
	pub roleNameId:i32,

	/// NAME: HP & MP Est Bottle Recovery Lottery Probability - HP&MPエスト瓶回復 抽選確率
	/// DESC: HP / MP est recovery probability when defeating an enemy. The numerator is obtained from the NPC para with 10000 as the denominator. - 敵を倒した際のHP/MPエストの回復確率。10000 を分母とし、分子をNPCパラから取得する。 
	pub estusFlaskLotPoint:u16,

	/// NAME: HP Est Bottle Recovery Lottery Probability - HPエスト瓶回復 抽選確率
	/// DESC: Recovery probability of MP Est when defeating an enemy. The numerator is obtained from the NPC para with 10000 as the denominator. - 敵を倒した際のMPエストの回復確率。10000 を分母とし、分子をNPCパラから取得する。 
	pub hpEstusFlaskLotPoint:u16,

	/// NAME: MP Est Bottle Recovery Lottery Probability - MPエスト瓶回復 抽選確率
	/// DESC: Recovery probability of MP Est when defeating an enemy. The numerator is obtained from the NPC para with 10000 as the denominator. - 敵を倒した際のMPエストの回復確率。10000 を分母とし、分子をNPCパラから取得する。 
	pub mpEstusFlaskLotPoint:u16,

	/// NAME: HP & MP Est Bottle Recovery Addition lottery probability at the time of defeat - HP&MPエスト瓶回復 落選時 加算抽選確率
	/// DESC: Next time probability increase value when you miss the HP / MP est recovery lottery. Addition value of numerator. - HP/MPエスト回復抽選に外れた際の次回確率上昇値。分子の加算値。
	pub estusFlaskRecovery_failedLotPointAdd:u16,

	/// NAME: HP Est Bottle Recovery Addition lottery probability at the time of defeat - HPエスト瓶回復 落選時 加算抽選確率
	/// DESC: The next probability increase value when you miss the HP Est Recovery Lottery. Addition value of numerator. - HPエスト回復抽選に外れた際の次回確率上昇値。分子の加算値。
	pub hpEstusFlaskRecovery_failedLotPointAdd:u16,

	/// NAME: MP est bottle recovery Addition lottery probability at the time of defeat - MPエスト瓶回復 落選時 加算抽選確率
	/// DESC: The next probability increase value when the MP est recovery lottery is missed. Addition value of numerator. - MPエスト回復抽選に外れた際の次回確率上昇値。分子の加算値。
	pub mpEstusFlaskRecovery_failedLotPointAdd:u16,

	/// NAME: Will you be a wandering ghost using a phantom shader? - ファントムシェーダを使用して徘徊ゴーストになるか
	/// DESC: Phantom shader with ID specified only on the guest side Specify the phantom shader ID and make it an illusion - ゲスト側でだけ指定されたIDのファントムシェーダIDを指定して幻影化
	pub WanderGhostPhantomId:i32,

	/// NAME: Hearing head size [m] - 聴覚用 頭のサイズ[m]
	/// DESC: The offset size to be set instead of the capsule offset at the time of hearing judgment. Use this value as an offset only if it is set to 0 or higher. - 聴覚判定時のカプセルオフセットの代わりに、設定するオフセットサイズ。0以上が設定されている場合のみ、この値をオフセットとして使用。
	pub hearingHeadSize:f32,

	/// NAME: Sound bank ID - サウンドバンクID
	/// DESC: Sound bank ID can be specified -1: Use the bank of character ID (resource name) - サウンドバンクIDが指定できます -1：キャラID(リソース名)のバンクを使用
	pub SoundBankId:i16,

	/// NAME: Maximum anteroposterior angle to match undulations - 起伏にあわせる最大前後角度
	/// DESC: The upper limit angle when adjusting the front-back angle to the undulation. If the total length is long, it is better to set it lower. - 起伏に前後の角度を合わせる場合の上限角度。全長が長い場合には低めに設定したほうがよいです。
	pub forwardUndulationLimit:u8,

	/// NAME: Maximum left-right angle to match undulations - 起伏にあわせる最大左右角度
	/// DESC: Upper limit angle when adjusting the left and right angles to the undulations. If the total length is long, it is better to set it lower. - 起伏に左右の角度を合わせる場合の上限角度。全長が長い場合には低めに設定したほうがよいです。
	pub sideUndulationLimit:u8,

	/// NAME: Platoon Deactive Movement Speed [m / s] - 小隊ディアクティブ移動の移動速度[m/s]
	/// DESC: Platoon Deactive Movement Speed [m / s] - 小隊ディアクティブ移動の移動速度[m/s]
	pub deactiveMoveSpeed:f32,

	/// NAME: Distance to switch to platoon deactive movement [m] - 小隊ディアクティブ移動に切り替わる距離[m]
	/// DESC: Distance to switch to platoon deactive movement [m] - 小隊ディアクティブ移動に切り替わる距離[m]
	pub deactiveMoveDist:f32,

	/// NAME: Sound source effective distance [m] - サウンド音源有効距離[m]
	/// DESC: The distance from the player for which the character sound source is valid. -1: Effective at all distances - キャラ音源が有効なプレイヤーからの距離です。-1：全距離で有効
	pub enableSoundObjDist:f32,

	/// NAME: Correction gain value to match undulations - 起伏にあわせる補正ゲイン値
	/// DESC: Set the speed when adjusting the angle to the undulations - 起伏に角度を合わせる際の速度を設定する
	pub undulationCorrectGain:f32,

	/// NAME: Foot decal identifier 2 - フットデカール識別子2
	/// DESC: Decal to be attached when a foot effect occurs. Floor material is also taken into consideration - フットエフェクト発生時に貼られるデカール。床材質も考慮される
	pub autoFootEffectDecalBaseId2:i16,

	/// NAME: Foot decal identifier 3 - フットデカール識別子3
	/// DESC: Decal to be attached when a foot effect occurs. Floor material is also taken into consideration - フットエフェクト発生時に貼られるデカール。床材質も考慮される
	pub autoFootEffectDecalBaseId3:i16,

	/// NAME: Retarget reference character ID - リターゲット参照キャラID
	/// DESC: Character ID to be referred to when specifying the motion retarget destination - モーションのリターゲット先の指定の際に参照するキャラID
	pub RetargetReferenceChrId:i16,

	/// NAME: SFX resource bank ID - SFXリソースバンクID
	/// DESC: SFX resource bank ID can be specified -1: Use the bank of character ID (resource name) - SFXリソースバンクIDが指定できます -1：キャラID(リソース名)のバンクを使用
	pub SfxResBankId:i16,

	/// NAME: Update and activate priorities - 更新とアクティベイトの優先度
	/// DESC: Used to determine the activation / renewal level. The larger it is, the lower the update level will be even if you are far from the player. - アクティベート・更新レベルの決定に使用する。大きいほどプレイヤーから離れていても更新レベルが下がらない。
	pub updateActivatePriolity:f32,

	/// NAME: Pre-death navigation mesh flag - 死亡前ナビメッシュフラグ
	/// DESC: Flag the value of the touching Nav Mesh while the character is alive. Does not follow the movement. - キャラクターが生存してる間、触れているナビメッシュに値のフラグを設定する。移動に追従しない。
	pub chrNavimeshFlag_Alive:u8,

	/// NAME: Post-mortem navigation mesh flag - 死亡後ナビメッシュフラグ
	/// DESC: Flag the value on the touching Nav Mesh while the character is dying. Does not follow the movement. - キャラクターが死亡してる間、触れているナビメッシュに値のフラグを設定する。移動に追従しない。
	pub chrNavimeshFlag_Dead:u8,

	/// NAME: pad - pad
	pub pad7:[u8;1],

	/// NAME: Wheel control type - 車輪制御タイプ
	/// DESC: Wheel control type - 車輪制御タイプ
	pub wheelRotType:u8,

	/// NAME: Wheel radius - 車輪の半径
	/// DESC: Specify the radius of the wheel [m] - 車輪の半径を指定[m]
	pub wheelRotRadius:f32,

	/// NAME: Retarget movement amount magnification - リターゲット移動量倍率
	/// DESC: Magnification of movement amount at the time of retargeting - リターゲット時の移動量の倍率
	pub retargetMoveRate:f32,

	/// NAME: Ladder warp position offset - はしごワープ位置オフセット
	/// DESC: Offsets along the Damipoly Z-axis at the specified value. Both positive and negative numbers can be specified. - 指定された値でダミポリZ軸方向にオフセットします。正数・負数どちらも指定可能です。
	pub ladderWarpOffset:f32,

	/// NAME: Load asset ID - 読み込みアセットID
	/// DESC: Asset ID to be read in connection with character loading (for dynamic generation of characters, etc.). - キャラロード時に関連して読み込むアセットID（キャラが動的に生成するなど用。
	pub loadAssetId:i32,

	/// NAME: Overlap camera target lock Damipoli ID - オーバーラップカメラ対象ロックダミポリID
	/// DESC: Set the Damipoly ID (220-227) to enable the overlap camera. If it is -1, it will be invalid. - オーバーラップカメラを有効にするダミポリID(220～227)を設定します。-1の場合は無効になります。
	pub overlapCameraDmypolyId:i32,

	/// NAME: Resident Material Expansion Para ID0 - 常駐マテリアル拡張パラID0
	/// DESC: Resident Material Expansion Para ID0 - 常駐マテリアル拡張パラID0
	pub residentMaterialExParamId00:i32,

	/// NAME: Resident Material Expansion Para ID1 - 常駐マテリアル拡張パラID1
	/// DESC: Resident Material Expansion Para ID1 - 常駐マテリアル拡張パラID1
	pub residentMaterialExParamId01:i32,

	/// NAME: Resident Material Expansion Para ID2 - 常駐マテリアル拡張パラID2
	/// DESC: Resident Material Expansion Para ID2 - 常駐マテリアル拡張パラID2
	pub residentMaterialExParamId02:i32,

	/// NAME: Resident Material Expansion Para ID3 - 常駐マテリアル拡張パラID3
	/// DESC: Resident Material Expansion Para ID3 - 常駐マテリアル拡張パラID3
	pub residentMaterialExParamId03:i32,

	/// NAME: Resident Material Expansion Para ID4 - 常駐マテリアル拡張パラID4
	/// DESC: Resident Material Expansion Para ID4 - 常駐マテリアル拡張パラID4
	pub residentMaterialExParamId04:i32,

	/// NAME: Item lottery ID_for enemies - ネムリ時アイテム抽選ID_エネミー用
	/// DESC: Specify the lottery ID_for enemies of the item to be acquired when collecting Nemuri. Please set only one of them. - ネムリ収集時に取得するアイテムの抽選ID_エネミー用を指定。どちらか片方のみ設定してください。
	pub sleepCollectorItemLotId_enemy:i32,

	/// NAME: Nemuri item lottery ID_for map - ネムリ時アイテム抽選ID_マップ用
	/// DESC: Specify for the lottery ID_map of the item to be acquired when collecting Nemuri. Please set only one of them. - ネムリ収集時に取得するアイテムの抽選ID_マップ用を指定。どちらか片方のみ設定してください。
	pub sleepCollectorItemLotId_map:i32,

	/// NAME: FootIK Appearance height correction ON gain value - FootIK見た目の高さ補正ONゲイン値
	/// DESC: FootIK Appearance height correction ON gain value - FootIK見た目の高さ補正ONゲイン値
	pub footIkErrorOnGain:f32,

	/// NAME: FootIK Appearance height correction OFF gain value - FootIK見た目の高さ補正OFFゲイン値
	/// DESC: FootIK Appearance height correction OFF gain value - FootIK見た目の高さ補正OFFゲイン値
	pub footIkErrorOffGain:f32,

	/// NAME: Additional sound bank ID - 追加サウンドバンクID
	/// DESC: You can specify an additional sound bank ID -1 or 0: No addition (SEQ 16135) - 追加のサウンドバンクIDが指定できます -1 or 0：追加なし(SEQ16135)
	pub SoundAddBankId:i16,

	/// NAME: Defensive material variation value - 防御材質バリエーション値
	/// DESC: It is a value used in combination with the defense material to classify abnormal conditions, damage SFX, and SE. SEQ16473 - 防御材質と組み合わせて状態異常、ダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
	pub materialVariationValue:u8,

	/// NAME: Weakness defense material variation value - 弱点防御材質バリエーション値
	/// DESC: It is a value used for variation of abnormal condition, damage SFX, SE in combination with weak point defense material. SEQ16473 - 弱点防御材質と組み合わせて状態異常、ダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
	pub materialVariationValue_Weak:u8,

	/// NAME: SA endurance - SA耐久力
	/// DESC: Super armor durability value - スーパーアーマー耐久値
	pub superArmorDurability:f32,

	/// NAME: SA recovery speed correction value - SA回復速度補正値
	/// DESC: Correct the SA recovery speed by multiplying the SA basic recovery amount - SA基礎回復量に乗算してSA回復速度を補正する
	pub saRecoveryRate:f32,

	/// NAME: SA attack cut rate [%] - SA攻撃カット率[％]
	/// DESC: Cut rate of SA damage when guard is successful - ガード成功時のSＡダメージのカット率
	pub saGuardCutRate:f32,

	/// NAME: Bleeding resistance correction rule ID - 出血耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the setting value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_blood:i32,

	/// NAME: Curse resistance correction rule ID - 呪耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the set value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_curse:i32,

	/// NAME: Cold resistance correction rule ID - 冷気耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the setting value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_freeze:i32,

	/// NAME: Sleep tolerance correction rule ID - 睡眠耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the set value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_sleep:i32,

	/// NAME: Madness resistance correction rule ID - 発狂耐性 補正ルールID
	/// DESC: When the abnormal condition is activated, the maximum value is temporarily changed by using the set value of the abnormal condition resistance correction parameter. - 状態異常の発動時、状態異常耐性補正パラメータの設定値を使って、一時的に最大値を変動させる
	pub resistCorrectId_madness:i32,

	/// NAME: Character death tutorial judgment flag ID - キャラ死亡チュートリアル判定フラグID
	/// DESC: Event flag ID for the tutorial when the character is defeated for the first time. Flag ON when the character dies. - 初めてキャラ倒した時のチュートリアル用のイベントフラグID。キャラ死亡時にフラグON。
	pub chrDeadTutorialFlagId:u32,

	/// NAME: Step crossing display interpolation time - 段差越え表示補間時間
	/// DESC: Step crossing display interpolation time - 段差越え表示補間時間
	pub stepDispInterpolateTime:f32,

	/// NAME: Step crossing display activation judgment value - 段差越え表示起動判定値
	/// DESC: Step crossing display activation judgment value - 段差越え表示起動判定値
	pub stepDispInterpolateTriggerValue:f32,

	/// NAME: Lock score correction value - ロックスコア補正値
	/// DESC: Lock score correction value - ロックスコア補正値
	pub lockScoreOffset:f32,

	/// NAME: Padding 12 - パディング12
	pub pad12:[u8;8],
}

impl NPC_PARAM_ST {
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
	}	/// Does the camera hit the enemy ragdoll? (Valid only when hitting the player) - 敵のラグドールにカメラがあたるか。(プレイヤにも当たるときのみ有効)
	/// Bitfield2
	pub fn get_useRagdollCamHit(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_useRagdollCamHit(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// If you want to prevent the cross rigid from hitting you ○ - クロスリジッドが自分に当たらないようにしたければ○
	/// Bitfield2
	pub fn get_disableClothRigidHit(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_disableClothRigidHit(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Whether to use front and back undulation addition - 前後の起伏加算を使用するか
	/// Bitfield2
	pub fn get_useUndulationAddAnimFB(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_useUndulationAddAnimFB(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Special attack A? Special attack A damage multiplier will be included in the calculation - 特攻Aか。特攻Aダメージ倍率が計算に含まれるようになります
	/// Bitfield2
	pub fn get_isWeakA(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_isWeakA(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// The opponent's attack will be able to slip through. The attack hits only when the "anti-spirit weapon" of the weapon para is attacked with the weapon of ○. Be careful not to confuse it with a wandering ghost - 相手の攻撃がすり抜けるようになります。武器パラの「対霊武器」が○の武器で攻撃された時のみ攻撃が当たります。徘徊ゴーストと混同しないように注意
	/// Bitfield2
	pub fn get_isGhost(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_isGhost(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Do you not play the damage motion when the damage is 0? - ダメージ0のときにダメージモーションを再生しないか。
	/// Bitfield2
	pub fn get_isNoDamageMotion(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_isNoDamageMotion(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Do you match the back and forth rotation of the character with the undulations of the ground? Cannot be used for flying characters - キャラの前後回転を地面の起伏に合わせるか。飛行キャラの場合は使用不可
	/// Bitfield2
	pub fn get_isUnduration(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_isUnduration(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Will it be a wandering ghost when the player is a client? - プレイヤーがクライアントのときに徘徊ゴーストになるか
	/// Bitfield2
	pub fn get_isChangeWanderGhost(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_isChangeWanderGhost(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask0(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask0(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask1(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask1(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask2(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask2(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask3(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask3(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask4(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask4(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask5(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask5(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask6(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask6(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield3
	pub fn get_modelDispMask7(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_modelDispMask7(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask8(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask8(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask9(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask9(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask10(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask10(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask11(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask11(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask12(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask12(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask13(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask13(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask14(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask14(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield4
	pub fn get_modelDispMask15(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_modelDispMask15(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Do you want to enable the swing set in Param Weaver? - パラムウィーバで設定された首振りを有効にするか。
	/// Bitfield5
	pub fn get_isEnableNeckTurn(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_isEnableNeckTurn(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// Do you ban respawn? - リスポンを禁止するか
	/// Bitfield5
	pub fn get_disableRespawn(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_disableRespawn(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// Do you want to play the moving animation until the animation is over? (Like a mayfly dragon.) - 移動アニメをアニメが終わるまで再生するか。（カゲロウ龍の様に。）
	/// Bitfield5
	pub fn get_isMoveAnimWait(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_isMoveAnimWait(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// Do you want to reduce the processing load during crowds? For babies (preferably phalanx) - 群集時の処理負荷軽減を行なうか。赤子用（できればファランクスも）
	/// Bitfield5
	pub fn get_isCrowd(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_isCrowd(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// Special attack B? Special attack B damage multiplier will be included in the calculation - 特攻Bか。特攻Bダメージ倍率が計算に含まれるようになります
	/// Bitfield5
	pub fn get_isWeakB(&self) -> bool {
		&self.Bitfield5 & 0x10 != 0
	}

	/// Bitfield5
	pub fn set_isWeakB(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x10
		} else {
			self.Bitfield5 &= 0xEF
		}
	}
	/// Special attack C? Special attack C damage multiplier will be included in the calculation - 特攻Cか。特攻Cダメージ倍率が計算に含まれるようになります
	/// Bitfield5
	pub fn get_isWeakC(&self) -> bool {
		&self.Bitfield5 & 0x20 != 0
	}

	/// Bitfield5
	pub fn set_isWeakC(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x20
		} else {
			self.Bitfield5 &= 0xDF
		}
	}
	/// Special attack D? Special attack D damage multiplier will be included in the calculation - 特攻Dか。特攻Dダメージ倍率が計算に含まれるようになります
	/// Bitfield5
	pub fn get_isWeakD(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_isWeakD(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// Always execute a special turn (even if there is no navigation mesh at the turn destination, the special turn is continuously executed) - 常時特殊旋回を実行するか(旋回移動先にナビメッシュがない場合も特殊旋回を継続実行します)
	/// Bitfield5
	pub fn get_doesAlwaysUseSpecialTurn(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_doesAlwaysUseSpecialTurn(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Will you be the target of a riding special attack (if you are riding)? - （騎乗中であれば）騎乗特攻の対象になるか
	/// Bitfield6
	pub fn get_isRideAtkTarget(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_isRideAtkTarget(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Whether to use step-over display interpolation - 段差越え表示補間を使用するか
	/// Bitfield6
	pub fn get_isEnableStepDispInterpolate(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_isEnableStepDispInterpolate(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Is it a stealth attack target? - ステルス攻撃対象か
	/// Bitfield6
	pub fn get_isStealthTarget(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_isStealthTarget(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// If you do not make an initial death, TRUE, even if you kill and save it, the corpse will not be reproduced. - 初期死亡をしない場合にTRUE、殺してセーブしても死体再現されません。
	/// Bitfield6
	pub fn get_disableInitializeDead(&self) -> bool {
		&self.Bitfield6 & 0x8 != 0
	}

	/// Bitfield6
	pub fn set_disableInitializeDead(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x8
		} else {
			self.Bitfield6 &= 0xF7
		}
	}
	/// TRUE if it vibrates when hit. Use when you want to change from a normal hit stop, such as a dead person. - ヒット時振動をする場合TRUE。亡者など、普通のヒットストップと変えたいときにつかう。
	/// Bitfield6
	pub fn get_isHitRumble(&self) -> bool {
		&self.Bitfield6 & 0x10 != 0
	}

	/// Bitfield6
	pub fn set_isHitRumble(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x10
		} else {
			self.Bitfield6 &= 0xEF
		}
	}
	/// Whether to perform interpolation when turning between nodes in route movement - ルート移動でのノード間旋回時、補間を行うか否か
	/// Bitfield6
	pub fn get_isSmoothTurn(&self) -> bool {
		&self.Bitfield6 & 0x20 != 0
	}

	/// Bitfield6
	pub fn set_isSmoothTurn(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x20
		} else {
			self.Bitfield6 &= 0xDF
		}
	}
	/// Special attack E? Special attack E damage multiplier will be included in the calculation - 特攻Eか。特攻Eダメージ倍率が計算に含まれるようになります
	/// Bitfield6
	pub fn get_isWeakE(&self) -> bool {
		&self.Bitfield6 & 0x40 != 0
	}

	/// Bitfield6
	pub fn set_isWeakE(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x40
		} else {
			self.Bitfield6 &= 0xBF
		}
	}
	/// Special attack F? Special attack F damage multiplier will be included in the calculation - 特攻Fか。特攻Fダメージ倍率が計算に含まれるようになります
	/// Bitfield6
	pub fn get_isWeakF(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_isWeakF(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask16(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask16(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask17(&self) -> bool {
		&self.Bitfield7 & 0x2 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask17(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x2
		} else {
			self.Bitfield7 &= 0xFD
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask18(&self) -> bool {
		&self.Bitfield7 & 0x4 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask18(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x4
		} else {
			self.Bitfield7 &= 0xFB
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask19(&self) -> bool {
		&self.Bitfield7 & 0x8 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask19(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x8
		} else {
			self.Bitfield7 &= 0xF7
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask20(&self) -> bool {
		&self.Bitfield7 & 0x10 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask20(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x10
		} else {
			self.Bitfield7 &= 0xEF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask21(&self) -> bool {
		&self.Bitfield7 & 0x20 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask21(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x20
		} else {
			self.Bitfield7 &= 0xDF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask22(&self) -> bool {
		&self.Bitfield7 & 0x40 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask22(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x40
		} else {
			self.Bitfield7 &= 0xBF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield7
	pub fn get_modelDispMask23(&self) -> bool {
		&self.Bitfield7 & 0x80 != 0
	}

	/// Bitfield7
	pub fn set_modelDispMask23(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x80
		} else {
			self.Bitfield7 &= 0x7F
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask24(&self) -> bool {
		&self.Bitfield8 & 0x1 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask24(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x1
		} else {
			self.Bitfield8 &= 0xFE
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask25(&self) -> bool {
		&self.Bitfield8 & 0x2 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask25(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x2
		} else {
			self.Bitfield8 &= 0xFD
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask26(&self) -> bool {
		&self.Bitfield8 & 0x4 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask26(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x4
		} else {
			self.Bitfield8 &= 0xFB
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask27(&self) -> bool {
		&self.Bitfield8 & 0x8 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask27(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x8
		} else {
			self.Bitfield8 &= 0xF7
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask28(&self) -> bool {
		&self.Bitfield8 & 0x10 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask28(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x10
		} else {
			self.Bitfield8 &= 0xEF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask29(&self) -> bool {
		&self.Bitfield8 & 0x20 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask29(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x20
		} else {
			self.Bitfield8 &= 0xDF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask30(&self) -> bool {
		&self.Bitfield8 & 0x40 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask30(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x40
		} else {
			self.Bitfield8 &= 0xBF
		}
	}
	/// Displays the model corresponding to the display mask. - 表示マスクに対応するモデルを表示します。
	/// Bitfield8
	pub fn get_modelDispMask31(&self) -> bool {
		&self.Bitfield8 & 0x80 != 0
	}

	/// Bitfield8
	pub fn set_modelDispMask31(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x80
		} else {
			self.Bitfield8 &= 0x7F
		}
	}
	/// Do you get a boss in soul? - ソウルはボス入手か
	/// Bitfield9
	pub fn get_isSoulGetByBoss(&self) -> bool {
		&self.Bitfield9 & 0x1 != 0
	}

	/// Bitfield9
	pub fn set_isSoulGetByBoss(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x1
		} else {
			self.Bitfield9 &= 0xFE
		}
	}
	/// If you become the owner of a bullet, a flag that applies the object's damage calculation related to the bullet. Used for damage correction by power. - 弾丸のオーナーとなった場合、弾丸に関連するダメージ計算などをオブジェのものを適用するようにするフラグ。勢力別ダメージ補正で使用。
	/// Bitfield9
	pub fn get_isBulletOwner_byObject(&self) -> bool {
		&self.Bitfield9 & 0x2 != 0
	}

	/// Bitfield9
	pub fn set_isBulletOwner_byObject(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x2
		} else {
			self.Bitfield9 &= 0xFD
		}
	}
	/// Whether to use the FootIk filter for low hits - ロウヒット用のFootIkフィルターを使用するか
	/// Bitfield9
	pub fn get_isUseLowHitFootIk(&self) -> bool {
		&self.Bitfield9 & 0x4 != 0
	}

	/// Bitfield9
	pub fn set_isUseLowHitFootIk(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x4
		} else {
			self.Bitfield9 &= 0xFB
		}
	}
	/// Decide whether to calculate damage as a "player" when calculating damage. If it is invalid, it is treated as an "enemy". - ダメージ計算時に「プレイヤー」としてダメージ計算するのかを決める。無効の場合は「敵」扱い。
	/// Bitfield9
	pub fn get_isCalculatePvPDamage(&self) -> bool {
		&self.Bitfield9 & 0x8 != 0
	}

	/// Bitfield9
	pub fn set_isCalculatePvPDamage(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x8
		} else {
			self.Bitfield9 &= 0xF7
		}
	}
	/// Can only be activated when active in the host world - ホスト世界でアクティブ時のみアクティベート可
	/// Bitfield9
	pub fn get_isHostSyncChr(&self) -> bool {
		&self.Bitfield9 & 0x10 != 0
	}

	/// Bitfield9
	pub fn set_isHostSyncChr(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x10
		} else {
			self.Bitfield9 &= 0xEF
		}
	}
	/// Weakness damage Whether to skip animation playback. "Part damage rate" and "defense material" are treated as weak points just by not playing the animation. - 弱点ダメージアニメ再生をスキップするかどうか。アニメを再生しないだけで「部位ダメージ率」「防御材質」は弱点として扱われます。
	/// Bitfield9
	pub fn get_isSkipWeakDamageAnim(&self) -> bool {
		&self.Bitfield9 & 0x20 != 0
	}

	/// Bitfield9
	pub fn set_isSkipWeakDamageAnim(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x20
		} else {
			self.Bitfield9 &= 0xDF
		}
	}
	/// When riding on a character with this parameter ○, the character's Atari remains while riding. - このパラメータが○のキャラに騎乗する際、騎乗中、キャラのアタリが残ったままになる 
	/// Bitfield9
	pub fn get_isKeepHitOnRide(&self) -> bool {
		&self.Bitfield9 & 0x40 != 0
	}

	/// Bitfield9
	pub fn set_isKeepHitOnRide(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x40
		} else {
			self.Bitfield9 &= 0xBF
		}
	}
	/// Is it a special character? - 特殊あたりに当たるキャラか
	/// Bitfield9
	pub fn get_isSpCollide(&self) -> bool {
		&self.Bitfield9 & 0x80 != 0
	}

	/// Bitfield9
	pub fn set_isSpCollide(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x80
		} else {
			self.Bitfield9 &= 0x7F
		}
	}
	/// Do you use the FootIK result to match your character to the undulations of the ground? Cannot be used for flying characters - キャラを地面の起伏に合わせる際に、FootIK結果を用いるか。飛行キャラの場合は使用不可
	/// Bitfield10
	pub fn get_isUseFootIKNormalByUnduration(&self) -> bool {
		&self.Bitfield10 & 0x1 != 0
	}

	/// Bitfield10
	pub fn set_isUseFootIKNormalByUnduration(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x1
		} else {
			self.Bitfield10 &= 0xFE
		}
	}
	/// Whether to ground the capsule at the time of initial death - 初期死亡時にカプセル接地するか
	/// Bitfield10
	pub fn get_attachHitInitializeDead(&self) -> bool {
		&self.Bitfield10 & 0x2 != 0
	}

	/// Bitfield10
	pub fn set_attachHitInitializeDead(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x2
		} else {
			self.Bitfield10 &= 0xFD
		}
	}
	/// Group reward: In the judgment of "all dead", characters with this parameter ○ are excluded from the judgment. - グループ報酬：「全員死亡」の判定において、このパラメータが○のキャラは判定から除外する 
	/// Bitfield10
	pub fn get_excludeGroupRewardCheck(&self) -> bool {
		&self.Bitfield10 & 0x4 != 0
	}

	/// Bitfield10
	pub fn set_excludeGroupRewardCheck(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x4
		} else {
			self.Bitfield10 &= 0xFB
		}
	}
	/// Is Rock Damipoli (for Enemy) 212 Effective? - ロックダミポリ(エネミー用)212が有効か
	/// Bitfield10
	pub fn get_enableAILockDmyPoly_212(&self) -> bool {
		&self.Bitfield10 & 0x8 != 0
	}

	/// Bitfield10
	pub fn set_enableAILockDmyPoly_212(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x8
		} else {
			self.Bitfield10 &= 0xF7
		}
	}
	/// Is Rock Damipoli (for Enemy) 213 Effective? - ロックダミポリ(エネミー用)213が有効か
	/// Bitfield10
	pub fn get_enableAILockDmyPoly_213(&self) -> bool {
		&self.Bitfield10 & 0x10 != 0
	}

	/// Bitfield10
	pub fn set_enableAILockDmyPoly_213(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x10
		} else {
			self.Bitfield10 &= 0xEF
		}
	}
	/// Is Rock Damipoli (for Enemy) 214 Effective? - ロックダミポリ(エネミー用)214が有効か
	/// Bitfield10
	pub fn get_enableAILockDmyPoly_214(&self) -> bool {
		&self.Bitfield10 & 0x20 != 0
	}

	/// Bitfield10
	pub fn set_enableAILockDmyPoly_214(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x20
		} else {
			self.Bitfield10 &= 0xDF
		}
	}
	/// Excluded from open_XB1 - オープン_XB1から除外
	/// Bitfield10
	pub fn get_disableActivateOpen_xb1(&self) -> bool {
		&self.Bitfield10 & 0x40 != 0
	}

	/// Bitfield10
	pub fn set_disableActivateOpen_xb1(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x40
		} else {
			self.Bitfield10 &= 0xBF
		}
	}
	/// Excluded from Legacy_XB1 - レガシー_XB1から除外
	/// Bitfield10
	pub fn get_disableActivateLegacy_xb1(&self) -> bool {
		&self.Bitfield10 & 0x80 != 0
	}

	/// Bitfield10
	pub fn set_disableActivateLegacy_xb1(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x80
		} else {
			self.Bitfield10 &= 0x7F
		}
	}

}
