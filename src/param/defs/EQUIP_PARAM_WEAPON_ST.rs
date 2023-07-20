/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 6
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_PARAM_WEAPON_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Behavior variation ID - 行動バリエーションID
	/// DESC: Used when determining the action parameter ID to be referenced during an attack - 攻撃時に参照する行動パラメータIDを決定するときに使う
	pub behaviorVariationId:i32,

	/// NAME: Sort ID - ソートID
	/// DESC: Sort ID (-1: Do not collect) (7 digits is the limit in s32 to add the enhancement level in the program) - ソートID(-1:集めない)(プログラム内で強化レベルを加味するため s32 では７桁が限界)
	pub sortId:i32,

	/// NAME: Wandering equipment ID - 徘徊装備ID
	/// DESC: Replacement equipment ID for wandering ghosts. - 徘徊ゴースト用の差し替え装備ID.
	pub wanderingEquipId:u32,

	/// NAME: Weight [kg] - 重量[kg]
	/// DESC: Weight [kg]. - 重量[kg].
	pub weight:f32,

	/// NAME: Equipment weight ratio - 装備重量比率
	/// DESC: Equipment weight ratio - 装備重量比率
	pub weaponWeightRate:f32,

	/// NAME: Repair price - 修理価格
	/// DESC: Basic repair price - 修理基本価格
	pub fixPrice:i32,

	/// NAME: Enhanced price - 強化価格
	/// DESC: Enhanced price - 強化価格
	pub reinforcePrice:i32,

	/// NAME: Sale price - 売却価格
	/// DESC: Selling price - 販売価格
	pub sellValue:i32,

	/// NAME: Strength correction - 筋力補正
	/// DESC: Charapara correction value. - キャラパラ補正値.
	pub correctStrength:f32,

	/// NAME: Agility correction - 俊敏補正
	/// DESC: Charapara correction value. - キャラパラ補正値.
	pub correctAgility:f32,

	/// NAME: Magic correction - 魔力補正
	/// DESC: Charapara correction value. - キャラパラ補正値.
	pub correctMagic:f32,

	/// NAME: Faith correction - 信仰補正
	/// DESC: Charapara correction value. - キャラパラ補正値.
	pub correctFaith:f32,

	/// NAME: Physical attack cut rate when guarding - ガード時物理攻撃カット率
	/// DESC: Set the damage cut rate when guarding for each attack - ガード時のダメージカット率を各攻撃ごとに設定
	pub physGuardCutRate:f32,

	/// NAME: Magic attack cut rate when guarding - ガード時魔法攻撃カット率
	/// DESC: If it is not a guard attack, enter 0 - ガード攻撃でない場合は、0を入れる
	pub magGuardCutRate:f32,

	/// NAME: Flame attack power cut rate when guarding - ガード時炎攻撃力カット率
	/// DESC: How much to cut the fire attack? - 炎攻撃をどれだけカットするか？
	pub fireGuardCutRate:f32,

	/// NAME: Electric shock attack power cut rate when guarding - ガード時電撃攻撃力カット率
	/// DESC: How much to cut the electric shock attack? - 電撃攻撃をどれだけカットするか？
	pub thunGuardCutRate:f32,

	/// NAME: Special effect ID 0 on attack hit - 攻撃ヒット時特殊効果ID0
	/// DESC: Register when adding special effects to weapons - 武器に特殊効果を追加するときに登録する
	pub spEffectBehaviorId0:i32,

	/// NAME: Special effect ID1 on attack hit - 攻撃ヒット時特殊効果ID1
	/// DESC: Register when adding special effects to weapons - 武器に特殊効果を追加するときに登録する
	pub spEffectBehaviorId1:i32,

	/// NAME: Special effect ID2 on attack hit - 攻撃ヒット時特殊効果ID2
	/// DESC: Register when adding special effects to weapons - 武器に特殊効果を追加するときに登録する
	pub spEffectBehaviorId2:i32,

	/// NAME: Resident special effect ID - 常駐特殊効果ID
	/// DESC: Resident special effect ID0 - 常駐特殊効果ID0
	pub residentSpEffectId:i32,

	/// NAME: Resident special effect ID1 - 常駐特殊効果ID1
	/// DESC: Resident special effect ID1 - 常駐特殊効果ID1
	pub residentSpEffectId1:i32,

	/// NAME: Resident special effect ID2 - 常駐特殊効果ID2
	/// DESC: Resident special effect ID2 - 常駐特殊効果ID2
	pub residentSpEffectId2:i32,

	/// NAME: Material ID - 素材ID
	/// DESC: Material parameter ID required for weapon enhancement - 武器強化に必要な素材パラメータID
	pub materialSetId:i32,

	/// NAME: Derivation source - 派生元
	/// DESC: This weapon's enhancement source weapon ID - この武器の強化元武器ID
	pub originEquipWep:i32,

	/// NAME: Derivative source enhancement +1 - 派生元 強化+1
	/// DESC: This weapon's enhancement source weapon ID1 - この武器の強化元武器ID1
	pub originEquipWep1:i32,

	/// NAME: Derivative source enhancement +2 - 派生元 強化+2
	/// DESC: This weapon's enhancement source weapon ID2 - この武器の強化元武器ID2
	pub originEquipWep2:i32,

	/// NAME: Derivative source enhancement +3 - 派生元 強化+3
	/// DESC: This weapon's enhancement source weapon ID3 - この武器の強化元武器ID3
	pub originEquipWep3:i32,

	/// NAME: Derivative source enhancement +4 - 派生元 強化+4
	/// DESC: This weapon's enhancement source weapon ID 4 - この武器の強化元武器ID4
	pub originEquipWep4:i32,

	/// NAME: Derivative source enhancement +5 - 派生元 強化+5
	/// DESC: This weapon's enhancement source weapon ID 5 - この武器の強化元武器ID5
	pub originEquipWep5:i32,

	/// NAME: Derivative source enhancement +6 - 派生元 強化+6
	/// DESC: This weapon's enhancement source weapon ID 6 - この武器の強化元武器ID6
	pub originEquipWep6:i32,

	/// NAME: Derivative source enhancement +7 - 派生元 強化+7
	/// DESC: This weapon's enhancement source weapon ID 7 - この武器の強化元武器ID7
	pub originEquipWep7:i32,

	/// NAME: Derivative source enhancement +8 - 派生元 強化+8
	/// DESC: This weapon's enhancement source weapon ID8 - この武器の強化元武器ID8
	pub originEquipWep8:i32,

	/// NAME: Derivative source enhancement +9 - 派生元 強化+9
	/// DESC: This weapon's enhancement source weapon ID 9 - この武器の強化元武器ID9
	pub originEquipWep9:i32,

	/// NAME: Derivative source enhancement +10 - 派生元 強化+10
	/// DESC: This weapon's enhancement source weapon ID 10 - この武器の強化元武器ID10
	pub originEquipWep10:i32,

	/// NAME: Derivative source enhancement +11 - 派生元 強化+11
	/// DESC: This weapon's enhancement source weapon ID 11 - この武器の強化元武器ID11
	pub originEquipWep11:i32,

	/// NAME: Derivative source enhancement +12 - 派生元 強化+12
	/// DESC: This weapon's enhancement source weapon ID 12 - この武器の強化元武器ID12
	pub originEquipWep12:i32,

	/// NAME: Derivative source enhancement +13 - 派生元 強化+13
	/// DESC: This weapon's enhancement source weapon ID 13 - この武器の強化元武器ID13
	pub originEquipWep13:i32,

	/// NAME: Derivative source enhancement +14 - 派生元 強化+14
	/// DESC: This weapon's enhancement source weapon ID14 - この武器の強化元武器ID14
	pub originEquipWep14:i32,

	/// NAME: Derivative source enhancement +15 - 派生元 強化+15
	/// DESC: This weapon's enhancement source weapon ID 15 - この武器の強化元武器ID15
	pub originEquipWep15:i32,

	/// NAME: Special attack A damage multiplier - 特攻Aダメージ倍率
	/// DESC: Damage multiplier for special attack A - 特攻A用のダメージ倍率
	pub weakA_DamageRate:f32,

	/// NAME: Special attack B damage multiplier - 特攻Bダメージ倍率
	/// DESC: Damage multiplier for special attack B - 特攻B用のダメージ倍率
	pub weakB_DamageRate:f32,

	/// NAME: Special attack C damage multiplier - 特攻Cダメージ倍率
	/// DESC: Damage multiplier for special attack C - 特攻C用のダメージ倍率
	pub weakC_DamageRate:f32,

	/// NAME: Special attack D damage multiplier - 特攻Dダメージ倍率
	/// DESC: Damage multiplier for special attack D - 特攻D用のダメージ倍率
	pub weakD_DamageRate:f32,

	/// NAME: Sleep tolerance cut rate_maximum correction value - 睡眠耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against sleep (set as special effect parameter) - 睡眠に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub sleepGuardResist_MaxCorrect:f32,

	/// NAME: Madness resistance cut rate_maximum correction value - 発狂耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against madness (set as special effect parameter) - 発狂に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub madnessGuardResist_MaxCorrect:f32,

	/// NAME: SA weapon attack power - SA武器攻撃力
	/// DESC: Super Armor Basic Attack Power - スーパーアーマー基本攻撃力
	pub saWeaponDamage:f32,

	/// NAME: Equipment model number - 装備モデル番号
	/// DESC: Equipment model number. - 装備モデルの番号.
	pub equipModelId:u16,

	/// NAME: Icon ID - アイコンID
	/// DESC: Menu icon ID. - メニューアイコンID.
	pub iconId:u16,

	/// NAME: Durability - 耐久度
	/// DESC: Initial durability. - 初期耐久度.
	pub durability:u16,

	/// NAME: Maximum durability - 耐久度最大値
	/// DESC: New durability. - 新品耐久度.
	pub durabilityMax:u16,

	/// NAME: Throw-through attack power basic value - 投げ抜け攻撃力基本値
	/// DESC: Basic value of throw-through attack power - 投げ抜け攻撃力の基本値
	pub attackThrowEscape:u16,

	/// NAME: Parry occurrence time [frame] - パリィ発生時間[frame]
	/// DESC: Limit the life of parry damage. It does not last longer than it is set in TimeAct. - パリィダメージの寿命を制限する。TimeActで設定されている以上には持続しない。
	pub parryDamageLife:i16,

	/// NAME: Physical attack power basic value - 物理攻撃力基本値
	/// DESC: Basic value of physical attribute attack that damages the enemy's HP - 敵のＨＰにダメージを与える物理属性攻撃の基本値
	pub attackBasePhysics:u16,

	/// NAME: Magic attack power basic value - 魔法攻撃力基本値
	/// DESC: Basic value of magic attribute attack that damages the enemy's HP - 敵のＨＰにダメージを与える魔法属性攻撃の基本値
	pub attackBaseMagic:u16,

	/// NAME: Fire attack power basic value - 炎攻撃力基本値
	/// DESC: Basic value of fire attribute attack that damages the enemy's HP - 敵のＨＰにダメージを与える炎属性攻撃の基本値
	pub attackBaseFire:u16,

	/// NAME: Electric shock attack power basic value - 電撃攻撃力基本値
	/// DESC: Basic value of electric shock attribute attack that damages the enemy's HP - 敵のＨＰにダメージを与える電撃属性攻撃の基本値
	pub attackBaseThunder:u16,

	/// NAME: Stamina attack power - スタミナ攻撃力
	/// DESC: Stamina attack power against the enemy - 敵へのスタミナ攻撃力
	pub attackBaseStamina:u16,

	/// NAME: Guard range [deg] - ガード範囲[deg]
	/// DESC: Defense occurrence range angle when guarding weapons - 武器のガード時の防御発生範囲角度
	pub guardAngle:i16,

	/// NAME: SA durability value - SA耐久値
	/// DESC: Additional SA durability used during attack motion - 攻撃モーション中に使われる追加SA耐久値
	pub saDurability:f32,

	/// NAME: Stamina defense when guarding - ガード時スタミナ防御力
	/// DESC: Defense against enemy stamina attacks when guarding successfully - ガード成功時に、敵のスタミナ攻撃に対する防御力
	pub staminaGuardDef:i16,

	/// NAME: Enhanced type ID - 強化タイプID
	/// DESC: Enhanced type ID - 強化タイプID
	pub reinforceTypeId:i16,

	/// NAME: Trophy S grade ID - トロフィーＳグレードID
	/// DESC: Is it related to the trophy system? - トロフィーシステムに関係あるか？
	pub trophySGradeId:i16,

	/// NAME: Trophy SEQ number - トロフィーSEQ番号
	/// DESC: Trophy SEQ number (13-29) - トロフィーのSEQ番号（１３～２９）
	pub trophySeqId:i16,

	/// NAME: Throw attack power multiplier - 投げ攻撃力倍率
	/// DESC: Throw attack power multiplier - 投げの攻撃力倍率
	pub throwAtkRate:i16,

	/// NAME: Bow distance correction [%] - 弓飛距離補正[％]
	/// DESC: Up% to extend the flight distance - 飛距離を伸ばすアップ％
	pub bowDistRate:i16,

	/// NAME: Equipment model type - 装備モデル種別
	/// DESC: Equipment model type. - 装備モデルの種別.
	pub equipModelCategory:u8,

	/// NAME: Equipment model gender - 装備モデル性別
	/// DESC: Gender of equipment model. - 装備モデルの性別.
	pub equipModelGender:u8,

	/// NAME: Weapon category - 武器カテゴリ
	/// DESC: Weapon category. - 武器のカテゴリ.
	pub weaponCategory:u8,

	/// NAME: Weapon motion category - 武器モーションカテゴリ
	/// DESC: Weapon motion category. - 武器モーションのカテゴリ.
	pub wepmotionCategory:u8,

	/// NAME: Guard motion category - ガードモーションカテゴリ
	/// DESC: Guard motion category - ガードモーションのカテゴリ
	pub guardmotionCategory:u8,

	/// NAME: Attack material - 攻撃材質
	/// DESC: Attack material used from attack para - 攻撃パラから使用される攻撃材質
	pub atkMaterial:u8,

	/// NAME: Defense SE Material 1 - 防御SE材質1
	/// DESC: Defense SE material used from attack para 1 - 攻撃パラから使用される防御SE材質1
	pub defSeMaterial1:u16,

	/// NAME: Correction type (physical attack power) - 補正タイプ（物理攻撃力）
	/// DESC: Correcting physical attack power by primary parameters Determines the type of graph - 一次パラメータによる物理攻撃力の補正グラフのタイプを決める
	pub correctType_Physics:u8,

	/// NAME: Special attributes - 特殊属性
	/// DESC: Weapon special attribute value - 武器の特殊属性値
	pub spAttribute:u8,

	/// NAME: Special attack category - 特殊攻撃カテゴリ
	/// DESC: Special attack category (possible from 50 to 999) - 特殊攻撃カテゴリ（50～999まで可能)
	pub spAtkcategory:u16,

	/// NAME: Weapon motion one-handed ID - 武器モーション片手ID
	/// DESC: Basic motion ID when equipped with one hand. - 片手装備時の基本モーションID.
	pub wepmotionOneHandId:u8,

	/// NAME: Weapon motion two-handed ID - 武器モーション両手ID
	/// DESC: Basic motion ID when equipped with both hands. - 両手装備時の基本モーションID.
	pub wepmotionBothHandId:u8,

	/// NAME: Equipment proper strength - 装備適正筋力
	/// DESC: Equipment appropriate value. - 装備適正値.
	pub properStrength:u8,

	/// NAME: Equipment proper agility - 装備適正俊敏
	/// DESC: Equipment appropriate value. - 装備適正値.
	pub properAgility:u8,

	/// NAME: Equipment proper magic power - 装備適正魔力
	/// DESC: Equipment appropriate value. - 装備適正値.
	pub properMagic:u8,

	/// NAME: Equipment proper faith - 装備適正信仰
	/// DESC: Equipment appropriate value. - 装備適正値.
	pub properFaith:u8,

	/// NAME: Strength over start value - 筋力オーバー開始値
	/// DESC: Strength over start value - 筋力オーバー開始値
	pub overStrength:u8,

	/// NAME: Parry attack base value - パリィ攻撃基本値
	/// DESC: Basic value to defeat the enemy's parry - 敵のパリィをやぶるための基本値
	pub attackBaseParry:u8,

	/// NAME: Parry defense value - パリィ防御値
	/// DESC: Used to judge whether to be a parry or a guard at the time of parry judgment - パリィ判定時に、パリィになるかガードになるかの判定に利用
	pub defenseBaseParry:u8,

	/// NAME: Flick defense basic value - はじき防御力基本値
	/// DESC: Used to determine if it will pop when guarding an enemy attack - 敵の攻撃をガードしたときに、はじけるかどうかの判定に利用
	pub guardBaseRepel:u8,

	/// NAME: Flick attack power basic value - はじき攻撃力基本値
	/// DESC: Guard Used to determine whether or not to be repelled when attacking an enemy - ガード敵を攻撃した時に、はじかれるかどうかの判定に利用
	pub attackBaseRepel:u8,

	/// NAME: Guard cut invalidation magnification - ガードカット無効化倍率
	/// DESC: Magnification that invalidates the opponent's guard cut. -100 is completely invalid. Double the defense effect of the opponent at 100. - 相手のガードカットを無効化させる倍率。-100で完全無効。100で相手の防御効果倍増。
	pub guardCutCancelRate:i8,

	/// NAME: Guard level - ガードレベル
	/// DESC: When guarding, which guard motion will the enemy attack? Decide - ガードしたとき、敵の攻撃をどのガードモーションで受けるか？を決める
	pub guardLevel:i8,

	/// NAME: Slash attack cut rate - 斬撃攻撃カット率
	/// DESC: Looking at the attack type, what percentage of the damage of the slashing attribute is cut? Specify - 攻撃タイプを見て、斬撃属性のダメージを何％カットするか？を指定
	pub slashGuardCutRate:i8,

	/// NAME: Batter attack cut rate - 打撃攻撃カット率
	/// DESC: Looking at the attack type, what percentage of the damage of the hit attribute is cut? Specify - 攻撃タイプを見て、打撃属性のダメージを何％カットするか？を指定
	pub blowGuardCutRate:i8,

	/// NAME: Puncture attack cut rate - 刺突攻撃カット率
	/// DESC: Looking at the attack type, what percentage of the damage of the piercing attribute is cut? Specify - 攻撃タイプを見て、刺突属性のダメージを何％カットするか？を指定
	pub thrustGuardCutRate:i8,

	/// NAME: Poison resistance cut rate - 毒耐性カット率
	/// DESC: How much to cut the attack power to poison (set to the special effect parameter) - 毒にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub poisonGuardResist:i8,

	/// NAME: Epidemic attack cut rate - 疫病攻撃カット率
	/// DESC: How much to cut the attack power (set as a special effect parameter) to make it a plague - 疫病にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub diseaseGuardResist:i8,

	/// NAME: Bleeding attack cut rate - 出血攻撃カット率
	/// DESC: How much to cut the attack power (set as a special effect parameter) to make bleeding - 出血にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub bloodGuardResist:i8,

	/// NAME: Curse attack cut rate - 呪攻撃カット率
	/// DESC: How much to cut the attack power (set as a special effect parameter) to curse - 呪いにする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub curseGuardResist:i8,

	/// NAME: Physical attribute 1 - 物理属性1
	/// DESC: Physical attribute 1 - 物理属性1
	pub atkAttribute:u8,

	/// NAME: Right hand equipment - 右手装備
	/// DESC: Is it possible to equip it with the right hand? - 右手装備可能か.
	pub Bitfield2:u8,

	/// NAME: Can be spelled - 呪術可能
	/// DESC: Cast magic when attacking - 攻撃時に呪術発動
	pub Bitfield3:u8,

	/// NAME: Is there a humanity correction? - 人間性補正あるか
	/// DESC: Is there an attack power correction by human nature? - 人間性による攻撃力補正があるか
	pub Bitfield4:u8,

	/// NAME: Weapon career change category - 武器転職カテゴリ
	/// DESC: Weapon career change category. Used to display the attribute icon. - 武器転職カテゴリ。属性アイコン表示に使用します。
	pub Bitfield5:u8,

	/// NAME: Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
	/// DESC: Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
	pub Bitfield6:u8,

	/// NAME: Defensive SFX Material 1 - 防御SFX材質1
	/// DESC: Defensive SFX material used from attack para 1 - 攻撃パラから使用される防御SFX材質1
	pub defSfxMaterial1:u16,

	/// NAME: Weapon collaborative setting - 武器コライダブル設定
	/// DESC: Weapon collaborative setting - 武器のコライダブル設定
	pub wepCollidableType0:u8,

	/// NAME: Weapon 1 collaborative setting - 武器1コライダブル設定
	/// DESC: Weapon 1 collaborative setting - 武器1のコライダブル設定
	pub wepCollidableType1:u8,

	/// NAME: Attitude control ID (right hand) - 姿勢制御ID(右手)
	/// DESC: Attitude control ID (right hand) - 姿勢制御ID(右手)
	pub postureControlId_Right:u8,

	/// NAME: Attitude control ID (left hand) - 姿勢制御ID(左手)
	/// DESC: Attitude control ID (left hand) - 姿勢制御ID(左手)
	pub postureControlId_Left:u8,

	/// NAME: Sword Flash SfxID_0 - 剣閃SfxID_０
	/// DESC: Sword flash SfxID_0 (-1 invalid) - 剣閃SfxID_０(-1無効)
	pub traceSfxId0:i32,

	/// NAME: Root Sword Flash Damipoli ID_0 - 根元剣閃ダミポリID_０
	/// DESC: Sword flash root Damipoli ID_0 (-1 invalid) - 剣閃根元ダミポリID_０(-1無効)
	pub traceDmyIdHead0:i32,

	/// NAME: Sword tip sword flash Damipoli ID_0 - 剣先剣閃ダミポリID_０
	/// DESC: Sword Flash Sword Tip Damipoli ID_0 - 剣閃剣先ダミポリID_０
	pub traceDmyIdTail0:i32,

	/// NAME: Sword Flash SfxID_1 - 剣閃SfxID_１
	/// DESC: Sword flash SfxID_1 (-1 invalid) - 剣閃SfxID_１(-1無効)
	pub traceSfxId1:i32,

	/// NAME: Root Sword Flash Damipoli ID_1 - 根元剣閃ダミポリID_１
	/// DESC: Sword Flash Root Damipoli ID_1 (-1 invalid) - 剣閃根元ダミポリID_１(-1無効)
	pub traceDmyIdHead1:i32,

	/// NAME: Sword tip sword flash Damipoli ID_1 - 剣先剣閃ダミポリID_１
	/// DESC: Sword Flash Sword Tip Damipoli ID_1 - 剣閃剣先ダミポリID_１
	pub traceDmyIdTail1:i32,

	/// NAME: Sword Flash SfxID_2 - 剣閃SfxID_２
	/// DESC: Sword flash SfxID_2 (-1 invalid) - 剣閃SfxID_２(-1無効)
	pub traceSfxId2:i32,

	/// NAME: Root Sword Flash Damipoli ID_2 - 根元剣閃ダミポリID_２
	/// DESC: Sword Flash Root Damipoli ID_2 (-1 invalid) - 剣閃根元ダミポリID_２(-1無効)
	pub traceDmyIdHead2:i32,

	/// NAME: Sword tip sword flash Damipoli ID_2 - 剣先剣閃ダミポリID_２
	/// DESC: Sword Flash Sword Tip Damipoli ID_2 - 剣閃剣先ダミポリID_２
	pub traceDmyIdTail2:i32,

	/// NAME: Sword Flash SfxID_3 - 剣閃SfxID_３
	/// DESC: Sword Flash SfxID_3 (-1 invalid) - 剣閃SfxID_３(-1無効)
	pub traceSfxId3:i32,

	/// NAME: Root Sword Flash Damipoli ID_3 - 根元剣閃ダミポリID_３
	/// DESC: Sword flash root Damipoli ID_3 (-1 invalid) - 剣閃根元ダミポリID_３(-1無効)
	pub traceDmyIdHead3:i32,

	/// NAME: Sword tip sword flash Damipoli ID_3 - 剣先剣閃ダミポリID_３
	/// DESC: Sword Flash Sword Tip Damipoli ID_3 - 剣閃剣先ダミポリID_３
	pub traceDmyIdTail3:i32,

	/// NAME: Sword Flash SfxID_4 - 剣閃SfxID_４
	/// DESC: Sword Flash SfxID_4 (-1 invalid) - 剣閃SfxID_４(-1無効)
	pub traceSfxId4:i32,

	/// NAME: Root Sword Flash Damipoli ID_4 - 根元剣閃ダミポリID_４
	/// DESC: Sword flash root Damipoli ID_4 (-1 invalid) - 剣閃根元ダミポリID_４(-1無効)
	pub traceDmyIdHead4:i32,

	/// NAME: Sword tip sword flash Damipoli ID_4 - 剣先剣閃ダミポリID_４
	/// DESC: Sword Flash Sword Tip Damipoli ID_4 - 剣閃剣先ダミポリID_４
	pub traceDmyIdTail4:i32,

	/// NAME: Sword Flash SfxID_5 - 剣閃SfxID_５
	/// DESC: Sword Flash SfxID_5 (-1 invalid) - 剣閃SfxID_５(-1無効)
	pub traceSfxId5:i32,

	/// NAME: Root Sword Flash Damipoli ID_5 - 根元剣閃ダミポリID_５
	/// DESC: Sword Flash Root Damipoli ID_5 (-1 invalid) - 剣閃根元ダミポリID_５(-1無効)
	pub traceDmyIdHead5:i32,

	/// NAME: Sword tip sword flash Damipoli ID_5 - 剣先剣閃ダミポリID_５
	/// DESC: Sword Flash Sword Tip Damipoli ID_5 - 剣閃剣先ダミポリID_５
	pub traceDmyIdTail5:i32,

	/// NAME: Sword Flash SfxID_6 - 剣閃SfxID_６
	/// DESC: Sword Flash SfxID_6 (-1 invalid) - 剣閃SfxID_６(-1無効)
	pub traceSfxId6:i32,

	/// NAME: Root Sword Flash Damipoli ID_6 - 根元剣閃ダミポリID_６
	/// DESC: Sword Flash Root Damipoli ID_6 (-1 invalid) - 剣閃根元ダミポリID_６(-1無効)
	pub traceDmyIdHead6:i32,

	/// NAME: Sword tip sword flash Damipoli ID_6 - 剣先剣閃ダミポリID_６
	/// DESC: Sword Flash Sword Tip Damipoli ID_6 - 剣閃剣先ダミポリID_６
	pub traceDmyIdTail6:i32,

	/// NAME: Sword Flash SfxID_7 - 剣閃SfxID_７
	/// DESC: Sword Flash SfxID_7 (-1 invalid) - 剣閃SfxID_７(-1無効)
	pub traceSfxId7:i32,

	/// NAME: Root Sword Flash Damipoli ID_7 - 根元剣閃ダミポリID_７
	/// DESC: Sword Flash Root Damipoli ID_7 (-1 invalid) - 剣閃根元ダミポリID_７(-1無効)
	pub traceDmyIdHead7:i32,

	/// NAME: Sword tip sword flash Damipoli ID_7 - 剣先剣閃ダミポリID_７
	/// DESC: Sword Flash Sword Tip Damipoli ID_7 - 剣閃剣先ダミポリID_７
	pub traceDmyIdTail7:i32,

	/// NAME: Defensive SFX Material 2 - 防御SFX材質2
	/// DESC: Defensive SFX material used from attack para 2 - 攻撃パラから使用される防御SFX材質2
	pub defSfxMaterial2:u16,

	/// NAME: Defense SE Material 2 - 防御SE材質2
	/// DESC: Defense SE material used from attack para 2 - 攻撃パラから使用される防御SE材質2
	pub defSeMaterial2:u16,

	/// NAME: Suction position Id - 吸着位置Id
	/// DESC: Weapon adsorption position parameter Id. This value determines the position where the weapon is attracted (-1: Refer to the value written directly in the old source code). - 武器吸着位置パラメータのId。この値により武器が吸着する位置を決定する(-1：旧ソースコード直書きの値を参照する)
	pub absorpParamId:i32,

	/// NAME: Toughness correction factor - 強靭度 補正倍率
	/// DESC: It is a magnification that corrects the basic value of toughness. - 強靭度の基本値を補正する倍率です
	pub toughnessCorrectRate:f32,

	/// NAME: Is the armor SA damage multiplier valid even at the initial value? - 防具SAダメージ倍率が初期値でも有効か？
	/// DESC: Whether the toughness calculation is performed even if the armor SA is the initial value. Check the toughness specification .xlsx for details - 防具SAが初期値でも強靭度計算が行われるかどうか。詳細は強靭度仕様書.xlsxを確認してください
	pub Bitfield7:u8,

	/// NAME: Correction type (magic attack power) - 補正タイプ（魔法攻撃力）
	/// DESC: Determining the type of magic attack power correction graph with primary parameters - 一次パラメータによる魔法攻撃力の補正グラフのタイプを決める
	pub correctType_Magic:u8,

	/// NAME: Correction type (flame attack power) - 補正タイプ（炎攻撃力）
	/// DESC: Determine the type of flame attack power correction graph with primary parameters - 一次パラメータによる炎攻撃力の補正グラフのタイプを決める
	pub correctType_Fire:u8,

	/// NAME: Correction type (lightning attack power) - 補正タイプ（雷攻撃力）
	/// DESC: Determine the type of lightning attack power correction graph with primary parameters - 一次パラメータによる雷攻撃力の補正グラフのタイプを決める
	pub correctType_Thunder:u8,

	/// NAME: Special attack E damage multiplier - 特攻Eダメージ倍率
	/// DESC: Damage multiplier for special attack E - 特攻E用のダメージ倍率
	pub weakE_DamageRate:f32,

	/// NAME: Special attack F damage multiplier - 特攻Fダメージ倍率
	/// DESC: Damage multiplier for special attack F - 特攻F用のダメージ倍率
	pub weakF_DamageRate:f32,

	/// NAME: Dark attack power cut rate when guarding - ガード時闇攻撃力カット率
	/// DESC: How much to cut the dark attack? - 闇攻撃をどれだけカットするか？
	pub darkGuardCutRate:f32,

	/// NAME: Dark attack power basic value - 闇攻撃力基本値
	/// DESC: Basic value of darkness attack that damages the enemy's HP - 敵のＨＰにダメージを与える闇属性攻撃の基本値
	pub attackBaseDark:u16,

	/// NAME: Correction type (dark attack power) - 補正タイプ（闇攻撃力）
	/// DESC: Determine the type of darkness attack power correction graph with primary parameters - 一次パラメータによる闇攻撃力の補正グラフのタイプを決める
	pub correctType_Dark:u8,

	/// NAME: Correction type (poison attack power) - 補正タイプ（毒攻撃力）
	/// DESC: Determining the type of poison attack power correction graph with primary parameters - 一次パラメータによる毒攻撃力の補正グラフのタイプを決める
	pub correctType_Poison:u8,

	/// NAME: Sort item type ID - ソートアイテム種別ID
	/// DESC: Sort item type ID. In the sort "Item type order", the same ID is displayed together as the same group. - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
	pub sortGroupId:u8,

	/// NAME: Physical attribute 2 - 物理属性2
	/// DESC: Physical attribute 2 - 物理属性2
	pub atkAttribute2:u8,

	/// NAME: Sleep attack cut rate - 睡眠攻撃カット率
	/// DESC: How much to cut the attack power (set as a special effect parameter) to sleep - 睡眠にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub sleepGuardResist:i8,

	/// NAME: Mad attack cut rate - 発狂攻撃カット率
	/// DESC: How much to cut the attack power (set to the special effect parameter) that makes you go mad - 発狂にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub madnessGuardResist:i8,

	/// NAME: Correction type (bleeding attack power) - 補正タイプ（出血攻撃力）
	/// DESC: Determine the type of bleeding attack power correction graph with primary parameters - 一次パラメータによる出血攻撃力の補正グラフのタイプを決める
	pub correctType_Blood:u8,

	/// NAME: Equipment proper luck - 装備適正運
	/// DESC: Equipment appropriate value. - 装備適正値.
	pub properLuck:u8,

	/// NAME: Cold attack cut rate - 冷気攻撃カット率
	/// DESC: How much to cut the attack power (set to the special effect parameter) to cool down - 冷気にする攻撃力（特殊効果パラメータに設定）をどれだけカットするか
	pub freezeGuardResist:i8,

	/// NAME: Automatic replenishment type - 自動補充タイプ
	/// DESC: Controls whether or not to automatically replenish and default settings - 自動補充する/しないの可否およびデフォルト設定をコントロールします
	pub autoReplenishType:u8,

	/// NAME: Arts parameter ID - アーツパラメータID
	/// DESC: Arts parameter ID - アーツパラメータのID
	pub swordArtsParamId:i32,

	/// NAME: Luck correction - 運補正
	/// DESC: Charapara correction value. - キャラパラ補正値.
	pub correctLuck:f32,

	/// NAME: Equipment ID for quiver (magazine) display model - 矢筒(弾倉)表示モデル用装備ID
	/// DESC: Equipment number of the quiver (magazine) display model. In the case of a bow, it is displayed as a quiver, and in the case of a crossbow, it is displayed as a magazine. - 矢筒(弾倉)表示モデルの装備品番号。弓の場合は矢筒、弩の場合は弾倉として表示する。
	pub arrowBoltEquipId:u32,

	/// NAME: Level setting at the time of reduction - 還元時レベル設定
	/// DESC: Type of how to set the enhancement level when returning or deriving a weapon - 武器を還元・派生させるときに強化レベルをどう設定するかの種別
	pub DerivationLevelType:u8,

	/// NAME: Enchantment Sfx size - エンチャントSfxサイズ
	/// DESC: Value offset to enchantment SfxId - エンチャントSfxIdにオフセットする値
	pub enchantSfxSize:u8,

	/// NAME: Weapon type - 武器種別
	/// DESC: Weapon type. Used for linking text and magic stones (* It is now used for other than text) - 武器種別。テキストと、魔石の紐付けに使われる（※テキスト以外にも使われるようになった）
	pub wepType:u16,

	/// NAME: Physical attack cut rate when guarding_maximum correction value - ガード時物理攻撃カット率_最大補正値
	/// DESC: Maximum damage physical cut rate correction value when guarding - ガード時のダメージ物理カット率の補正値の最大値
	pub physGuardCutRate_MaxCorrect:f32,

	/// NAME: Magic attack cut rate when guarding_maximum correction value - ガード時魔法攻撃カット率_最大補正値
	/// DESC: Maximum value of correction value of damage magic cut rate when guarding - ガード時のダメージ魔法カット率の補正値の最大値
	pub magGuardCutRate_MaxCorrect:f32,

	/// NAME: Flame attack power cut rate when guarding_maximum correction value - ガード時炎攻撃力カット率_最大補正値
	/// DESC: Maximum value of correction value of damage flame cut rate when guarding - ガード時のダメージ炎カット率の補正値の最大値
	pub fireGuardCutRate_MaxCorrect:f32,

	/// NAME: Electric shock attack power cut rate when guarding_maximum correction value - ガード時電撃攻撃力カット率_最大補正値
	/// DESC: Maximum value of correction value of damage electric shock cut rate when guarding - ガード時のダメージ電撃カット率の補正値の最大値
	pub thunGuardCutRate_MaxCorrect:f32,

	/// NAME: Darkness attack power cut rate when guarding_maximum correction value - ガード時闇攻撃力カット率_最大補正値
	/// DESC: Maximum value of correction value of damage darkness cut rate when guarding - ガード時のダメージ闇カット率の補正値の最大値
	pub darkGuardCutRate_MaxCorrect:f32,

	/// NAME: Poison resistance cut rate_maximum correction value - 毒耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against poison (set as special effect parameter) - 毒に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub poisonGuardResist_MaxCorrect:f32,

	/// NAME: Epidemic resistance cut rate_maximum correction value - 疫病耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against plague (set as special effect parameter) - 疫病に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub diseaseGuardResist_MaxCorrect:f32,

	/// NAME: Bleeding resistance cut rate_maximum correction value - 出血耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against bleeding (set as special effect parameter) - 出血に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub bloodGuardResist_MaxCorrect:f32,

	/// NAME: Curse resistance cut rate_maximum correction value - 呪耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against curse (set as special effect parameter) - 呪いに対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub curseGuardResist_MaxCorrect:f32,

	/// NAME: Cold resistance cut rate_maximum correction value - 冷気耐性カット率_最大補正値
	/// DESC: Maximum value of cut rate correction value for attack power against cold air (set as special effect parameter) - 冷気に対する攻撃力（特殊効果パラメータに設定）のカット率補正値の最大値
	pub freezeGuardResist_MaxCorrect:f32,

	/// NAME: Stamina defense when guarding_maximum correction value - ガード時スタミナ防御力_最大補正値
	/// DESC: When the guard is successful, the maximum value of the defense power correction value against the enemy's stamina attack - ガード成功時に、敵のスタミナ攻撃に対する防御力の補正値の最大値
	pub staminaGuardDef_MaxCorrect:f32,

	/// NAME: Resident SfxId1 - 常駐SfxId１
	/// DESC: Resident SfxId1 - 常駐SfxId1
	pub residentSfxId_1:i32,

	/// NAME: Resident SfxId2 - 常駐SfxId２
	/// DESC: Resident SfxId2 - 常駐SfxId2
	pub residentSfxId_2:i32,

	/// NAME: Resident SfxId3 - 常駐SfxId３
	/// DESC: Resident SfxId3 - 常駐SfxId3
	pub residentSfxId_3:i32,

	/// NAME: Resident SfxId4 - 常駐SfxId４
	/// DESC: Resident SfxId4 - 常駐SfxId4
	pub residentSfxId_4:i32,

	/// NAME: Resident Sfx Damipoli Id1 - 常駐SfxダミポリId１
	/// DESC: Resident Sfx Damipoli Id1 - 常駐SfxダミポリId１
	pub residentSfx_DmyId_1:i32,

	/// NAME: Resident Sfx Damipoli Id2 - 常駐SfxダミポリId２
	/// DESC: Resident Sfx Damipoli Id2 - 常駐SfxダミポリId２
	pub residentSfx_DmyId_2:i32,

	/// NAME: Resident Sfx Damipoli Id3 - 常駐SfxダミポリId３
	/// DESC: Resident Sfx Damipoli Id3 - 常駐SfxダミポリId３
	pub residentSfx_DmyId_3:i32,

	/// NAME: Resident Sfx Damipoli Id4 - 常駐SfxダミポリId４
	/// DESC: Resident Sfx Damipoli Id4 - 常駐SfxダミポリId４
	pub residentSfx_DmyId_4:i32,

	/// NAME: Stamina consumption ratio - スタミナ消費量倍率
	/// DESC: Stamina consumption ratio - スタミナ消費量倍率
	pub staminaConsumptionRate:f32,

	/// NAME: Physical damage correction factor against player - 対プレイヤー 物理ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Physics:f32,

	/// NAME: Anti-player magic damage correction factor - 対プレイヤー 魔法ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Magic:f32,

	/// NAME: Anti-player flame damage correction factor - 対プレイヤー 炎ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Fire:f32,

	/// NAME: Anti-player lightning damage correction factor - 対プレイヤー 雷ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Thunder:f32,

	/// NAME: Against player darkness damage correction factor - 対プレイヤー 闇ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Dark:f32,

	/// NAME: Anti-player poison damage correction factor - 対プレイヤー 毒ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Poison:f32,

	/// NAME: Anti-player bleeding damage correction factor - 対プレイヤー 出血ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Blood:f32,

	/// NAME: Anti-player cold damage correction factor - 対プレイヤー 冷気ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Freeze:f32,

	/// NAME: Weapon ability release status value: Strength - 武器能力解放ステータス値：筋力
	/// DESC: To change the R2 attack into a special action when the status is X or higher when using a specific weapon. - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
	pub attainmentWepStatusStr:i32,

	/// NAME: Weapon ability release status value: Skill - 武器能力解放ステータス値：技量
	/// DESC: To change the R2 attack into a special action when the status is X or higher when using a specific weapon. - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
	pub attainmentWepStatusDex:i32,

	/// NAME: Weapon ability release status value: reason - 武器能力解放ステータス値：理力
	/// DESC: To change the R2 attack into a special action when the status is X or higher when using a specific weapon. - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
	pub attainmentWepStatusMag:i32,

	/// NAME: Weapon ability release status value: Faith - 武器能力解放ステータス値：信仰
	/// DESC: This is to change the R2 attack into a special action when the status is X or higher when using a specific weapon. - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
	pub attainmentWepStatusFai:i32,

	/// NAME: Weapon ability release status value: Luck - 武器能力解放ステータス値：運
	/// DESC: This is to change the R2 attack into a special action when the status is X or higher when using a specific weapon. - 特定の武器を使った際、ステータスがX以上だとR2攻撃が特殊なアクションに変わるようするためのもの
	pub attainmentWepStatusLuc:i32,

	/// NAME: Attack attribute correction ID - 攻撃属性補正ID
	/// DESC: ID of the parameter that corrects the attack attribute - 攻撃属性を補正するパラメータのID
	pub attackElementCorrectId:i32,

	/// NAME: Selling price - 販売価格
	/// DESC: Selling price - 販売価格
	pub saleValue:i32,

	/// NAME: Enhanced shop category - 強化ショップカテゴリ
	/// DESC: Enhanced shop category - 強化ショップカテゴリ
	pub reinforceShopCategory:u8,

	/// NAME: Maximum number of arrows - 矢の最大所持数
	/// DESC: Maximum number of arrows - 矢の最大所持数
	pub maxArrowQuantity:u8,

	/// NAME: Resident SFX1 Whether to display at the time of sword delivery - 常駐SFX1納刀時表示するか
	/// DESC: If "Do you want to display when resident SFX1 is delivered?" Is true, hide the SFX set to "resident SFX ID1" when the weapon is delivered. - 「常駐SFX1納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID1」に設定されているSFXを非表示にする
	pub Bitfield8:u8,

	/// NAME: Weapon SEID offset value - 武器SEIDオフセット値
	/// DESC: SEID offset value - SEIDのオフセット値 
	pub wepSeIdOffset:i8,

	/// NAME: Evolution price - 進化価格
	/// DESC: Evolution price - 進化価格
	pub baseChangePrice:i32,

	/// NAME: Level sync correction ID - レベルシンク補正ID
	/// DESC: Level sync correction ID - レベルシンク補正ID
	pub levelSyncCorrectId:i16,

	/// NAME: Correction type (sleep attack power) - 補正タイプ（睡眠攻撃力）
	/// DESC: Determine the type of sleep attack correction graph with primary parameters - 一次パラメータによる睡眠攻撃力の補正グラフのタイプを決める
	pub correctType_Sleep:u8,

	/// NAME: Correction type (mad attack power) - 補正タイプ（発狂攻撃力）
	/// DESC: Determining the type of mad attack power correction graph with primary parameters - 一次パラメータによる発狂攻撃力の補正グラフのタイプを決める
	pub correctType_Madness:u8,

	/// NAME: Rarity - レア度
	/// DESC: Rarity used in item acquisition logs - アイテム取得ログで使うレア度
	pub rarity:u8,

	/// NAME: Is it possible to attach magic stones? - 魔石装着可能か
	/// DESC: Is it possible to attach magic stones? - 魔石装着可能か
	pub gemMountType:u8,

	/// NAME: Weapon regain amount - 武器リゲイン量
	/// DESC: Weapon regain amount - 武器リゲイン量
	pub wepRegainHp:u16,

	/// NAME: Effect text ID 00 - 効果テキストID00
	/// DESC: Effect text ID 00 (Weapon_Effect). Weapon-specific effect text to display in status - 効果テキストID00(Weapon_Effect)。ステータスに表示する武器固有効果のテキスト
	pub spEffectMsgId0:i32,

	/// NAME: Effect text ID 01 - 効果テキストID01
	/// DESC: Effect text ID 01 (Weapon_Effect). Weapon-specific effect text to display in status - 効果テキストID01(Weapon_Effect)。ステータスに表示する武器固有効果のテキスト
	pub spEffectMsgId1:i32,

	/// NAME: Effect text ID 02 - 効果テキストID02
	/// DESC: Effect text ID 02 (Weapon_Effect). Weapon-specific effect text to display in status - 効果テキストID02(Weapon_Effect)。ステータスに表示する武器固有効果のテキスト
	pub spEffectMsgId2:i32,

	/// NAME: Derivative source enhancement +16 - 派生元 強化+16
	/// DESC: This weapon's enhancement source weapon ID 16 - この武器の強化元武器ID16
	pub originEquipWep16:i32,

	/// NAME: Derivative source enhancement +17 - 派生元 強化+17
	/// DESC: This weapon's enhancement source weapon ID 17 - この武器の強化元武器ID17
	pub originEquipWep17:i32,

	/// NAME: Derivative source enhancement +18 - 派生元 強化+18
	/// DESC: This weapon's enhancement source weapon ID18 - この武器の強化元武器ID18
	pub originEquipWep18:i32,

	/// NAME: Derivative source enhancement +19 - 派生元 強化+19
	/// DESC: This weapon's enhancement source weapon ID 19 - この武器の強化元武器ID19
	pub originEquipWep19:i32,

	/// NAME: Derivative source enhancement +20 - 派生元 強化+20
	/// DESC: This weapon's enhancement source weapon ID 20 - この武器の強化元武器ID20
	pub originEquipWep20:i32,

	/// NAME: Derivative source enhancement +21 - 派生元 強化+21
	/// DESC: This weapon's enhancement source weapon ID21 - この武器の強化元武器ID21
	pub originEquipWep21:i32,

	/// NAME: Derivative source enhancement +22 - 派生元 強化+22
	/// DESC: This weapon's enhancement source weapon ID 22 - この武器の強化元武器ID22
	pub originEquipWep22:i32,

	/// NAME: Derivative source enhancement +23 - 派生元 強化+23
	/// DESC: This weapon's enhancement source weapon ID 23 - この武器の強化元武器ID23
	pub originEquipWep23:i32,

	/// NAME: Derivative source enhancement +24 - 派生元 強化+24
	/// DESC: This weapon's enhancement source weapon ID 24 - この武器の強化元武器ID24
	pub originEquipWep24:i32,

	/// NAME: Derivative source enhancement +25 - 派生元 強化+25
	/// DESC: This weapon's enhancement source weapon ID 25 - この武器の強化元武器ID25
	pub originEquipWep25:i32,

	/// NAME: Anti-player sleep damage correction factor - 対プレイヤー 睡眠ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Sleep:f32,

	/// NAME: Anti-player mad damage correction factor - 対プレイヤー 発狂ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Madness:f32,

	/// NAME: SA attack cut rate when guarding - ガード時SA攻撃カット率
	/// DESC: SA damage cut rate when guard is successful - ガード成功時のSAダメージのカット率
	pub saGuardCutRate:f32,

	/// NAME: Defensive material variation value - 防御材質バリエーション値
	/// DESC: It is a value used to divide the damage SFX and SE into variations in combination with the defense material used when guarding. SEQ16473 - ガード時に使用される防御材質と組み合わせてダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
	pub defMaterialVariationValue:u8,

	/// NAME: Special attribute variation value - 特殊属性バリエーション値
	/// DESC: It is a value used to give variation to abnormal state SFX, SE, etc. in combination with the special attribute of the weapon. SEQ16473 - 武器の特殊属性と組み合わせて状態異常SFX,SEなどにバリエーションを持たせるために使用する値です。SEQ16473
	pub spAttributeVariationValue:u8,

	/// NAME: Stealth attack power multiplier - ステルス攻撃力倍率
	/// DESC: Stealth attack power multiplier - ステルス攻撃力倍率
	pub stealthAtkRate:i16,

	/// NAME: Anti-player plague damage correction factor - 対プレイヤー 疫病ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Disease:f32,

	/// NAME: Anti-player curse damage correction factor - 対プレイヤー 呪ダメージ補正倍率
	/// DESC: Only attacks on the player will correct the damage done. - プレイヤーに対する攻撃のみ、与えるダメージを補正する。
	pub vsPlayerDmgCorrectRate_Curse:f32,

	/// NAME: pad - pad
	/// DESC: pad - pad
	pub pad:[u8;8],
}

impl Paramdef for EQUIP_PARAM_WEAPON_ST {
const NAME: &'static str = "EQUIP_PARAM_WEAPON_ST";
const VERSION: u16 = 6;
}
impl EQUIP_PARAM_WEAPON_ST {
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
	}	/// Is it possible to equip it with the right hand? - 右手装備可能か.
	/// Bitfield2
	pub fn get_rightHandEquipable(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_rightHandEquipable(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Is it possible to equip it with the left hand? - 左手装備可能か.
	/// Bitfield2
	pub fn get_leftHandEquipable(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_leftHandEquipable(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Is it possible to equip with both hands? - 両手装備可能か.
	/// Bitfield2
	pub fn get_bothHandEquipable(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_bothHandEquipable(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Is it possible to equip a bow bullet? - 弓用矢弾装備可能か.
	/// Bitfield2
	pub fn get_arrowSlotEquipable(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_arrowSlotEquipable(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Is it possible to equip a crossbow bullet? - 弩用矢弾装備可能か.
	/// Bitfield2
	pub fn get_boltSlotEquipable(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_boltSlotEquipable(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Guard with L1 when equipped with left hand - 左手装備時L1でガード
	/// Bitfield2
	pub fn get_enableGuard(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_enableGuard(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Parry with L2 when equipped with left hand - 左手装備時L2でパリィ
	/// Bitfield2
	pub fn get_enableParry(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_enableParry(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Activates magic when attacking - 攻撃時に魔法発動
	/// Bitfield2
	pub fn get_enableMagic(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_enableMagic(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Cast magic when attacking - 攻撃時に呪術発動
	/// Bitfield3
	pub fn get_enableSorcery(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_enableSorcery(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Miracle activated when attacking - 攻撃時に奇蹟発動
	/// Bitfield3
	pub fn get_enableMiracle(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_enableMiracle(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Activates pledge magic when attacking - 攻撃時に誓約魔法発動
	/// Bitfield3
	pub fn get_enableVowMagic(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_enableVowMagic(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Attack type for menu display. Is it normal? - メニュー表示用攻撃タイプ。通常か
	/// Bitfield3
	pub fn get_isNormalAttackType(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_isNormalAttackType(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Attack type for menu display. Is it a blow? - メニュー表示用攻撃タイプ。打撃か
	/// Bitfield3
	pub fn get_isBlowAttackType(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_isBlowAttackType(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Attack type for menu display. Is it a slash? - メニュー表示用攻撃タイプ。斬撃か
	/// Bitfield3
	pub fn get_isSlashAttackType(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_isSlashAttackType(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Attack type for menu display. Is it a piercing? - メニュー表示用攻撃タイプ。刺突か
	/// Bitfield3
	pub fn get_isThrustAttackType(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_isThrustAttackType(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Can it be strengthened with pine fat? - 松脂などで、強化可能か？
	/// Bitfield3
	pub fn get_isEnhance(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_isEnhance(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Is there an attack power correction by human nature? - 人間性による攻撃力補正があるか
	/// Bitfield4
	pub fn get_isHeroPointCorrect(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_isHeroPointCorrect(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Listed in the enhancement target list at the enhancement shop (may be deleted due to specification changes?) - 強化ショップで強化対象リストに並ぶ(仕様変更で削除するかも？)
	/// Bitfield4
	pub fn get_isCustom(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_isCustom(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Is job change reset prohibited? - 転職リセット禁止か
	/// Bitfield4
	pub fn get_disableBaseChangeReset(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_disableBaseChangeReset(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Is repair prohibited? - 修理禁止か
	/// Bitfield4
	pub fn get_disableRepair(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_disableRepair(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Is it a dark hand? - ダークハンドか
	/// Bitfield4
	pub fn get_isDarkHand(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_isDarkHand(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// Is there a simple model for DLC? - ＤＬＣ用シンプルモデルが存在しているか
	/// Bitfield4
	pub fn get_simpleModelForDlc(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_simpleModelForDlc(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Is it a lantern weapon? - ランタン武器か
	/// Bitfield4
	pub fn get_lanternWep(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_lanternWep(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// NPC Para's "spirit body" will now hit the opponent of ○. Also, the attack para "Is it a ghost attack?" Will be able to guard the attack of ○. - NPCパラの「霊体か」が○の相手に攻撃を当たるようになります。また、攻撃パラの「霊体攻撃か」が○の攻撃をガードできるようになります。
	/// Bitfield4
	pub fn get_isVersusGhostWep(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_isVersusGhostWep(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Weapon career change category. Used to display the attribute icon. - 武器転職カテゴリ。属性アイコン表示に使用します。
	/// Bitfield5
	pub fn get_baseChangeCategory(&self) -> u8 {
		&self.Bitfield5 & 0xE0
	}

	/// Bitfield5 MAX: 63
	pub fn set_baseChangeCategory(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 5) & 0xE0;
			let newVal = &self.Bitfield5 & 0x1F | val;
			self.Bitfield5 = newVal
		} else {
			self.Bitfield5 &= 0x1F
		}
	}	/// Is it a dragon hunting weapon? - 竜狩り武器か
	/// Bitfield5
	pub fn get_isDragonSlayer(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_isDragonSlayer(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// Can you leave it in the warehouse? - 倉庫に預けれるか
	/// Bitfield5
	pub fn get_isDeposit(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_isDeposit(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
	/// Bitfield6
	pub fn get_disableMultiDropShare(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_disableMultiDropShare(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
	/// Bitfield6
	pub fn get_isDiscard(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_isDiscard(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Can I put the item on the spot? TRUE = can be placed - アイテムをその場に置けるか？TRUE=置ける
	/// Bitfield6
	pub fn get_isDrop(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_isDrop(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// Whether to display in the item acquisition log when acquiring the item (not entered: ○) - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
	/// Bitfield6
	pub fn get_showLogCondType(&self) -> bool {
		&self.Bitfield6 & 0x8 != 0
	}

	/// Bitfield6
	pub fn set_showLogCondType(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x8
		} else {
			self.Bitfield6 &= 0xF7
		}
	}
	/// Whether it is a throwable weapon - 投げ可能な武器かどうか
	/// Bitfield6
	pub fn get_enableThrow(&self) -> bool {
		&self.Bitfield6 & 0x10 != 0
	}

	/// Bitfield6
	pub fn set_enableThrow(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x10
		} else {
			self.Bitfield6 &= 0xEF
		}
	}
	/// Whether to display it in the item acquisition dialog when acquiring an item (not entered: new only) - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
	/// Bitfield6
	pub fn get_showDialogCondType(&self) -> u8 {
		&self.Bitfield6 & 0x60
	}

	/// Bitfield6 MAX: 3
	pub fn set_showDialogCondType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 5) & 0x60;
			let newVal = &self.Bitfield6 & 0x9F | val;
			self.Bitfield6 = newVal
		} else {
			self.Bitfield6 &= 0x9F
		}
	}	/// Is it prohibited to change the magic stone attribute? - 魔石属性変更禁止か
	/// Bitfield6
	pub fn get_disableGemAttr(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_disableGemAttr(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Whether the toughness calculation is performed even if the armor SA is the initial value. Check the toughness specification .xlsx for details - 防具SAが初期値でも強靭度計算が行われるかどうか。詳細は強靭度仕様書.xlsxを確認してください
	/// Bitfield7
	pub fn get_isValidTough_ProtSADmg(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_isValidTough_ProtSADmg(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// Is this weapon a twin sword? - この武器は双剣か。
	/// Bitfield7
	pub fn get_isDualBlade(&self) -> bool {
		&self.Bitfield7 & 0x2 != 0
	}

	/// Bitfield7
	pub fn set_isDualBlade(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x2
		} else {
			self.Bitfield7 &= 0xFD
		}
	}
	/// Only valid for arrows and bolts. Whether to automatically equip this weapon if the target equipment slot is empty when picking up this weapon - 矢・ボルトのみ有効。新しくこの武器を拾っ時に対象装備スロットが空の場合に自動で装備するかどうか
	/// Bitfield7
	pub fn get_isAutoEquip(&self) -> bool {
		&self.Bitfield7 & 0x4 != 0
	}

	/// Bitfield7
	pub fn set_isAutoEquip(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x4
		} else {
			self.Bitfield7 &= 0xFB
		}
	}
	/// Is it an emergency avoidable weapon? Pass it to the behavior script. - 緊急回避可能な武器かどうか。ビヘイビアスクリプトに渡す。
	/// Bitfield7
	pub fn get_isEnableEmergencyStep(&self) -> bool {
		&self.Bitfield7 & 0x8 != 0
	}

	/// Bitfield7
	pub fn set_isEnableEmergencyStep(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x8
		} else {
			self.Bitfield7 &= 0xF7
		}
	}
	/// Is it hidden during cutscenes? - カットシーン中非表示か
	/// Bitfield7
	pub fn get_invisibleOnRemo(&self) -> bool {
		&self.Bitfield7 & 0x10 != 0
	}

	/// Bitfield7
	pub fn set_invisibleOnRemo(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x10
		} else {
			self.Bitfield7 &= 0xEF
		}
	}
	/// 
	/// Bitfield7
	pub fn get_pad2(&self) -> u8 {
		&self.Bitfield7 & 0xE0
	}

	/// Bitfield7 MAX: 7
	pub fn set_pad2(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 5) & 0xE0;
			let newVal = &self.Bitfield7 & 0x1F | val;
			self.Bitfield7 = newVal
		} else {
			self.Bitfield7 &= 0x1F
		}
	}	/// If "Do you want to display when resident SFX1 is delivered?" Is true, hide the SFX set to "resident SFX ID1" when the weapon is delivered. - 「常駐SFX1納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID1」に設定されているSFXを非表示にする
	/// Bitfield8
	pub fn get_residentSfx_1_IsVisibleForHang(&self) -> bool {
		&self.Bitfield8 & 0x1 != 0
	}

	/// Bitfield8
	pub fn set_residentSfx_1_IsVisibleForHang(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x1
		} else {
			self.Bitfield8 &= 0xFE
		}
	}
	/// If "Do you want to display when resident SFX2 is delivered?" Is true, hide the SFX set to "resident SFX ID2" when the weapon is delivered. - 「常駐SFX2納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID2」に設定されているSFXを非表示にする
	/// Bitfield8
	pub fn get_residentSfx_2_IsVisibleForHang(&self) -> bool {
		&self.Bitfield8 & 0x2 != 0
	}

	/// Bitfield8
	pub fn set_residentSfx_2_IsVisibleForHang(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x2
		} else {
			self.Bitfield8 &= 0xFD
		}
	}
	/// If "Do you want to display when resident SFX3 is delivered?" Is true, hide the SFX set to "resident SFX ID3" when the weapon is delivered. - 「常駐SFX3納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID3」に設定されているSFXを非表示にする
	/// Bitfield8
	pub fn get_residentSfx_3_IsVisibleForHang(&self) -> bool {
		&self.Bitfield8 & 0x4 != 0
	}

	/// Bitfield8
	pub fn set_residentSfx_3_IsVisibleForHang(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x4
		} else {
			self.Bitfield8 &= 0xFB
		}
	}
	/// If "Do you want to display when resident SFX4 is delivered?" Is true, hide the SFX set to "resident SFX ID 4" when the weapon is delivered. - 「常駐SFX4納刀時表示するか」がtrueの場合、武器が納刀された時に「常駐SFXID4」に設定されているSFXを非表示にする
	/// Bitfield8
	pub fn get_residentSfx_4_IsVisibleForHang(&self) -> bool {
		&self.Bitfield8 & 0x8 != 0
	}

	/// Bitfield8
	pub fn set_residentSfx_4_IsVisibleForHang(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x8
		} else {
			self.Bitfield8 &= 0xF7
		}
	}
	/// Whether the vfx parameter "Soul Param ID for Weapon Enchantment" and "Invisible Weapon for Weapon Enchantment" settings are applied - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
	/// Bitfield8
	pub fn get_isSoulParamIdChange_model0(&self) -> bool {
		&self.Bitfield8 & 0x10 != 0
	}

	/// Bitfield8
	pub fn set_isSoulParamIdChange_model0(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x10
		} else {
			self.Bitfield8 &= 0xEF
		}
	}
	/// Whether the vfx parameter "Soul Param ID for Weapon Enchantment" and "Invisible Weapon for Weapon Enchantment" settings are applied - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
	/// Bitfield8
	pub fn get_isSoulParamIdChange_model1(&self) -> bool {
		&self.Bitfield8 & 0x20 != 0
	}

	/// Bitfield8
	pub fn set_isSoulParamIdChange_model1(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x20
		} else {
			self.Bitfield8 &= 0xDF
		}
	}
	/// Whether the vfx parameter "Soul Param ID for Weapon Enchantment" and "Invisible Weapon for Weapon Enchantment" settings are applied - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
	/// Bitfield8
	pub fn get_isSoulParamIdChange_model2(&self) -> bool {
		&self.Bitfield8 & 0x40 != 0
	}

	/// Bitfield8
	pub fn set_isSoulParamIdChange_model2(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x40
		} else {
			self.Bitfield8 &= 0xBF
		}
	}
	/// Whether the vfx parameter "Soul Param ID for Weapon Enchantment" and "Invisible Weapon for Weapon Enchantment" settings are applied - vfxパラメータの「武器エンチャント用ソウルパラムID」と「武器エンチャント用インビジブルウェポンか」設定が適応されるか
	/// Bitfield8
	pub fn get_isSoulParamIdChange_model3(&self) -> bool {
		&self.Bitfield8 & 0x80 != 0
	}

	/// Bitfield8
	pub fn set_isSoulParamIdChange_model3(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x80
		} else {
			self.Bitfield8 &= 0x7F
		}
	}

}
