/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 6
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_PARAM_PROTECTOR_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Sort ID - ソートID
	/// DESC: Sort ID (7 digits is the limit for s32 because the enhancement level is added in the program) - ソートID(プログラム内で強化レベルを加味しているので s32 では７桁が限界)
	pub sortId:i32,

	/// NAME: Wandering equipment ID - 徘徊装備ID
	/// DESC: Replacement equipment ID for wandering ghosts. - 徘徊ゴースト用の差し替え装備ID.
	pub wanderingEquipId:u32,

	/// NAME: Sleep tolerance - 睡眠耐性
	/// DESC: Difficulty in getting sleep abnormalities - 睡眠状態異常へのかかりにくさ
	pub resistSleep:u16,

	/// NAME: Madness resistance - 発狂耐性
	/// DESC: Difficulty in getting mad - 発狂状態異常へのかかりにくさ
	pub resistMadness:u16,

	/// NAME: SA durability value - SA耐久値
	/// DESC: Super armor endurance - スーパーアーマー耐久力
	pub saDurability:f32,

	/// NAME: Toughness correction factor - 強靭度 補正倍率
	/// DESC: It is a magnification that corrects the basic value of toughness. - 強靭度の基本値を補正する倍率です
	pub toughnessCorrectRate:f32,

	/// NAME: Repair price - 修理価格
	/// DESC: Basic repair price - 修理基本価格
	pub fixPrice:i32,

	/// NAME: Basic price - 基本価格
	/// DESC: Basic price - 基本価格
	pub basicPrice:i32,

	/// NAME: Sale price - 売却価格
	/// DESC: Selling price - 販売価格
	pub sellValue:i32,

	/// NAME: Weight [kg] - 重量[kg]
	/// DESC: Weight [kg]. - 重量[kg].
	pub weight:f32,

	/// NAME: Resident special effect ID1 - 常駐特殊効果ID1
	/// DESC: Resident special effect ID1 - 常駐特殊効果ID1
	pub residentSpEffectId:i32,

	/// NAME: Resident special effect ID2 - 常駐特殊効果ID2
	/// DESC: Resident special effect ID2 - 常駐特殊効果ID2
	pub residentSpEffectId2:i32,

	/// NAME: Resident special effect ID3 - 常駐特殊効果ID3
	/// DESC: Resident special effect ID3 - 常駐特殊効果ID3
	pub residentSpEffectId3:i32,

	/// NAME: Material ID - 素材ID
	/// DESC: Material parameter ID required for weapon enhancement - 武器強化に必要な素材パラメータID
	pub materialSetId:i32,

	/// NAME: Part damage rate - 部位ダメージ率
	/// DESC: Part damage rate - 部位ダメージ率
	pub partsDamageRate:f32,

	/// NAME: SA recovery time correction value - SA回復時間補正値
	/// DESC: Super Armor Recovery Time Correction Value - スーパーアーマー回復時間の補正値
	pub corectSARecover:f32,

	/// NAME: Derivation source - 派生元
	/// DESC: Strengthening of this armor Original armor ID - この防具の強化元防具ID
	pub originEquipPro:i32,

	/// NAME: Derivative source enhancement +1 - 派生元 強化+1
	/// DESC: Strengthening this armor Original armor ID1 - この防具の強化元防具ID1
	pub originEquipPro1:i32,

	/// NAME: Derivative source enhancement +2 - 派生元 強化+2
	/// DESC: Strengthening this armor Original armor ID2 - この防具の強化元防具ID2
	pub originEquipPro2:i32,

	/// NAME: Derivative source enhancement +3 - 派生元 強化+3
	/// DESC: Strengthening this armor Original armor ID3 - この防具の強化元防具ID3
	pub originEquipPro3:i32,

	/// NAME: Derivative source enhancement +4 - 派生元 強化+4
	/// DESC: Strengthening this armor Original armor ID4 - この防具の強化元防具ID4
	pub originEquipPro4:i32,

	/// NAME: Derivative source enhancement +5 - 派生元 強化+5
	/// DESC: Strengthening this armor Original armor ID5 - この防具の強化元防具ID5
	pub originEquipPro5:i32,

	/// NAME: Derivative source enhancement +6 - 派生元 強化+6
	/// DESC: Strengthening this armor Original armor ID6 - この防具の強化元防具ID6
	pub originEquipPro6:i32,

	/// NAME: Derivative source enhancement +7 - 派生元 強化+7
	/// DESC: Strengthening this armor Original armor ID7 - この防具の強化元防具ID7
	pub originEquipPro7:i32,

	/// NAME: Derivative source enhancement +8 - 派生元 強化+8
	/// DESC: Strengthening this armor Original armor ID8 - この防具の強化元防具ID8
	pub originEquipPro8:i32,

	/// NAME: Derivative source enhancement +9 - 派生元 強化+9
	/// DESC: Strengthening this armor Original armor ID9 - この防具の強化元防具ID9
	pub originEquipPro9:i32,

	/// NAME: Derivative source enhancement +10 - 派生元 強化+10
	/// DESC: Strengthening this armor Original armor ID10 - この防具の強化元防具ID10
	pub originEquipPro10:i32,

	/// NAME: Derivative source enhancement +11 - 派生元 強化+11
	/// DESC: Strengthening this armor Original armor ID11 - この防具の強化元防具ID11
	pub originEquipPro11:i32,

	/// NAME: Derivative source enhancement +12 - 派生元 強化+12
	/// DESC: Strengthening this armor Original armor ID12 - この防具の強化元防具ID12
	pub originEquipPro12:i32,

	/// NAME: Derivative source enhancement +13 - 派生元 強化+13
	/// DESC: Strengthening this armor Original armor ID13 - この防具の強化元防具ID13
	pub originEquipPro13:i32,

	/// NAME: Derivative source enhancement +14 - 派生元 強化+14
	/// DESC: Strengthening this armor Original armor ID14 - この防具の強化元防具ID14
	pub originEquipPro14:i32,

	/// NAME: Derivative source enhancement +15 - 派生元 強化+15
	/// DESC: Strengthening this armor Original armor ID15 - この防具の強化元防具ID15
	pub originEquipPro15:i32,

	/// NAME: Man profile enlargement scale - 男横顔拡大スケール
	pub faceScaleM_ScaleX:f32,

	/// NAME: Male face enlargement scale - 男前顔拡大スケール
	pub faceScaleM_ScaleZ:f32,

	/// NAME: Maximum magnification for male profile enlargement - 男横顔拡大最大倍率
	pub faceScaleM_MaxX:f32,

	/// NAME: Maximum magnification for man's face enlargement - 男前顔拡大最大倍率
	pub faceScaleM_MaxZ:f32,

	/// NAME: Female profile enlargement scale - 女横顔拡大スケール
	pub faceScaleF_ScaleX:f32,

	/// NAME: Female face enlargement scale - 女前顔拡大スケール
	pub faceScaleF_ScaleZ:f32,

	/// NAME: Female profile enlargement maximum magnification - 女横顔拡大最大倍率
	pub faceScaleF_MaxX:f32,

	/// NAME: Maximum magnification for female face enlargement - 女前顔拡大最大倍率
	pub faceScaleF_MaxZ:f32,

	/// NAME: QWCID - QWCID
	/// DESC: QWC parameter ID - QWCのパラメタID
	pub qwcId:i32,

	/// NAME: Equipment model number - 装備モデル番号
	/// DESC: Equipment model number. - 装備モデルの番号.
	pub equipModelId:u16,

	/// NAME: Icon ID for men - 男用アイコンID
	/// DESC: Men's menu icon ID. - 男用メニューアイコンID.
	pub iconIdM:u16,

	/// NAME: Female icon ID - 女用アイコンID
	/// DESC: Women's menu icon ID. - 女用メニューアイコンID.
	pub iconIdF:u16,

	/// NAME: Knockback cut rate - ノックバックカット率
	/// DESC: Knockback reduction value. - ノックバックの減少値.
	pub knockBack:u16,

	/// NAME: Knockback repulsion rate - ノックバック反発率
	/// DESC: Knockback repulsion rate. - ノックバックの反発率.
	pub knockbackBounceRate:u16,

	/// NAME: Durability - 耐久度
	/// DESC: Initial durability. - 初期耐久度.
	pub durability:u16,

	/// NAME: Maximum durability - 耐久度最大値
	/// DESC: New durability. - 新品耐久度.
	pub durabilityMax:u16,

	/// NAME: pad - pad
	pub pad03:[u8;2],

	/// NAME: Repellent defense - はじき防御力
	/// DESC: Used to determine the repulsion of enemy attacks. - 敵の攻撃のはじき返し判定に利用.
	pub defFlickPower:u16,

	/// NAME: Physical defense - 物理防御力
	/// DESC: Physical attack damage protection. - 物理攻撃のダメージ防御.
	pub defensePhysics:u16,

	/// NAME: Magic defense - 魔法防御力
	/// DESC: Magic attack damage protection. - 魔法攻撃のダメージ防御.
	pub defenseMagic:u16,

	/// NAME: Fire defense - 炎防御力
	/// DESC: Fire attack damage protection. - 炎攻撃のダメージ防御.
	pub defenseFire:u16,

	/// NAME: Electric shock defense - 電撃防御力
	/// DESC: Damage protection for electric shock attacks. - 電撃攻撃のダメージ防御.
	pub defenseThunder:u16,

	/// NAME: Slash defense - 斬撃防御力
	/// DESC: Look at the attack type, and if it is a slashing attribute, reduce the defense power - 攻撃タイプを見て、斬撃属性のときは、防御力を減少させる
	pub defenseSlash:i16,

	/// NAME: Strike defense - 打撃防御力
	/// DESC: Look at the attack attribute, and if it is a hit attribute, reduce the defense power. - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
	pub defenseBlow:i16,

	/// NAME: Puncture defense - 刺突防御力
	/// DESC: Look at the attack attribute, and if it is a hit attribute, reduce the defense power. - 攻撃属性を見て、打撃属性のときは、防御力を減少させる.
	pub defenseThrust:i16,

	/// NAME: Poison resistance - 毒耐性
	/// DESC: Difficulty in getting poisonous - 毒状態異常へのかかりにくさ
	pub resistPoison:u16,

	/// NAME: Epidemic resistance - 疫病耐性
	/// DESC: Difficulty in getting sick - 疫病状態異常へのかかりにくさ
	pub resistDisease:u16,

	/// NAME: Bleeding resistance - 出血耐性
	/// DESC: Difficulty in getting bleeding abnormalities - 出血状態異常へのかかりにくさ
	pub resistBlood:u16,

	/// NAME: Curse resistance - 呪耐性
	/// DESC: Difficulty in getting a curse condition - 呪い状態異常へのかかりにくさ
	pub resistCurse:u16,

	/// NAME: Enhanced type ID - 強化タイプID
	/// DESC: Enhanced type ID - 強化タイプID
	pub reinforceTypeId:i16,

	/// NAME: Trophy - トロフィー
	/// DESC: Is it related to the trophy system? - トロフィーシステムに関係あるか？
	pub trophySGradeId:i16,

	/// NAME: Shop level - ショップレベル
	/// DESC: Level that can be sold at the store - お店で販売できるレベル
	pub shopLv:i16,

	/// NAME: Knockback parameter ID - ノックバックパラメータID
	/// DESC: ID of the parameter used for knockback - ノックバックで使用するパラメータのID
	pub knockbackParamId:u8,

	/// NAME: Damage attenuation rate when repelling [%] - はじき時ダメージ減衰率[%]
	/// DESC: Used for damage attenuation rate when repelling - はじき時のダメージ減衰率に使用
	pub flickDamageCutRate:u8,

	/// NAME: Equipment model type - 装備モデル種別
	/// DESC: Equipment model type. - 装備モデルの種別.
	pub equipModelCategory:u8,

	/// NAME: Equipment model gender - 装備モデル性別
	/// DESC: Gender of equipment model. - 装備モデルの性別.
	pub equipModelGender:u8,

	/// NAME: Armor category - 防具カテゴリ
	/// DESC: Armor category. - 防具のカテゴリ.
	pub protectorCategory:u8,

	/// NAME: Rarity - レア度
	/// DESC: Rarity used in item acquisition logs - アイテム取得ログで使うレア度
	pub rarity:u8,

	/// NAME: Sort item type ID - ソートアイテム種別ID
	/// DESC: Sort item type ID. In the sort "Item type order", the same ID is displayed together as the same group. - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
	pub sortGroupId:u8,

	/// NAME: Part damage application attack - 部位ダメージ適用攻撃
	/// DESC: Set the attack type to judge the part damage - 部位ダメージ判定を行う攻撃タイプを設定
	pub partsDmgType:u8,

	/// NAME: Padding - パディング
	pub pad04:[u8;2],

	/// NAME: Can i deposit - 預けれるか
	/// DESC: Can you leave it in the warehouse? - 倉庫に預けれるか
	pub Bitfield2:u8,

	/// NAME: Weakness defense material variation value - 弱点防御材質バリエーション値
	/// DESC: It is a value used for variation of abnormal condition, damage SFX, SE in combination with weak point defense material. SEQ16473 - 弱点防御材質と組み合わせて状態異常、ダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
	pub defenseMaterialVariationValue_Weak:u8,

	/// NAME: Foot decal identifier 2 - フットデカール識別子2
	/// DESC: Decal ID for automatic foot effects. Floor material is also considered. Only used when the "armor category" is "legs". - 自動フットエフェクトのデカールID。床材質も考慮される。防具カテゴリ」が「脚」のときのみ利用される。
	pub autoFootEffectDecalBaseId2:i16,

	/// NAME: Foot decal identifier 3 - フットデカール識別子3
	/// DESC: Decal ID for automatic foot effects. Floor material is also considered. Only used when the "armor category" is "legs". - 自動フットエフェクトのデカールID。床材質も考慮される。防具カテゴリ」が「脚」のときのみ利用される。
	pub autoFootEffectDecalBaseId3:i16,

	/// NAME: Defensive material variation value - 防御材質バリエーション値
	/// DESC: It is a value used in combination with the defense material to classify abnormal conditions, damage SFX, and SE. SEQ16473 - 防御材質と組み合わせて状態異常、ダメージSFX,SEのバリエーション分けに使用する値です。SEQ16473
	pub defenseMaterialVariationValue:u8,

	/// NAME: Can you throw it away - 捨てれるか
	/// DESC: Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
	pub Bitfield3:u8,

	/// NAME: Non-attribute damage multiplier - 無属性ダメージ倍率
	/// DESC: Non-attribute damage multiplier - 無属性ダメージ倍率
	pub neutralDamageCutRate:f32,

	/// NAME: Slash damage multiplier - 斬撃ダメージ倍率
	/// DESC: Slash damage multiplier - 斬撃ダメージ倍率
	pub slashDamageCutRate:f32,

	/// NAME: Batter damage multiplier - 打撃ダメージ倍率
	/// DESC: Batter damage multiplier - 打撃ダメージ倍率
	pub blowDamageCutRate:f32,

	/// NAME: Puncture damage ratio - 刺突ダメージ倍率
	/// DESC: Puncture damage ratio - 刺突ダメージ倍率
	pub thrustDamageCutRate:f32,

	/// NAME: Magic damage multiplier - 魔法ダメージ倍率
	/// DESC: Magic damage multiplier - 魔法ダメージ倍率
	pub magicDamageCutRate:f32,

	/// NAME: Flame damage multiplier - 火炎ダメージ倍率
	/// DESC: Flame damage multiplier - 火炎ダメージ倍率
	pub fireDamageCutRate:f32,

	/// NAME: Electric shock damage ratio - 電撃ダメージ倍率
	/// DESC: Electric shock damage ratio - 電撃ダメージ倍率
	pub thunderDamageCutRate:f32,

	/// NAME: Defensive material 1 [SFX] - 防御材質1【SFX】
	/// DESC: For SFX when moving / defending. 1 - 移動/防御時のSFX用.1
	pub defenseMaterialSfx1:u16,

	/// NAME: Weakness protection material 1 [SFX] - 弱点防御材質1【SFX】
	/// DESC: For SFX when weak points are damaged 1 - 弱点部位ダメージ時のSFX用1
	pub defenseMaterialSfx_Weak1:u16,

	/// NAME: Defensive material 1 [SE] - 防御材質1【SE】
	/// DESC: For SE when moving / defending. 1 - 移動/防御時のSE用.1
	pub defenseMaterial1:u16,

	/// NAME: Weakness defense material 1 [SE] - 弱点防御材質1【SE】
	/// DESC: For SE when weak points are damaged 1 - 弱点部位ダメージ時のSE用1
	pub defenseMaterial_Weak1:u16,

	/// NAME: Defensive material 2 [SFX] - 防御材質2【SFX】
	/// DESC: For SFX when moving / defending. 2 - 移動/防御時のSFX用.2
	pub defenseMaterialSfx2:u16,

	/// NAME: Weakness protection material 2 [SFX] - 弱点防御材質2【SFX】
	/// DESC: 2 for SFX when weak points are damaged - 弱点部位ダメージ時のSFX用2
	pub defenseMaterialSfx_Weak2:u16,

	/// NAME: Foot equipment material [SE] - 足装備材質【SE】
	/// DESC: Material for foot equipment SE. Only foot equipment is referenced. ([GR] SEQ10061) In the case of "139: None", the defense material 1 [SE] is referred to. - 足装備SE用材質。足装備のみ参照される。(【GR】SEQ10061) 「139:なし」の場合は防御材質1【SE】が参照される
	pub footMaterialSe:u16,

	/// NAME: Weakness defense material 2 [SE] - 弱点防御材質2【SE】
	/// DESC: 2 for SE when weak points are damaged - 弱点部位ダメージ時のSE用2
	pub defenseMaterial_Weak2:u16,

	/// NAME: Foot decal identifier 1 - フットデカール識別子1
	/// DESC: Decal ID for automatic foot effects. Floor material is also considered. Only used when the "armor category" is "legs". - 自動フットエフェクトのデカールID。床材質も考慮される。防具カテゴリ」が「脚」のときのみ利用される。
	pub autoFootEffectDecalBaseId1:i32,

	/// NAME: Toughness Damage multiplier - 強靭度 被ダメージ倍率
	/// DESC: Toughness version cut rate - 強靭度版カット率
	pub toughnessDamageCutRate:f32,

	/// NAME: Toughness recovery time correction value - 強靭度 回復時間補正値
	/// DESC: Correction value for toughness recovery time - 強靭度の回復時間用の補正値
	pub toughnessRecoverCorrection:f32,

	/// NAME: Dark damage multiplier - 闇ダメージ倍率
	/// DESC: Dark damage multiplier - 闇ダメージ倍率
	pub darkDamageCutRate:f32,

	/// NAME: Dark defense - 闇防御力
	/// DESC: Dark attack damage protection. - 闇攻撃のダメージ防御.
	pub defenseDark:u16,

	/// NAME: PAD_original_ # 48 # hidden - PAD_元_#48#非表示
	/// DESC: Original _ # 48 # hidden - 元_#48#非表示
	pub Bitfield4:u8,

	/// NAME: PAD_original_ # 56 # hidden - PAD_元_#56#非表示
	/// DESC: Original _ # 56 # hidden - 元_#56#非表示
	pub Bitfield5:u8,

	/// NAME: PAD_original_ # 64 # hidden - PAD_元_#64#非表示
	/// DESC: Original _ # 64 # hidden - 元_#64#非表示
	pub Bitfield6:u8,

	/// NAME: PAD_original_ # 72 # hidden - PAD_元_#72#非表示
	/// DESC: Original _ # 72 # hidden - 元_#72#非表示
	pub Bitfield7:u8,

	/// NAME: PAD_original_ # 80 # hidden - PAD_元_#80#非表示
	/// DESC: Original _ # 80 # hidden - 元_#80#非表示
	pub Bitfield8:u8,

	/// NAME: Attitude control ID (torso) - 姿勢制御ID(胴)
	/// DESC: Attitude control ID (torso) - 姿勢制御ID(胴)
	pub postureControlId:u8,

	/// NAME: pad - pad
	pub pad2:[u8;4],

	/// NAME: Selling price - 販売価格
	/// DESC: Selling price - 販売価格
	pub saleValue:i32,

	/// NAME: Cold resistance - 冷気耐性
	/// DESC: Difficulty in getting cold air condition abnormal - 冷気状態異常へのかかりにくさ
	pub resistFreeze:u16,

	/// NAME: # 00 #Hidden (Gender designation) - #00#非表示(男女指定)
	/// DESC: Bangs tip - 前髪の先
	pub invisibleFlag_SexVer00:u8,

	/// NAME: # 01 #Hidden (Gender designation) - #01#非表示(男女指定)
	/// DESC: Bangs root - 前髪の根元
	pub invisibleFlag_SexVer01:u8,

	/// NAME: # 02 #Hidden (Gender designation) - #02#非表示(男女指定)
	/// DESC: Sideburns - もみあげ
	pub invisibleFlag_SexVer02:u8,

	/// NAME: # 03 #Hidden (Gender designation) - #03#非表示(男女指定)
	/// DESC: Top of the head - 頭頂部
	pub invisibleFlag_SexVer03:u8,

	/// NAME: # 04 #Hidden (Gender designation) - #04#非表示(男女指定)
	/// DESC: Top of the head - 頭頂部
	pub invisibleFlag_SexVer04:u8,

	/// NAME: # 05 #Hidden (Gender designation) - #05#非表示(男女指定)
	/// DESC: Back hair - 後ろ髪
	pub invisibleFlag_SexVer05:u8,

	/// NAME: # 06 #Hidden (Gender designation) - #06#非表示(男女指定)
	/// DESC: The tip of the back hair - 後ろ髪の先
	pub invisibleFlag_SexVer06:u8,

	/// NAME: # 07 #Hidden (Gender designation) - #07#非表示(男女指定)
	pub invisibleFlag_SexVer07:u8,

	/// NAME: # 08 #Hidden (Gender designation) - #08#非表示(男女指定)
	pub invisibleFlag_SexVer08:u8,

	/// NAME: # 09 #Hidden (Gender designation) - #09#非表示(男女指定)
	pub invisibleFlag_SexVer09:u8,

	/// NAME: # 10 #Hidden (Gender designation) - #10#非表示(男女指定)
	/// DESC: collar - 襟
	pub invisibleFlag_SexVer10:u8,

	/// NAME: # 11 #Hidden (Gender designation) - #11#非表示(男女指定)
	/// DESC: Around the collar - 襟回り
	pub invisibleFlag_SexVer11:u8,

	/// NAME: # 12 #Hidden (Gender designation) - #12#非表示(男女指定)
	pub invisibleFlag_SexVer12:u8,

	/// NAME: # 13 #Hidden (Gender designation) - #13#非表示(男女指定)
	pub invisibleFlag_SexVer13:u8,

	/// NAME: # 14 #Hidden (Gender designation) - #14#非表示(男女指定)
	pub invisibleFlag_SexVer14:u8,

	/// NAME: # 15 #Hidden (Gender designation) - #15#非表示(男女指定)
	/// DESC: Hood hem - 頭巾の裾
	pub invisibleFlag_SexVer15:u8,

	/// NAME: # 16 #Hidden (Gender designation) - #16#非表示(男女指定)
	pub invisibleFlag_SexVer16:u8,

	/// NAME: # 17 #Hidden (Gender designation) - #17#非表示(男女指定)
	pub invisibleFlag_SexVer17:u8,

	/// NAME: # 18 #Hidden (Gender designation) - #18#非表示(男女指定)
	pub invisibleFlag_SexVer18:u8,

	/// NAME: # 19 #Hidden (Gender designation) - #19#非表示(男女指定)
	pub invisibleFlag_SexVer19:u8,

	/// NAME: # 20 #Hidden (Gender designation) - #20#非表示(男女指定)
	/// DESC: Sleeve A - 袖A
	pub invisibleFlag_SexVer20:u8,

	/// NAME: # 21 #Hidden (Gender designation) - #21#非表示(男女指定)
	/// DESC: Sleeve B - 袖B
	pub invisibleFlag_SexVer21:u8,

	/// NAME: # 22 #Hidden (Gender designation) - #22#非表示(男女指定)
	pub invisibleFlag_SexVer22:u8,

	/// NAME: # 23 #Hidden (Gender designation) - #23#非表示(男女指定)
	pub invisibleFlag_SexVer23:u8,

	/// NAME: # 24 #Hidden (Gender designation) - #24#非表示(男女指定)
	pub invisibleFlag_SexVer24:u8,

	/// NAME: # 25 #Hidden (Gender designation) - #25#非表示(男女指定)
	/// DESC: arm - 腕
	pub invisibleFlag_SexVer25:u8,

	/// NAME: # 26 #Hidden (Gender designation) - #26#非表示(男女指定)
	pub invisibleFlag_SexVer26:u8,

	/// NAME: # 27 #Hidden (Gender designation) - #27#非表示(男女指定)
	pub invisibleFlag_SexVer27:u8,

	/// NAME: # 28 #Hidden (Gender designation) - #28#非表示(男女指定)
	pub invisibleFlag_SexVer28:u8,

	/// NAME: # 29 #Hidden (Gender designation) - #29#非表示(男女指定)
	pub invisibleFlag_SexVer29:u8,

	/// NAME: # 30 #Hidden (Gender designation) - #30#非表示(男女指定)
	/// DESC: belt - ベルト
	pub invisibleFlag_SexVer30:u8,

	/// NAME: # 31 #Hidden (Gender designation) - #31#非表示(男女指定)
	pub invisibleFlag_SexVer31:u8,

	/// NAME: # 32 #Hidden (Men and women specified) - #32#非表示(男女指定)
	pub invisibleFlag_SexVer32:u8,

	/// NAME: # 33 #Hidden (Gender designation) - #33#非表示(男女指定)
	pub invisibleFlag_SexVer33:u8,

	/// NAME: # 34 #Hidden (Gender designation) - #34#非表示(男女指定)
	pub invisibleFlag_SexVer34:u8,

	/// NAME: # 35 #Hidden (Gender designation) - #35#非表示(男女指定)
	pub invisibleFlag_SexVer35:u8,

	/// NAME: # 36 #Hidden (Gender designation) - #36#非表示(男女指定)
	pub invisibleFlag_SexVer36:u8,

	/// NAME: # 37 #Hidden (Gender designation) - #37#非表示(男女指定)
	pub invisibleFlag_SexVer37:u8,

	/// NAME: # 38 #Hidden (Gender designation) - #38#非表示(男女指定)
	pub invisibleFlag_SexVer38:u8,

	/// NAME: # 39 #Hidden (Gender designation) - #39#非表示(男女指定)
	pub invisibleFlag_SexVer39:u8,

	/// NAME: # 40 #Hidden (Gender designation) - #40#非表示(男女指定)
	pub invisibleFlag_SexVer40:u8,

	/// NAME: # 41 #Hidden (Gender designation) - #41#非表示(男女指定)
	pub invisibleFlag_SexVer41:u8,

	/// NAME: # 42 #Hidden (Gender designation) - #42#非表示(男女指定)
	pub invisibleFlag_SexVer42:u8,

	/// NAME: # 43 #Hidden (Gender designation) - #43#非表示(男女指定)
	pub invisibleFlag_SexVer43:u8,

	/// NAME: # 44 #Hidden (Gender designation) - #44#非表示(男女指定)
	pub invisibleFlag_SexVer44:u8,

	/// NAME: # 45 #Hidden (Gender designation) - #45#非表示(男女指定)
	pub invisibleFlag_SexVer45:u8,

	/// NAME: # 46 #Hidden (Gender designation) - #46#非表示(男女指定)
	pub invisibleFlag_SexVer46:u8,

	/// NAME: # 47 #Hidden (Gender designation) - #47#非表示(男女指定)
	pub invisibleFlag_SexVer47:u8,

	/// NAME: # 48 #Hidden (Gender designation) - #48#非表示(男女指定)
	pub invisibleFlag_SexVer48:u8,

	/// NAME: # 49 #Hidden (Gender designation) - #49#非表示(男女指定)
	pub invisibleFlag_SexVer49:u8,

	/// NAME: # 50 #Hidden (Gender designation) - #50#非表示(男女指定)
	pub invisibleFlag_SexVer50:u8,

	/// NAME: # 51 #Hidden (Gender designation) - #51#非表示(男女指定)
	pub invisibleFlag_SexVer51:u8,

	/// NAME: # 52 #Hidden (Gender designation) - #52#非表示(男女指定)
	pub invisibleFlag_SexVer52:u8,

	/// NAME: # 53 #Hidden (Gender designation) - #53#非表示(男女指定)
	pub invisibleFlag_SexVer53:u8,

	/// NAME: # 54 #Hidden (Gender designation) - #54#非表示(男女指定)
	pub invisibleFlag_SexVer54:u8,

	/// NAME: # 55 #Hidden (Gender designation) - #55#非表示(男女指定)
	pub invisibleFlag_SexVer55:u8,

	/// NAME: # 56 #Hidden (Gender designation) - #56#非表示(男女指定)
	pub invisibleFlag_SexVer56:u8,

	/// NAME: # 57 #Hidden (Gender designation) - #57#非表示(男女指定)
	pub invisibleFlag_SexVer57:u8,

	/// NAME: # 58 #Hidden (Gender designation) - #58#非表示(男女指定)
	pub invisibleFlag_SexVer58:u8,

	/// NAME: # 59 #Hidden (Gender designation) - #59#非表示(男女指定)
	pub invisibleFlag_SexVer59:u8,

	/// NAME: # 60 #Hidden (Men and women specified) - #60#非表示(男女指定)
	pub invisibleFlag_SexVer60:u8,

	/// NAME: # 61 #Hidden (Gender designation) - #61#非表示(男女指定)
	pub invisibleFlag_SexVer61:u8,

	/// NAME: # 62 #Hidden (Gender designation) - #62#非表示(男女指定)
	pub invisibleFlag_SexVer62:u8,

	/// NAME: # 63 #Hidden (Gender designation) - #63#非表示(男女指定)
	pub invisibleFlag_SexVer63:u8,

	/// NAME: # 64 #Hidden (Gender designation) - #64#非表示(男女指定)
	pub invisibleFlag_SexVer64:u8,

	/// NAME: # 65 #Hidden (Gender designation) - #65#非表示(男女指定)
	pub invisibleFlag_SexVer65:u8,

	/// NAME: # 66 #Hidden (Gender designation) - #66#非表示(男女指定)
	pub invisibleFlag_SexVer66:u8,

	/// NAME: # 67 #Hidden (Gender designation) - #67#非表示(男女指定)
	pub invisibleFlag_SexVer67:u8,

	/// NAME: # 68 #Hidden (Gender designation) - #68#非表示(男女指定)
	pub invisibleFlag_SexVer68:u8,

	/// NAME: # 69 #Hidden (Gender designation) - #69#非表示(男女指定)
	pub invisibleFlag_SexVer69:u8,

	/// NAME: # 70 #Hidden (Gender designation) - #70#非表示(男女指定)
	pub invisibleFlag_SexVer70:u8,

	/// NAME: # 71 #Hidden (Gender designation) - #71#非表示(男女指定)
	pub invisibleFlag_SexVer71:u8,

	/// NAME: # 72 #Hidden (Gender designation) - #72#非表示(男女指定)
	pub invisibleFlag_SexVer72:u8,

	/// NAME: # 73 #Hidden (Gender designation) - #73#非表示(男女指定)
	pub invisibleFlag_SexVer73:u8,

	/// NAME: # 74 #Hidden (Gender designation) - #74#非表示(男女指定)
	pub invisibleFlag_SexVer74:u8,

	/// NAME: # 75 #Hidden (Gender designation) - #75#非表示(男女指定)
	pub invisibleFlag_SexVer75:u8,

	/// NAME: # 76 #Hidden (Gender designation) - #76#非表示(男女指定)
	pub invisibleFlag_SexVer76:u8,

	/// NAME: # 77 #Hidden (Gender designation) - #77#非表示(男女指定)
	pub invisibleFlag_SexVer77:u8,

	/// NAME: # 78 #Hidden (Gender designation) - #78#非表示(男女指定)
	pub invisibleFlag_SexVer78:u8,

	/// NAME: # 79 #Hidden (Gender designation) - #79#非表示(男女指定)
	pub invisibleFlag_SexVer79:u8,

	/// NAME: # 80 #Hidden (Gender designation) - #80#非表示(男女指定)
	pub invisibleFlag_SexVer80:u8,

	/// NAME: # 81 #Hidden (Gender designation) - #81#非表示(男女指定)
	pub invisibleFlag_SexVer81:u8,

	/// NAME: # 82 #Hidden (Gender designation) - #82#非表示(男女指定)
	pub invisibleFlag_SexVer82:u8,

	/// NAME: # 83 #Hidden (Gender designation) - #83#非表示(男女指定)
	pub invisibleFlag_SexVer83:u8,

	/// NAME: # 84 #Hidden (Gender designation) - #84#非表示(男女指定)
	pub invisibleFlag_SexVer84:u8,

	/// NAME: # 85 #Hidden (Gender designation) - #85#非表示(男女指定)
	pub invisibleFlag_SexVer85:u8,

	/// NAME: # 86 #Hidden (Gender designation) - #86#非表示(男女指定)
	pub invisibleFlag_SexVer86:u8,

	/// NAME: # 87 #Hidden (Gender designation) - #87#非表示(男女指定)
	pub invisibleFlag_SexVer87:u8,

	/// NAME: # 88 #Hidden (Gender designation) - #88#非表示(男女指定)
	pub invisibleFlag_SexVer88:u8,

	/// NAME: # 89 #Hidden (Gender designation) - #89#非表示(男女指定)
	pub invisibleFlag_SexVer89:u8,

	/// NAME: # 90 #Hidden (Gender designation) - #90#非表示(男女指定)
	pub invisibleFlag_SexVer90:u8,

	/// NAME: # 91 #Hidden (Gender designation) - #91#非表示(男女指定)
	pub invisibleFlag_SexVer91:u8,

	/// NAME: # 92 #Hidden (Gender designation) - #92#非表示(男女指定)
	pub invisibleFlag_SexVer92:u8,

	/// NAME: # 93 #Hidden (Gender designation) - #93#非表示(男女指定)
	pub invisibleFlag_SexVer93:u8,

	/// NAME: # 94 #Hidden (Gender designation) - #94#非表示(男女指定)
	pub invisibleFlag_SexVer94:u8,

	/// NAME: # 95 #Hidden (Gender designation) - #95#非表示(男女指定)
	pub invisibleFlag_SexVer95:u8,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad404:[u8;14],
}

impl Paramdef for EQUIP_PARAM_PROTECTOR_ST {
const NAME: &'static str = "EQUIP_PARAM_PROTECTOR_ST";
const VERSION: u16 = 6;
}
impl EQUIP_PARAM_PROTECTOR_ST {
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
	}	/// Can you leave it in the warehouse? - 倉庫に預けれるか
	/// Bitfield2
	pub fn get_isDeposit(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isDeposit(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Is it head equipment? - 頭装備か.
	/// Bitfield2
	pub fn get_headEquip(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_headEquip(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Is it torso equipment? - 胴装備か.
	/// Bitfield2
	pub fn get_bodyEquip(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_bodyEquip(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Is it arm equipment? - 腕装備か.
	/// Bitfield2
	pub fn get_armEquip(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_armEquip(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Is it leg equipment? - 脚装備か.
	/// Bitfield2
	pub fn get_legEquip(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_legEquip(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Whether to use a face scale - 顔スケールを使用するか
	/// Bitfield2
	pub fn get_useFaceScale(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_useFaceScale(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Weakness damage Whether to skip animation playback. "Part damage rate" and "defense material" are treated as weak points just by not playing the animation. - 弱点ダメージアニメ再生をスキップするかどうか。アニメを再生しないだけで「部位ダメージ率」「防御材質」は弱点として扱われます。
	/// Bitfield2
	pub fn get_isSkipWeakDamageAnim(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_isSkipWeakDamageAnim(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// 
	/// Bitfield2
	pub fn get_pad06(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_pad06(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
	/// Bitfield3
	pub fn get_isDiscard(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_isDiscard(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Can I put the item on the spot? TRUE = can be placed - アイテムをその場に置けるか？TRUE=置ける
	/// Bitfield3
	pub fn get_isDrop(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_isDrop(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
	/// Bitfield3
	pub fn get_disableMultiDropShare(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_disableMultiDropShare(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Is there a simple model for DLC? - ＤＬＣ用シンプルモデルが存在しているか
	/// Bitfield3
	pub fn get_simpleModelForDlc(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_simpleModelForDlc(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Whether to display in the item acquisition log when acquiring the item (not entered: ○) - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
	/// Bitfield3
	pub fn get_showLogCondType(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_showLogCondType(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Whether to display it in the item acquisition dialog when acquiring an item (not entered: new only) - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
	/// Bitfield3
	pub fn get_showDialogCondType(&self) -> u8 {
		&self.Bitfield3 & 0x60
	}

	/// Bitfield3 MAX: 3
	pub fn set_showDialogCondType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 5) & 0x60;
			let newVal = &self.Bitfield3 & 0x9F | val;
			self.Bitfield3 = newVal
		} else {
			self.Bitfield3 &= 0x9F
		}
	}	/// 
	/// Bitfield3
	pub fn get_pad(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_pad(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Original _ # 48 # hidden - 元_#48#非表示
	/// Bitfield4
	pub fn get_invisibleFlag48(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag48(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Original _ # 49 # hidden - 元_#49#非表示
	/// Bitfield4
	pub fn get_invisibleFlag49(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag49(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Original _ # 50 # hidden - 元_#50#非表示
	/// Bitfield4
	pub fn get_invisibleFlag50(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag50(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Original _ # 51 # hidden - 元_#51#非表示
	/// Bitfield4
	pub fn get_invisibleFlag51(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag51(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Original _ # 52 # hidden - 元_#52#非表示
	/// Bitfield4
	pub fn get_invisibleFlag52(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag52(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// Original _ # 53 # hidden - 元_#53#非表示
	/// Bitfield4
	pub fn get_invisibleFlag53(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag53(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Original _ # 54 # hidden - 元_#54#非表示
	/// Bitfield4
	pub fn get_invisibleFlag54(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag54(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// Original _ # 55 # hidden - 元_#55#非表示
	/// Bitfield4
	pub fn get_invisibleFlag55(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_invisibleFlag55(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Original _ # 56 # hidden - 元_#56#非表示
	/// Bitfield5
	pub fn get_invisibleFlag56(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag56(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// Original _ # 57 # hidden - 元_#57#非表示
	/// Bitfield5
	pub fn get_invisibleFlag57(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag57(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// Original _ # 58 # hidden - 元_#58#非表示
	/// Bitfield5
	pub fn get_invisibleFlag58(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag58(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// Original _ # 59 # hidden - 元_#59#非表示
	/// Bitfield5
	pub fn get_invisibleFlag59(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag59(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// Original _ # 60 # hidden - 元_#60#非表示
	/// Bitfield5
	pub fn get_invisibleFlag60(&self) -> bool {
		&self.Bitfield5 & 0x10 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag60(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x10
		} else {
			self.Bitfield5 &= 0xEF
		}
	}
	/// Original _ # 61 # Hidden - 元_#61#非表示
	/// Bitfield5
	pub fn get_invisibleFlag61(&self) -> bool {
		&self.Bitfield5 & 0x20 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag61(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x20
		} else {
			self.Bitfield5 &= 0xDF
		}
	}
	/// Original _ # 62 # hidden - 元_#62#非表示
	/// Bitfield5
	pub fn get_invisibleFlag62(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag62(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// Original _ # 63 # hidden - 元_#63#非表示
	/// Bitfield5
	pub fn get_invisibleFlag63(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_invisibleFlag63(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Original _ # 64 # hidden - 元_#64#非表示
	/// Bitfield6
	pub fn get_invisibleFlag64(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag64(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Original _ # 65 # hidden - 元_#65#非表示
	/// Bitfield6
	pub fn get_invisibleFlag65(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag65(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Original _ # 66 # hidden - 元_#66#非表示
	/// Bitfield6
	pub fn get_invisibleFlag66(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag66(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// Original _ # 67 # hidden - 元_#67#非表示
	/// Bitfield6
	pub fn get_invisibleFlag67(&self) -> bool {
		&self.Bitfield6 & 0x8 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag67(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x8
		} else {
			self.Bitfield6 &= 0xF7
		}
	}
	/// Original _ # 68 # hidden - 元_#68#非表示
	/// Bitfield6
	pub fn get_invisibleFlag68(&self) -> bool {
		&self.Bitfield6 & 0x10 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag68(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x10
		} else {
			self.Bitfield6 &= 0xEF
		}
	}
	/// Original _ # 69 # hidden - 元_#69#非表示
	/// Bitfield6
	pub fn get_invisibleFlag69(&self) -> bool {
		&self.Bitfield6 & 0x20 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag69(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x20
		} else {
			self.Bitfield6 &= 0xDF
		}
	}
	/// Original _ # 70 # hidden - 元_#70#非表示
	/// Bitfield6
	pub fn get_invisibleFlag70(&self) -> bool {
		&self.Bitfield6 & 0x40 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag70(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x40
		} else {
			self.Bitfield6 &= 0xBF
		}
	}
	/// Original _ # 71 # hidden - 元_#71#非表示
	/// Bitfield6
	pub fn get_invisibleFlag71(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_invisibleFlag71(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Original _ # 72 # hidden - 元_#72#非表示
	/// Bitfield7
	pub fn get_invisibleFlag72(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag72(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// Original _ # 73 # hidden - 元_#73#非表示
	/// Bitfield7
	pub fn get_invisibleFlag73(&self) -> bool {
		&self.Bitfield7 & 0x2 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag73(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x2
		} else {
			self.Bitfield7 &= 0xFD
		}
	}
	/// Original _ # 74 # hidden - 元_#74#非表示
	/// Bitfield7
	pub fn get_invisibleFlag74(&self) -> bool {
		&self.Bitfield7 & 0x4 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag74(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x4
		} else {
			self.Bitfield7 &= 0xFB
		}
	}
	/// Original _ # 75 # hidden - 元_#75#非表示
	/// Bitfield7
	pub fn get_invisibleFlag75(&self) -> bool {
		&self.Bitfield7 & 0x8 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag75(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x8
		} else {
			self.Bitfield7 &= 0xF7
		}
	}
	/// Original _ # 76 # hidden - 元_#76#非表示
	/// Bitfield7
	pub fn get_invisibleFlag76(&self) -> bool {
		&self.Bitfield7 & 0x10 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag76(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x10
		} else {
			self.Bitfield7 &= 0xEF
		}
	}
	/// Original _ # 77 # hidden - 元_#77#非表示
	/// Bitfield7
	pub fn get_invisibleFlag77(&self) -> bool {
		&self.Bitfield7 & 0x20 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag77(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x20
		} else {
			self.Bitfield7 &= 0xDF
		}
	}
	/// Original _ # 78 # hidden - 元_#78#非表示
	/// Bitfield7
	pub fn get_invisibleFlag78(&self) -> bool {
		&self.Bitfield7 & 0x40 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag78(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x40
		} else {
			self.Bitfield7 &= 0xBF
		}
	}
	/// Original _ # 79 # hidden - 元_#79#非表示
	/// Bitfield7
	pub fn get_invisibleFlag79(&self) -> bool {
		&self.Bitfield7 & 0x80 != 0
	}

	/// Bitfield7
	pub fn set_invisibleFlag79(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x80
		} else {
			self.Bitfield7 &= 0x7F
		}
	}
	/// Original _ # 80 # hidden - 元_#80#非表示
	/// Bitfield8
	pub fn get_invisibleFlag80(&self) -> bool {
		&self.Bitfield8 & 0x1 != 0
	}

	/// Bitfield8
	pub fn set_invisibleFlag80(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x1
		} else {
			self.Bitfield8 &= 0xFE
		}
	}
	/// 
	/// Bitfield8
	pub fn get_padbit(&self) -> u8 {
		&self.Bitfield8 & 0xFE
	}

	/// Bitfield8 MAX: 127
	pub fn set_padbit(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield8 & 0x1 | val;
			self.Bitfield8 = newVal
		} else {
			self.Bitfield8 &= 0x1
		}
	}
}
