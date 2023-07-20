/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ATK_PARAM_ST {
    /// NAME: 0 radius per - あたり0 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit0_Radius: f32,

    /// NAME: 1 radius per - あたり1 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit1_Radius: f32,

    /// NAME: 2 radii per - あたり2 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit2_Radius: f32,

    /// NAME: 3 radii per - あたり3 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit3_Radius: f32,

    /// NAME: Knockback distance [m] - ノックバック距離[m]
    /// DESC: Knockback distance [m] - ノックバック距離[m]
    pub knockbackDist: f32,

    /// NAME: Hit stop time [s] - ヒットストップ時間[s]
    /// DESC: Hit stop stop time [s] - ヒットストップの停止時間[s]
    pub hitStopTime: f32,

    /// NAME: Special effects 0 - 特殊効果0
    /// DESC: Enter the ID created with the special effects parameter - 特殊効果パラメータで作成したＩＤを入れる
    pub spEffectId0: i32,

    /// NAME: Special effect 1 - 特殊効果1
    /// DESC: Enter the ID created with the special effects parameter - 特殊効果パラメータで作成したＩＤを入れる
    pub spEffectId1: i32,

    /// NAME: Special effect 2 - 特殊効果2
    /// DESC: Enter the ID created with the special effects parameter - 特殊効果パラメータで作成したＩＤを入れる
    pub spEffectId2: i32,

    /// NAME: Special effect 3 - 特殊効果3
    /// DESC: Enter the ID created with the special effects parameter - 特殊効果パラメータで作成したＩＤを入れる
    pub spEffectId3: i32,

    /// NAME: Special effect 4 - 特殊効果4
    /// DESC: Enter the ID created with the special effects parameter - 特殊効果パラメータで作成したＩＤを入れる
    pub spEffectId4: i32,

    /// NAME: Per 0 Damipoli 1 - あたり0 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit0_DmyPoly1: i16,

    /// NAME: 1 per Damipoli 1 - あたり1 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit1_DmyPoly1: i16,

    /// NAME: 2 per Damipoli 1 - あたり2 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit2_DmyPoly1: i16,

    /// NAME: 3 per Damipoli 1 - あたり3 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit3_DmyPoly1: i16,

    /// NAME: Per 0 Damipoli 2 - あたり0 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit0_DmyPoly2: i16,

    /// NAME: 1 per Damipoli 2 - あたり1 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit1_DmyPoly2: i16,

    /// NAME: 2 per Damipoli 2 - あたり2 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit2_DmyPoly2: i16,

    /// NAME: 3 per Damipoli 2 - あたり3 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit3_DmyPoly2: i16,

    /// NAME: Blow-off correction value - 吹き飛ばし補正値
    /// DESC: Correction value when blowing off - 吹き飛ばす時の補正値
    pub blowingCorrection: u16,

    /// NAME: Physical attack power correction value - 物理攻撃力補正値
    /// DESC: PC only. Multiplier by multiplying the basic physical attack power - PCのみ。物理攻撃力基本値に掛ける倍率
    pub atkPhysCorrection: u16,

    /// NAME: Magic attack power correction value - 魔法攻撃力補正値
    /// DESC: PC only. Multiply the magic attack power (in the case of a bow, correct the missile) - PCのみ。魔法攻撃力に掛ける倍率（弓の場合は、飛び道具を補正）
    pub atkMagCorrection: u16,

    /// NAME: Fire attack power correction value - 炎攻撃力補正値
    /// DESC: PC only. Multiply the fire attack power (in the case of a bow, correct the missile) - PCのみ。炎攻撃力に掛ける倍率（弓の場合は、飛び道具を補正）
    pub atkFireCorrection: u16,

    /// NAME: Electric shock attack power correction value - 電撃攻撃力補正値
    /// DESC: PC only. Multiplier for electric shock attack power (in the case of a bow, correct the missile) - PCのみ。電撃攻撃力に掛ける倍率（弓の場合は、飛び道具を補正）
    pub atkThunCorrection: u16,

    /// NAME: Stamina attack power correction value - スタミナ攻撃力補正値
    /// DESC: PC only. Multiplier for stamina attack power - PCのみ。スタミナ攻撃力に掛ける倍率
    pub atkStamCorrection: u16,

    /// NAME: Repellent attack power correction value - はじき攻撃力補正値
    /// DESC: PC only. 1 only - PCのみ。1のみ
    pub guardAtkRateCorrection: u16,

    /// NAME: Repellent defense correction value - はじき防御力補正値
    /// DESC: PC only. Multiplying the base value by the repelling of the attack - PCのみ。攻撃のはじかれ基本値に掛ける倍率
    pub guardBreakCorrection: u16,

    /// NAME: Throw-through attack power correction value - 投げ抜け攻撃力補正値
    /// DESC: Weapon correction value for throw-through attacks - 投げ抜け攻撃に対する武器補正値
    pub atkThrowEscapeCorrection: u16,

    /// NAME: Subcategory 1 - サブカテゴリ1
    /// DESC: Subcategory 1 - サブカテゴリ1
    pub subCategory1: u8,

    /// NAME: Subcategory 2 - サブカテゴリ2
    /// DESC: Subcategory 2 - サブカテゴリ2
    pub subCategory2: u8,

    /// NAME: Physical attack power - 物理攻撃力
    /// DESC: NPCs only. Basic damage of physical attack - NPCのみ。物理攻撃の基本ダメージ
    pub atkPhys: u16,

    /// NAME: Magic attack power - 魔法攻撃力
    /// DESC: NPCs only. Additional damage from magic attacks - NPCのみ。魔法攻撃の追加ダメージ
    pub atkMag: u16,

    /// NAME: Fire attack power - 炎攻撃力
    /// DESC: NPCs only. Additional damage from fire attacks - NPCのみ。炎攻撃の追加ダメージ
    pub atkFire: u16,

    /// NAME: Electric shock attack power - 電撃攻撃力
    /// DESC: NPCs only. Additional damage from electric shock attacks - NPCのみ。電撃攻撃の追加ダメージ
    pub atkThun: u16,

    /// NAME: Stamina attack power - スタミナ攻撃力
    /// DESC: NPCs only. Amount of damage to enemy (player) stamina - NPCのみ。敵（プレイヤー）のスタミナに対するダメージ量
    pub atkStam: u16,

    /// NAME: Repellent attack power - はじき攻撃力
    /// DESC: NPCs only. Flick value - NPCのみ。はじき値
    pub guardAtkRate: u16,

    /// NAME: Repellent defense - はじき防御力
    /// DESC: NPCs only. Value used to determine if an attack is repelled - NPCのみ。攻撃がはじかれるかどうかの判定に利用する値
    pub guardBreakRate: u16,

    /// NAME: pad - pad
    pub pad6: [u8; 1],

    /// NAME: Can damage bushes - 茂みにダメージ可
    /// DESC: Do you want to calculate damage for assets that are "Break due to bush damage"? To set. 〇: Calculate, ×: Do not calculate (that is, you cannot inflict damage) [GR] SEQ20617 - 「茂みダメージで壊れるか」ONのアセットに対してダメージ計算をするか？を設定します。〇：計算する、×：計算しない(つまりダメージをあたえることはできない)【GR】SEQ20617
    pub isEnableCalcDamageForBushesObj: u8,

    /// NAME: Throw-through attack power - 投げ抜け攻撃力
    /// DESC: Throw-through attack power - 投げ抜け攻撃力
    pub atkThrowEscape: u16,

    /// NAME: Object attack power - オブジェ攻撃力
    /// DESC: Attack power against OBJ - ＯＢＪに対する攻撃力
    pub atkObj: u16,

    /// NAME: Stamina cut rate correction when guarding - ガード時スタミナカット率補正
    /// DESC: Correct the [stamina cut rate when guarding] set in the weapon parameter and NPC parameter. - 武器パラメータ、ＮＰＣパラメータに設定されている【ガード時スタミナカット率】を補正する
    pub guardStaminaCutRate: i16,

    /// NAME: Guard magnification - ガード倍率
    /// DESC: NPC, the guard performance set in the weapon parameter is uniformly corrected. 0, 1x / 100, 2x / -100, guard multiplier = (guard multiplier / 100) to increase / decrease the parameter to 0. + 1) - ＮＰＣ、武器パラメータで設定してあるガード性能を一律で補正を掛ける0で、1倍／100で、2倍／－100で、0　にパラメータが増減するようにするガード倍率　=　（ガード倍率/100　+　1）
    pub guardRate: i16,

    /// NAME: Throw type ID - 投げタイプID
    /// DESC: ID associated with the throw parameter - 投げパラメータと紐付けされているID
    pub throwTypeId: u16,

    /// NAME: 0 parts per - あたり0 部位
    /// DESC: Hit part - あたり部位
    pub hit0_hitType: u8,

    /// NAME: 1 part per part - あたり1 部位
    /// DESC: Hit part - あたり部位
    pub hit1_hitType: u8,

    /// NAME: 2 parts per - あたり2 部位
    /// DESC: Hit part - あたり部位
    pub hit2_hitType: u8,

    /// NAME: 3 parts per - あたり3 部位
    /// DESC: Hit part - あたり部位
    pub hit3_hitType: u8,

    /// NAME: 0 Priority per - あたり0 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti0_Priority: u8,

    /// NAME: 1 priority per - あたり1 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti1_Priority: u8,

    /// NAME: 2 priorities per - あたり2 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti2_Priority: u8,

    /// NAME: 3 priorities per - あたり3 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti3_Priority: u8,

    /// NAME: Damage level - ダメージレベル
    /// DESC: Which damage motion should be played against the enemy when attacking? To decide. - 攻撃したとき、敵にどのダメージモーションを再生するか？を決める.
    pub dmgLevel: u8,

    /// NAME: See per map - マップあたり参照
    /// DESC: Which map do you see around the attack? The set - 攻撃あたりが、どのマップあたりを見るか？を設定
    pub mapHitType: u8,

    /// NAME: Guard cut rate invalidation factor - ガードカット率無効化倍率
    /// DESC: Guard cut rate invalidation ratio (-100 to 100) → Normal at 0 / Completely invalidated at -100 / Doubles the defense effect of the opponent at 100 → -50, 100% cut shield becomes 50% cut Become  - ガードカット率無効化倍率（－100～100）　→0のとき通常／－100で完全無効化／100で相手の防御効果倍増 　→－50とすれば、100％カットの盾が、50％カットになります
    pub guardCutCancelRate: i8,

    /// NAME: Physical attributes - 物理属性
    /// DESC: Physical attributes to set for attacks - 攻撃に設定する物理属性
    pub atkAttribute: u8,

    /// NAME: Special attributes - 特殊属性
    /// DESC: Special attributes to set for attacks - 攻撃に設定する特殊属性
    pub spAttribute: u8,

    /// NAME: Attack attribute [SFX / SE] - 攻撃属性[SFX/SE]
    /// DESC: Specify SFX / SE at the time of attack (1 set by attribute, material, size) - 攻撃時のSFX/SEを指定(属性、材質、サイズで1セット)
    pub atkType: u8,

    /// NAME: Attack material [SFX / SE] - 攻撃材質[SFX/SE]
    /// DESC: Specify SFX / SE at the time of attack (1 set by attribute, material, size) - 攻撃時のSFX/SEを指定(属性、材質、サイズで1セット)
    pub atkMaterial: u8,

    /// NAME: Guard judgment position - ガード判定位置
    /// DESC: Guard judgment position - ガード判定位置
    pub guardRangeType: u8,

    /// NAME: Defensive material 1 [SE] - 防御材質1[SE]
    /// DESC: Used for SE when guarding 1 - ガード時のSEに使用1
    pub defSeMaterial1: u16,

    /// NAME: Source per - あたり発生源
    /// DESC: Where do you get the Damipoli ID per attack? To specify - 攻撃あたりのダミポリＩＤをどこから取ってくるか？を指定する
    pub hitSourceType: u8,

    /// NAME: Throw - 投げ
    /// DESC: Flag used for throwing information - 投げ情報に用いるフラグ
    pub throwFlag: u8,

    /// NAME: Unguardable flag - ガード不可フラグ
    /// DESC: If 1, ignore the guard on the guard side and enter the damage level - 1の場合、ガード側のガードを無視して、ダメージレベルを入れる
    pub Bitfield1: u8,

    /// NAME: Attack strength [SFX] - 攻撃強度[SFX]
    /// DESC: Attack strength [SFX] - 攻撃強度[SFX]
    pub atkPow_forSfx: i8,

    /// NAME: Attack direction [SFX] - 攻撃方向[SFX]
    /// DESC: Attack direction [SFX] - 攻撃方向[SFX]
    pub atkDir_forSfx: i8,

    /// NAME: Target: ● Hostile - 対象：●敵対
    /// DESC: Target: ● Hostile - 対象：●敵対
    pub Bitfield2: u8,

    /// NAME: Behavior identification value 1 - Behavior用識別値1
    /// DESC: Behavior identification value: Extra large damage transition - Behavior用識別値：特大ダメージ遷移
    pub atkBehaviorId: u8,

    /// NAME: Attack strength [SE] - 攻撃強度[SE]
    /// DESC: Attack strength [SE] - 攻撃強度[SE]
    pub atkPow_forSe: i8,

    /// NAME: SA attack power - SA攻撃力
    /// DESC: NPCs only. Value used for SA break calculation formula - NPCのみ。SAブレイク計算式に利用すする値
    pub atkSuperArmor: f32,

    /// NAME: Decal ID 1 (directly specified) - デカールID1（直接指定）
    /// DESC: Decal ID 1 (directly specified) - デカールID1（直接指定）
    pub decalId1: i32,

    /// NAME: Decal ID 2 (directly specified) - デカールID2（直接指定）
    /// DESC: Decal ID 2 (directly specified) - デカールID2（直接指定）
    pub decalId2: i32,

    /// NAME: AI sound ID when it occurs - 発生時AI音ID
    /// DESC: ID of AI sound generated when an attack occurs - 攻撃発生時に発生させるAI音のID
    pub AppearAiSoundId: i32,

    /// NAME: AI sound ID on hit - ヒット時AI音ID
    /// DESC: ID of AI sound generated at the time of hit - ヒット時に発生させるAI音のID
    pub HitAiSoundId: i32,

    /// NAME: Vibration effect on hit (-1 invalid) - ヒット時振動効果(-1無効)
    /// DESC: Vibration ID at the time of hit (-1 invalid). It is a vibration ID when none of the following three applies - ヒット時の振動ID（-1無効）。次の3つのどれにも当てはまらない時の振動IDとなる
    pub HitRumbleId: i32,

    /// NAME: Vibration ID when the tip hits - 先端ヒット時振動ID
    /// DESC: Vibration ID at the time of hit when hitting the tip (-1 invalid) - 先端にヒットした時のヒット時振動ID（-1無効）
    pub HitRumbleIdByNormal: i32,

    /// NAME: Vibration ID when hit in the middle - 真ん中ヒット時振動ID
    /// DESC: Vibration ID at the time of hit when hit in the middle (-1 invalid) - 真ん中にヒットした時のヒット時振動ID（-1無効）
    pub HitRumbleIdByMiddle: i32,

    /// NAME: Vibration ID at the time of root hit - 根本ヒット時振動ID
    /// DESC: Vibration ID at the time of hit when hitting the root (-1 invalid) - 根本にヒットした時のヒット時振動ID（-1無効）
    pub HitRumbleIdByRoot: i32,

    /// NAME: Sword Flash SfxID_0 - 剣閃SfxID_０
    /// DESC: Sword flash SfxID_0 (-1 invalid) - 剣閃SfxID_０(-1無効)
    pub traceSfxId0: i32,

    /// NAME: Root Sword Flash Damipoli ID_0 - 根元剣閃ダミポリID_０
    /// DESC: Sword flash root Damipoli ID_0 (-1 invalid) - 剣閃根元ダミポリID_０(-1無効)
    pub traceDmyIdHead0: i32,

    /// NAME: Sword tip sword flash Damipoli ID_0 - 剣先剣閃ダミポリID_０
    /// DESC: Sword Flash Sword Tip Damipoli ID_0 - 剣閃剣先ダミポリID_０
    pub traceDmyIdTail0: i32,

    /// NAME: Sword Flash SfxID_1 - 剣閃SfxID_１
    /// DESC: Sword flash SfxID_1 (-1 invalid) - 剣閃SfxID_１(-1無効)
    pub traceSfxId1: i32,

    /// NAME: Root Sword Flash Damipoli ID_1 - 根元剣閃ダミポリID_１
    /// DESC: Sword Flash Root Damipoli ID_1 (-1 invalid) - 剣閃根元ダミポリID_１(-1無効)
    pub traceDmyIdHead1: i32,

    /// NAME: Sword tip sword flash Damipoli ID_1 - 剣先剣閃ダミポリID_１
    /// DESC: Sword Flash Sword Tip Damipoli ID_1 - 剣閃剣先ダミポリID_１
    pub traceDmyIdTail1: i32,

    /// NAME: Sword Flash SfxID_2 - 剣閃SfxID_２
    /// DESC: Sword flash SfxID_2 (-1 invalid) - 剣閃SfxID_２(-1無効)
    pub traceSfxId2: i32,

    /// NAME: Root Sword Flash Damipoli ID_2 - 根元剣閃ダミポリID_２
    /// DESC: Sword Flash Root Damipoli ID_2 (-1 invalid) - 剣閃根元ダミポリID_２(-1無効)
    pub traceDmyIdHead2: i32,

    /// NAME: Sword tip sword flash Damipoli ID_2 - 剣先剣閃ダミポリID_２
    /// DESC: Sword Flash Sword Tip Damipoli ID_2 - 剣閃剣先ダミポリID_２
    pub traceDmyIdTail2: i32,

    /// NAME: Sword Flash SfxID_3 - 剣閃SfxID_３
    /// DESC: Sword Flash SfxID_3 (-1 invalid) - 剣閃SfxID_３(-1無効)
    pub traceSfxId3: i32,

    /// NAME: Root Sword Flash Damipoli ID_3 - 根元剣閃ダミポリID_３
    /// DESC: Sword flash root Damipoli ID_3 (-1 invalid) - 剣閃根元ダミポリID_３(-1無効)
    pub traceDmyIdHead3: i32,

    /// NAME: Sword tip sword flash Damipoli ID_3 - 剣先剣閃ダミポリID_３
    /// DESC: Sword Flash Sword Tip Damipoli ID_3 - 剣閃剣先ダミポリID_３
    pub traceDmyIdTail3: i32,

    /// NAME: Sword Flash SfxID_4 - 剣閃SfxID_４
    /// DESC: Sword Flash SfxID_4 (-1 invalid) - 剣閃SfxID_４(-1無効)
    pub traceSfxId4: i32,

    /// NAME: Root Sword Flash Damipoli ID_4 - 根元剣閃ダミポリID_４
    /// DESC: Sword flash root Damipoli ID_4 (-1 invalid) - 剣閃根元ダミポリID_４(-1無効)
    pub traceDmyIdHead4: i32,

    /// NAME: Sword tip sword flash Damipoli ID_4 - 剣先剣閃ダミポリID_４
    /// DESC: Sword Flash Sword Tip Damipoli ID_4 - 剣閃剣先ダミポリID_４
    pub traceDmyIdTail4: i32,

    /// NAME: Sword Flash SfxID_5 - 剣閃SfxID_５
    /// DESC: Sword Flash SfxID_5 (-1 invalid) - 剣閃SfxID_５(-1無効)
    pub traceSfxId5: i32,

    /// NAME: Root Sword Flash Damipoli ID_5 - 根元剣閃ダミポリID_５
    /// DESC: Sword Flash Root Damipoli ID_5 (-1 invalid) - 剣閃根元ダミポリID_５(-1無効)
    pub traceDmyIdHead5: i32,

    /// NAME: Sword tip sword flash Damipoli ID_5 - 剣先剣閃ダミポリID_５
    /// DESC: Sword Flash Sword Tip Damipoli ID_5 - 剣閃剣先ダミポリID_５
    pub traceDmyIdTail5: i32,

    /// NAME: Sword Flash SfxID_6 - 剣閃SfxID_６
    /// DESC: Sword Flash SfxID_6 (-1 invalid) - 剣閃SfxID_６(-1無効)
    pub traceSfxId6: i32,

    /// NAME: Root Sword Flash Damipoli ID_6 - 根元剣閃ダミポリID_６
    /// DESC: Sword Flash Root Damipoli ID_6 (-1 invalid) - 剣閃根元ダミポリID_６(-1無効)
    pub traceDmyIdHead6: i32,

    /// NAME: Sword tip sword flash Damipoli ID_6 - 剣先剣閃ダミポリID_６
    /// DESC: Sword Flash Sword Tip Damipoli ID_6 - 剣閃剣先ダミポリID_６
    pub traceDmyIdTail6: i32,

    /// NAME: Sword Flash SfxID_7 - 剣閃SfxID_７
    /// DESC: Sword Flash SfxID_7 (-1 invalid) - 剣閃SfxID_７(-1無効)
    pub traceSfxId7: i32,

    /// NAME: Root Sword Flash Damipoli ID_7 - 根元剣閃ダミポリID_７
    /// DESC: Sword Flash Root Damipoli ID_7 (-1 invalid) - 剣閃根元ダミポリID_７(-1無効)
    pub traceDmyIdHead7: i32,

    /// NAME: Sword tip sword flash Damipoli ID_7 - 剣先剣閃ダミポリID_７
    /// DESC: Sword Flash Sword Tip Damipoli ID_7 - 剣閃剣先ダミポリID_７
    pub traceDmyIdTail7: i32,

    /// NAME: 4 radii per - あたり4 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit4_Radius: f32,

    /// NAME: 5 radii per - あたり5 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit5_Radius: f32,

    /// NAME: 6 radii per - あたり6 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit6_Radius: f32,

    /// NAME: 7 radii per - あたり7 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit7_Radius: f32,

    /// NAME: 8 radii per - あたり8 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit8_Radius: f32,

    /// NAME: 9 radii per - あたり9 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit9_Radius: f32,

    /// NAME: 10 radii per - あたり10 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit10_Radius: f32,

    /// NAME: 11 radii per - あたり11 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit11_Radius: f32,

    /// NAME: 12 radii per - あたり12 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit12_Radius: f32,

    /// NAME: 13 radii per - あたり13 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit13_Radius: f32,

    /// NAME: 14 radii per - あたり14 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit14_Radius: f32,

    /// NAME: 15 radii per - あたり15 半径
    /// DESC: Sphere, capsule radius - 球、カプセルの半径
    pub hit15_Radius: f32,

    /// NAME: 4 per Damipoli 1 - あたり4 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit4_DmyPoly1: i16,

    /// NAME: 5 per Damipoli 1 - あたり5 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit5_DmyPoly1: i16,

    /// NAME: 6 per Damipoli 1 - あたり6 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit6_DmyPoly1: i16,

    /// NAME: 7 per Damipoli 1 - あたり7 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit7_DmyPoly1: i16,

    /// NAME: 8 Damipoli per 1 - あたり8ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit8_DmyPoly1: i16,

    /// NAME: 9 per Damipoli 1 - あたり9 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit9_DmyPoly1: i16,

    /// NAME: 10 per Damipoli 1 - あたり10 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit10_DmyPoly1: i16,

    /// NAME: Per 11 Damipoli 1 - あたり11 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit11_DmyPoly1: i16,

    /// NAME: 12 per Damipoli 1 - あたり12 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit12_DmyPoly1: i16,

    /// NAME: 13 Damipoli per 1 - あたり13ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit13_DmyPoly1: i16,

    /// NAME: Per 14 Damipoli 1 - あたり14 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit14_DmyPoly1: i16,

    /// NAME: 15 per Damipoli 1 - あたり15 ダミポリ1
    /// DESC: Damipoli in sphere, capsule position - 球、カプセル位置のダミポリ
    pub hit15_DmyPoly1: i16,

    /// NAME: 4 per Damipoli 2 - あたり4 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit4_DmyPoly2: i16,

    /// NAME: 5 Damipoli 2 per - あたり5ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit5_DmyPoly2: i16,

    /// NAME: 6 Damipoli 2 per - あたり6ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit6_DmyPoly2: i16,

    /// NAME: 7 Damipoli 2 per - あたり7ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit7_DmyPoly2: i16,

    /// NAME: 8 per Damipoli 2 - あたり8 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit8_DmyPoly2: i16,

    /// NAME: 9 Damipoli per 2 - あたり9ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit9_DmyPoly2: i16,

    /// NAME: 10 per Damipoli 2 - あたり10 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit10_DmyPoly2: i16,

    /// NAME: Per 11 Damipoli 2 - あたり11 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit11_DmyPoly2: i16,

    /// NAME: 12 per Damipoli 2 - あたり12 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit12_DmyPoly2: i16,

    /// NAME: Per 13 Damipoli 2 - あたり13 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit13_DmyPoly2: i16,

    /// NAME: Per 14 Damipoli 2 - あたり14 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit14_DmyPoly2: i16,

    /// NAME: 15 per Damipoli 2 - あたり15 ダミポリ2
    /// DESC: The position of another point on the capsule Damipoli. -1 makes it a sphere - カプセルのもうひとつの点の位置ダミポリ。-1だと球になる
    pub hit15_DmyPoly2: i16,

    /// NAME: 4 parts per - あたり4 部位
    /// DESC: Hit part - あたり部位
    pub hit4_hitType: u8,

    /// NAME: 5 parts per - あたり5 部位
    /// DESC: Hit part - あたり部位
    pub hit5_hitType: u8,

    /// NAME: 6 parts per - あたり6 部位
    /// DESC: Hit part - あたり部位
    pub hit6_hitType: u8,

    /// NAME: 7 parts per - あたり7 部位
    /// DESC: Hit part - あたり部位
    pub hit7_hitType: u8,

    /// NAME: 8 parts per - あたり8 部位
    /// DESC: Hit part - あたり部位
    pub hit8_hitType: u8,

    /// NAME: 9 parts per - あたり9 部位
    /// DESC: Hit part - あたり部位
    pub hit9_hitType: u8,

    /// NAME: 10 parts per - あたり10 部位
    /// DESC: Hit part - あたり部位
    pub hit10_hitType: u8,

    /// NAME: 11 parts per - あたり11 部位
    /// DESC: Hit part - あたり部位
    pub hit11_hitType: u8,

    /// NAME: 12 parts per - あたり12 部位
    /// DESC: Hit part - あたり部位
    pub hit12_hitType: u8,

    /// NAME: 13 parts per - あたり13 部位
    /// DESC: Hit part - あたり部位
    pub hit13_hitType: u8,

    /// NAME: 14 parts per - あたり14 部位
    /// DESC: Hit part - あたり部位
    pub hit14_hitType: u8,

    /// NAME: 15 parts per - あたり15 部位
    /// DESC: Hit part - あたり部位
    pub hit15_hitType: u8,

    /// NAME: 4 priorities per - あたり4 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti4_Priority: u8,

    /// NAME: 5 priorities per - あたり5 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti5_Priority: u8,

    /// NAME: 6 priorities per - あたり6 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti6_Priority: u8,

    /// NAME: 7 priorities per - あたり7 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti7_Priority: u8,

    /// NAME: 8 priorities per - あたり8 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti8_Priority: u8,

    /// NAME: 9 priorities per - あたり9 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti9_Priority: u8,

    /// NAME: 10 priorities per - あたり10 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti10_Priority: u8,

    /// NAME: 11 priorities per - あたり11 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti11_Priority: u8,

    /// NAME: 12 priorities per - あたり12 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti12_Priority: u8,

    /// NAME: 13 priorities per - あたり13 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti13_Priority: u8,

    /// NAME: 14 priorities per - あたり14 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti14_Priority: u8,

    /// NAME: 15 priorities per - あたり15 優先順位
    /// DESC: priority. If there are two or more hits at the same time, the one with the higher priority will be adopted. - 優先度。同時に2つ以上のあたりがあたった場合、優先度が高いほうを採用する。
    pub hti15_Priority: u8,

    /// NAME: Defensive material 1 [SFX] - 防御材質1[SFX]
    /// DESC: Used for SFX when guarding. 1 - ガード時のSFXに使用.1
    pub defSfxMaterial1: u16,

    /// NAME: Defensive material 2 [SE] - 防御材質2[SE]
    /// DESC: Used for SE when guarding 2 - ガード時のSEに使用2
    pub defSeMaterial2: u16,

    /// NAME: Defensive material 2 [SFX] - 防御材質2[SFX]
    /// DESC: Used for SFX when guarding. 2 - ガード時のSFXに使用.2
    pub defSfxMaterial2: u16,

    /// NAME: Dark attack power correction value - 闇攻撃力補正値
    /// DESC: PC only. Multiply the dark attack power (in the case of a bow, correct the missile) - PCのみ。闇攻撃力に掛ける倍率（弓の場合は、飛び道具を補正）
    pub atkDarkCorrection: u16,

    /// NAME: Dark attack power - 闇攻撃力
    /// DESC: NPCs only. Additional damage from dark attacks - NPCのみ。闇攻撃の追加ダメージ
    pub atkDark: u16,

    /// NAME: pad - pad
    /// DESC: pad - pad
    pub Bitfield3: u8,

    /// NAME: Damage level vs. player - ダメージレベル 対プレイヤー
    /// DESC: Damage level to the player. If it is "0 (default)", it is not used. The meaning of the range other than "0 (default)" is the same as "Damage level". - プレイヤーに対するダメージレベル。“0(デフォルト)”であれば使わない。“0(デフォルト)”以外の値域の意味は、《ダメージレベル》と同じ。
    pub dmgLevel_vsPlayer: i8,

    /// NAME: Abnormal state attack power magnification correction - 状態異常攻撃力倍率補正
    /// DESC: Magnification correction is performed for the abnormal state attack power of special effects. - 特殊効果の状態異常攻撃力に対して、倍率補正を行う。
    pub statusAilmentAtkPowerCorrectRate: u16,

    /// NAME: Special effects attack power multiplier correction (attack power points) - 特殊効果攻撃力倍率補正（攻撃力ポイント）
    /// DESC: Magnification correction is performed for the special effect ~ ~ attack power [point]. - 特殊効果の～～攻撃力[point]に対して、倍率補正を行う。
    pub spEffectAtkPowerCorrectRate_byPoint: u16,

    /// NAME: Special effect attack power multiplier correction (attack power multiplier) - 特殊効果攻撃力倍率補正（攻撃力倍率）
    /// DESC: Magnification is corrected for the special effect's attack power multiplier. - 特殊効果の～～攻撃力倍率に対して、倍率補正を行う。
    pub spEffectAtkPowerCorrectRate_byRate: u16,

    /// NAME: Special effect attack power multiplier correction (final attack power multiplier) - 特殊効果攻撃力倍率補正（最終攻撃力倍率）
    /// DESC: Attack side of special effect: ~ ~ Performs magnification correction for damage multiplier. - 特殊効果の攻撃側：～～ダメージ倍率に対して、倍率補正を行う。
    pub spEffectAtkPowerCorrectRate_byDmg: u16,

    /// NAME: Behavior identification value 2 - Behavior用識別値2
    /// DESC: Behavior identification value: Plays damage motion only at specific times - Behavior用識別値：特定の時だけダメージモーションを再生する
    pub atkBehaviorId_2: u8,

    /// NAME: Throw damage attribute - 投げダメージ属性
    /// DESC: Attribute of throw damage of attack judgment. Corresponding special effects will be applied. It works only when the attack ATK_PATAM_THROWFLAG_TYPE is "2: Throw". - 攻撃判定の投げダメージの属性。対応する特殊効果がかかるようになる。攻撃のATK_PATAM_THROWFLAG_TYPEが「2：投げ」の場合にのみ、機能を発揮する
    pub throwDamageAttribute: u8,

    /// NAME: Special effect status abnormality correction (attack power point) - 特殊効果状態異常補正（攻撃力ポイント）
    /// DESC: Magnification correction is performed for the special effect "Whether to apply the abnormal state attack power magnification correction". - 特殊効果の「状態異常攻撃力倍率補正を適応するか」に対して、倍率補正を行う。
    pub statusAilmentAtkPowerCorrectRate_byPoint: u16,

    /// NAME: Attack attribute correction ID overwrite - 攻撃属性補正ID上書き
    /// DESC: For overwriting the ID of the parameter that corrects the attack attribute - 攻撃属性を補正するパラメータのID上書き用
    pub overwriteAttackElementCorrectId: i32,

    /// NAME: Decal identifier 1 - デカール識別子1
    /// DESC: Decal identifier 1 (3 digits) - デカール識別子1(3桁)
    pub decalBaseId1: i16,

    /// NAME: Decal identifier 2 - デカール識別子2
    /// DESC: Decal identifier 2 (3 digits) - デカール識別子2(3桁)
    pub decalBaseId2: i16,

    /// NAME: Weapon regain amount correction value - 武器リゲイン量補正値
    /// DESC: Weapon regain amount correction value - 武器リゲイン量補正値
    pub wepRegainHpScale: u16,

    /// NAME: Amount of attack regain - 攻撃リゲイン量
    /// DESC: Amount of attack regain - 攻撃リゲイン量
    pub atkRegainHp: u16,

    /// NAME: Regainable time correction factor - リゲイン可能時間補正倍率
    /// DESC: Regainable time correction factor - リゲイン可能時間補正倍率
    pub regainableTimeScale: f32,

    /// NAME: Regainable rate correction factor - リゲイン可能率補正倍率
    /// DESC: Regainable rate correction factor - リゲイン可能率補正倍率
    pub regainableHpRateScale: f32,

    /// NAME: Same attack judgment ID - 同一攻撃判定ID
    /// DESC: Same attack judgment ID - 同一攻撃判定ID
    pub regainableSlotId: i8,

    /// NAME: Special attribute variation value - 特殊属性バリエーション値
    /// DESC: Value for giving variation to SFX and SE generated by special attribute in combination with "special attribute" (SEQ16473) - 「特殊属性」と組み合わせて特殊属性によって発生するSFX、SEにバリエーションを持たせるための値(SEQ16473)
    pub spAttributeVariationValue: u8,

    /// NAME: Front angle offset of parry establishment condition - パリィ成立条件の正面角度オフセット
    /// DESC: Front angle offset of [collapsed side] of parry establishment condition - パリィ成立条件の【崩される側】の正面角度オフセット
    pub parryForwardOffset: i16,

    /// NAME: SA attack power correction value - SA攻撃力補正値
    /// DESC: PC only. Correction value to be applied to the [basic value] set for the weapon - PCのみ。武器に設定された【基本値】にかける補正値
    pub atkSuperArmorCorrection: f32,

    /// NAME: Defensive material variation value - 防御材質バリエーション値
    /// DESC: A value to have variations of damage SFX and SE in combination with "defense material 1 or 2" used when guarding. (SEQ16473) - ガード時に使用される「防御材質1or2」と組み合わせてダメージSFX、SEのバリエーションを持たせるための値。(SEQ16473)
    pub defSfxMaterialVariationValue: u8,

    /// NAME: pad - pad
    pub pad4: [u8; 3],

    /// NAME: finalDamageRateId
    pub finalDamageRateId: i32,

    /// NAME: pad7
    pub pad7: [u8; 12],
}

impl Paramdef for ATK_PARAM_ST {
    const NAME: &'static str = "ATK_PARAM_ST";
    const VERSION: u16 = 4;
}
impl ATK_PARAM_ST {
    /// If 1, ignore the guard on the guard side and enter the damage level - 1の場合、ガード側のガードを無視して、ダメージレベルを入れる
    /// Bitfield1
    pub fn get_disableGuard(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_disableGuard(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// "Destruction judgment" is performed by stamina attack power, but stamina is not actually reduced. - スタミナ攻撃力による「崩され判定」は行うが、実際にスタミナは減らさない
    /// Bitfield1
    pub fn get_disableStaminaAttack(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_disableStaminaAttack(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Disables special effects when an attack hits. SCE bug countermeasures - 攻撃ヒットしたときの特殊効果を無効にします。SCEバグ対策
    /// Bitfield1
    pub fn get_disableHitSpEffect(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_disableHitSpEffect(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// Do not notify AI of missed swing - AIに空振り通知しない
    /// Bitfield1
    pub fn get_IgnoreNotifyMissSwingForAI(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_IgnoreNotifyMissSwingForAI(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
    /// Enemy only: Does SFX occur continuously when hitting a wall? - 敵専用：壁Hit時のSFXが連続で発生するか
    /// Bitfield1
    pub fn get_repeatHitSfx(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_repeatHitSfx(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xEF
        }
    }
    /// Used for site damage judgment. - 部位ダメージ判定に使用する。
    /// Bitfield1
    pub fn get_isArrowAtk(&self) -> bool {
        &self.Bitfield1 & 0x20 != 0
    }

    /// Bitfield1
    pub fn set_isArrowAtk(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x20
        } else {
            self.Bitfield1 &= 0xDF
        }
    }
    /// Used for determining spirit damage. - 霊体ダメージ判定に使用。
    /// Bitfield1
    pub fn get_isGhostAtk(&self) -> bool {
        &self.Bitfield1 & 0x40 != 0
    }

    /// Bitfield1
    pub fn set_isGhostAtk(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x40
        } else {
            self.Bitfield1 &= 0xBF
        }
    }
    /// Ignore invincible effects such as steps, TAE's complete invincibility cannot be ignored. - ステップ等の無敵効果を無視します、TAEの完全無敵は無視できません。
    /// Bitfield1
    pub fn get_isDisableNoDamage(&self) -> bool {
        &self.Bitfield1 & 0x80 != 0
    }

    /// Bitfield1
    pub fn set_isDisableNoDamage(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x80
        } else {
            self.Bitfield1 &= 0x7F
        }
    }
    /// Target: ● Hostile - 対象：●敵対
    /// Bitfield2
    pub fn get_opposeTarget(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_opposeTarget(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Target: ○ Allies - 対象：○味方
    /// Bitfield2
    pub fn get_friendlyTarget(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_friendlyTarget(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Target: myself - 対象：自分
    /// Bitfield2
    pub fn get_selfTarget(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_selfTarget(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Whether to check the door penetration. In the case of ○, it is judged whether or not the target through the door can be attacked. - 扉貫通チェックを行うかどうか。○の場合は扉越しの対象を攻撃できるかどうかの判定を行います。
    /// Bitfield2
    pub fn get_isCheckDoorPenetration(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_isCheckDoorPenetration(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    /// If you hit the target of the riding special attack while riding, the SA damage will be multiplied by the multiplier. - 騎乗中の騎乗特攻対象に攻撃を当てた場合、SAダメージに倍率補正が掛かる
    /// Bitfield2
    pub fn get_isVsRideAtk(&self) -> bool {
        &self.Bitfield2 & 0x10 != 0
    }

    /// Bitfield2
    pub fn set_isVsRideAtk(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x10
        } else {
            self.Bitfield2 &= 0xEF
        }
    }
    /// Do you refer to the additional attack power even in weapon attacks? - 武器攻撃でも加算攻撃力を参照するか
    /// Bitfield2
    pub fn get_isAddBaseAtk(&self) -> bool {
        &self.Bitfield2 & 0x20 != 0
    }

    /// Bitfield2
    pub fn set_isAddBaseAtk(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x20
        } else {
            self.Bitfield2 &= 0xDF
        }
    }
    /// Is it excluded from threat level notification? - 脅威度通知対象除外か
    /// Bitfield2
    pub fn get_excludeThreatLvNotify(&self) -> bool {
        &self.Bitfield2 & 0x40 != 0
    }

    /// Bitfield2
    pub fn set_excludeThreatLvNotify(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x40
        } else {
            self.Bitfield2 &= 0xBF
        }
    }
    ///
    /// Bitfield2
    pub fn get_pad1(&self) -> bool {
        &self.Bitfield2 & 0x80 != 0
    }

    /// Bitfield2
    pub fn set_pad1(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x80
        } else {
            self.Bitfield2 &= 0x7F
        }
    }
    /// pad - pad
    /// Bitfield3
    pub fn get_pad5(&self) -> bool {
        &self.Bitfield3 & 0x1 != 0
    }

    /// Bitfield3
    pub fn set_pad5(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x1
        } else {
            self.Bitfield3 &= 0xFE
        }
    }
    /// This is a flag to disable the new parry control. A process that determines that the damage on the attacking side has been parried when it comes into contact with a character in the parry state on the defending side. - 新パリィ制御を無効化するかどうかのフラグです。攻撃側のダメージが、防御側でパリィ状態のキャラに接触した場合にパリィされたと判定する処理。
    /// Bitfield3
    pub fn get_isDisableParry(&self) -> bool {
        &self.Bitfield3 & 0x2 != 0
    }

    /// Bitfield3
    pub fn set_isDisableParry(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x2
        } else {
            self.Bitfield3 &= 0xFD
        }
    }
    /// Avoid using the 1.5x growth status adaptation with both hands - 両手時の成長ステータス1.5倍適応を使わないようにする
    /// Bitfield3
    pub fn get_isDisableBothHandsAtkBonus(&self) -> bool {
        &self.Bitfield3 & 0x4 != 0
    }

    /// Bitfield3
    pub fn set_isDisableBothHandsAtkBonus(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x4
        } else {
            self.Bitfield3 &= 0xFB
        }
    }
    /// If "Do you want to penetrate invincibility" is ◯, this setting will be ignored. - 「無敵を貫通するか」が◯の場合、この設定は無視されます
    /// Bitfield3
    pub fn get_isInvalidatedByNoDamageInAir(&self) -> bool {
        &self.Bitfield3 & 0x8 != 0
    }

    /// Bitfield3
    pub fn set_isInvalidatedByNoDamageInAir(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x8
        } else {
            self.Bitfield3 &= 0xF7
        }
    }
    ///
    /// Bitfield3
    pub fn get_pad2(&self) -> u8 {
        &self.Bitfield3 & 0xF0
    }

    /// Bitfield3 MAX: 15
    pub fn set_pad2(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 4) & 0xF0;
            let newVal = &self.Bitfield3 & 0xF | val;
            self.Bitfield3 = newVal
        } else {
            self.Bitfield3 &= 0xF
        }
    }
}
