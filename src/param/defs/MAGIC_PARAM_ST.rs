/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 6
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MAGIC_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Yes / No Dialog Message ID - Yes/NoダイアログメッセージID
    /// DESC: Message ID of Yes / No dialog issued when using magic - 魔法使用時に出すYes/NoダイアログのメッセージID
    pub yesNoDialogMessageId: i32,

    /// NAME: Special effect ID that is out of usage restrictions - 使用制限から外れる特殊効果ID
    /// DESC: Usage restrictions can be ignored when the specified special effect ID is activated - 指定した特殊効果IDが発動している時は使用制限を無視できる
    pub limitCancelSpEffectId: i32,

    /// NAME: SortID - SortID
    /// DESC: Sort ID (-1: Do not collect) - ソートID(-1:集めない)
    pub sortId: i16,

    /// NAME: Equipment conditions [luck] - 装備条件【運】
    /// DESC: Can't be equipped without more PC luck - PCの運がこれ以上無いと装備できない
    pub requirementLuck: u8,

    /// NAME: AI notification type - AI通知タイプ
    /// DESC: AI interrupt occurs in act ("AI notification when magic is activated") - act("魔法発動時AI通知")でAIインタラプトが発生する
    pub aiNotifyType: u8,

    /// NAME: MP consumption [normal] - 消費MP[通常]
    /// DESC: MP consumption - 消費MP
    pub mp: i16,

    /// NAME: Consumed stamina [normal] - 消費スタミナ[通常]
    /// DESC: Consumption stamina - 消費スタミナ
    pub stamina: i16,

    /// NAME: Icon ID - アイコンID
    /// DESC: Specify icon> For menu - アイコンを指定　＞メニュー用
    pub iconId: i16,

    /// NAME: Action ID - 行動ID
    /// DESC: Set the action ID - 行動IDを設定する
    pub behaviorId: i16,

    /// NAME: Required item ID - 必要アイテムID
    /// DESC: Item ID required for purchase - 購入に必要なアイテムID
    pub mtrlItemId: i16,

    /// NAME: Magic ID to replace - 差し替える魔法ID
    /// DESC: ID to be replaced when the state change matches (-1: invalid) - 状態変化一致時に差し替えるID(-1:無効)
    pub replaceMagicId: i16,

    /// NAME: Maximum number - 最大個数
    /// DESC: Number per piece (-1: infinity) - １個当たりの個数(-1:無限)
    pub maxQuantity: i16,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory1: u8,

    /// NAME: Skill over start value - 技量オーバー開始値
    /// DESC: Skill over start value - 技量オーバー開始値
    pub overDexterity: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory2: u8,

    /// NAME: Required slot - 必要スロット
    /// DESC: Number of slots required for equipment> For menu - 装備に必要なスロット数 ＞メニュー用
    pub slotLength: u8,

    /// NAME: Equipment conditions [Intelligence] - 装備条件【知力】
    /// DESC: Can't be equipped without more PC intelligence - PCの知力がこれ以上無いと装備できない
    pub requirementIntellect: u8,

    /// NAME: Equipment conditions [reason] - 装備条件【理力】
    /// DESC: It cannot be equipped without the power of the PC any more. - PCの理力がこれ以上無いと装備できない
    pub requirementFaith: u8,

    /// NAME: Analog minimum workmanship - アナログ最低技量
    /// DESC: Motion cancel analog: Minimum skill value - モーションキャンセルアナログ化：最低技量値
    pub analogDexterityMin: u8,

    /// NAME: Maximum analog workmanship - アナログ最大技量
    /// DESC: Motion cancel analog: Maximum skill value - モーションキャンセルアナログ化：最高技量値
    pub analogDexterityMax: u8,

    /// NAME: category - カテゴリ
    /// DESC: Used for sorting> For menu - 並べ替えに使用　＞メニュー用
    pub ezStateBehaviorType: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory3: u8,

    /// NAME: Special effects category - 特殊効果カテゴリ
    /// DESC: Since there are effects (enchantment weapons, etc.) whose parameters fluctuate depending on skills, magic, items, etc., set each action so that the determined effect can correspond to the effect such as "power up only weapon attack". Set "None" for items that do not need to be set, such as varistor. - スキルや、魔法、アイテムなどで、パラメータが変動する効果（エンチャントウェポンなど）があるので、│定した効果が、「武器攻撃のみをパワーアップする」といった効果に対応できるように行動ごとに設定するバリスタなど、設定の必要のないものは「なし」を設定する
    pub spEffectCategory: u8,

    /// NAME: Motion category - モーションカテゴリ
    /// DESC: Specify motion> For EzState - モーションを指定　＞EzState用
    pub refType: u8,

    /// NAME: Menu type at the time of use - 使用時メニュータイプ
    /// DESC: Menu type to be displayed when using magic - 魔法使用時に出すメニュータイプ
    pub opmeMenuType: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory4: u8,

    /// NAME: Which normal is it? - どの常態か？
    /// DESC: Specify the state change that needs to replace the magic ID - 魔法IDを差し替える必要がある状態変化を指定
    pub hasSpEffectType: u16,

    /// NAME: Replacement category - 差し替えカテゴリ
    /// DESC: Additional conditions when replacing the magic ID - 魔法IDを差し替える時の追加条件
    pub replaceCategory: u8,

    /// NAME: Restrictions on use by special effects category - 特殊効果カテゴリによる使用制限
    /// DESC: Specified to control availability by special effects - 特殊効果によって使用可能かどうかを制御する為に指定
    pub useLimitCategory: u8,

    /// NAME: Pledge 0 - 誓約0
    /// DESC: Pledge 0 - 誓約0
    pub Bitfield2: u8,

    /// NAME: Can it be used in multi? - マルチでも使用可能か
    /// DESC: Can it be used in multi? Can be used for both single and multi - マルチでも使用できるか。シングル、マルチ両方で使える
    pub Bitfield3: u8,

    /// NAME: Is it unavailable offline? - オフラインで使用不可か
    /// DESC: Is it unavailable offline? - オフラインで使用不可か
    pub Bitfield4: u8,

    /// NAME: Pledge 8 - 誓約8
    /// DESC: Pledge 8 - 誓約8
    pub Bitfield5: u8,

    /// NAME: Chanting SFXID - 詠唱SFXID
    /// DESC: SFXID during magical chanting - 魔法詠唱中のSFXID
    pub castSfxId: i32,

    /// NAME: Activate SFXID - 発動SFXID
    /// DESC: SFX ID when magic is activated - 魔法発動時のSFXID
    pub fireSfxId: i32,

    /// NAME: Effect SFXID - 効果SFXID
    /// DESC: SFXID during magic effect - 魔法効果中のSFXID
    pub effectSfxId: i32,

    /// NAME: Toughness correction factor - 強靭度 補正倍率
    /// DESC: It is a magnification that corrects the basic value of toughness. - 強靭度の基本値を補正する倍率です
    pub toughnessCorrectRate: f32,

    /// NAME: Replacement status type - 差し替えステータスタイプ
    /// DESC: Replacement status type - 差し替えステータスタイプ
    pub ReplacementStatusType: u8,

    /// NAME: Replacement status value 1 - 差し替えステータス値1
    /// DESC: Replacement status value 1 - 差し替えステータス値1
    pub ReplacementStatus1: i8,

    /// NAME: Replacement status value 2 - 差し替えステータス値2
    /// DESC: Replacement status value 2 - 差し替えステータス値2
    pub ReplacementStatus2: i8,

    /// NAME: Replacement status value 3 - 差し替えステータス値3
    /// DESC: Replacement status value 3 - 差し替えステータス値3
    pub ReplacementStatus3: i8,

    /// NAME: Replacement status value 4 - 差し替えステータス値4
    /// DESC: Replacement status value 4 - 差し替えステータス値4
    pub ReplacementStatus4: i8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory5: u8,

    /// NAME: Consume SA [Normal / Reservoir] - 消費SA[通常/溜め]
    /// DESC: SA consumption [normal / reservoir] - 消費SA量[通常/溜め]
    pub consumeSA: i16,

    /// NAME: Replacement ID1 - 差し替えID1
    /// DESC: Replacement ID1 - 差し替えID1
    pub ReplacementMagic1: i32,

    /// NAME: Replacement ID2 - 差し替えID2
    /// DESC: Replacement ID2 - 差し替えID2
    pub ReplacementMagic2: i32,

    /// NAME: Replacement ID3 - 差し替えID3
    /// DESC: Replacement ID3 - 差し替えID3
    pub ReplacementMagic3: i32,

    /// NAME: Replacement ID 4 - 差し替えID4
    /// DESC: Replacement ID 4 - 差し替えID4
    pub ReplacementMagic4: i32,

    /// NAME: MP consumption [reservoir] - 消費MP[溜め]
    /// DESC: MP consumption [reservoir] - 消費MP[溜め]
    pub mp_charge: i16,

    /// NAME: Consumption stamina [reservoir] - 消費スタミナ[溜め]
    /// DESC: Consumption stamina [reservoir] - 消費スタミナ[溜め]
    pub stamina_charge: i16,

    /// NAME: Creation limit group Id - 作成制限グループId
    /// DESC: If it is 0, it is unused. Check the number of bullets created for the specified group Id, and if the upper limit is reached, you will not be able to use magic. - 0なら未使用。指定したグループIdの弾丸作成数を確認し、上限に達していたら魔法の使用をできなくする。
    pub createLimitGroupId: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory6: u8,

    /// NAME: Subcategory 1 - サブカテゴリ1
    /// DESC: Subcategory 1 - サブカテゴリ1
    pub subCategory1: u8,

    /// NAME: Subcategory 2 - サブカテゴリ2
    /// DESC: Subcategory 2 - サブカテゴリ2
    pub subCategory2: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory7: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory8: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory9: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: Call ID category [Attack, Projectile, Special Effects] - 呼び出しIDのカテゴリ[攻撃、飛び道具、特殊効果]
    pub refCategory10: u8,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId1: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId2: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId3: i32,

    /// NAME: AI usage judgment ID - AI使用判断ID
    /// DESC: AI usage judgment ID - AI使用判断ID
    pub aiUseJudgeId: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId4: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId5: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId6: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId7: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId8: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId9: i32,

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from magic - 魔法から呼び出すID
    pub refId10: i32,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType1: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType2: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType3: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType4: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType5: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType6: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType7: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType8: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType9: u8,

    /// NAME: Consumption type - 消費タイプ
    /// DESC: Consumption type - 消費タイプ
    pub consumeType10: u8,

    /// NAME: MP consumption for menu supplement display - メニュー補足表示用消費MP
    /// DESC: MP consumption for menu supplement display - メニュー補足表示用消費MP
    pub consumeLoopMP_forMenu: i16,

    /// NAME: PAD12 - PAD12
    /// DESC: PAD12 - PAD12
    pub pad: [u8; 8],
}

impl Paramdef for MAGIC_PARAM_ST {
    const NAME: &'static str = "MAGIC_PARAM_ST";
    const VERSION: u16 = 6;
}
impl MAGIC_PARAM_ST {
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
    }
    /// Pledge 0 - 誓約0
    /// Bitfield2
    pub fn get_vowType0(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_vowType0(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Pledge 1 - 誓約1
    /// Bitfield2
    pub fn get_vowType1(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_vowType1(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Pledge 2 - 誓約2
    /// Bitfield2
    pub fn get_vowType2(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_vowType2(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Pledge 3 - 誓約3
    /// Bitfield2
    pub fn get_vowType3(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_vowType3(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    /// Pledge 4 - 誓約4
    /// Bitfield2
    pub fn get_vowType4(&self) -> bool {
        &self.Bitfield2 & 0x10 != 0
    }

    /// Bitfield2
    pub fn set_vowType4(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x10
        } else {
            self.Bitfield2 &= 0xEF
        }
    }
    /// Pledge 5 - 誓約5
    /// Bitfield2
    pub fn get_vowType5(&self) -> bool {
        &self.Bitfield2 & 0x20 != 0
    }

    /// Bitfield2
    pub fn set_vowType5(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x20
        } else {
            self.Bitfield2 &= 0xDF
        }
    }
    /// Pledge 6 - 誓約6
    /// Bitfield2
    pub fn get_vowType6(&self) -> bool {
        &self.Bitfield2 & 0x40 != 0
    }

    /// Bitfield2
    pub fn set_vowType6(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x40
        } else {
            self.Bitfield2 &= 0xBF
        }
    }
    /// Pledge 7 - 誓約7
    /// Bitfield2
    pub fn get_vowType7(&self) -> bool {
        &self.Bitfield2 & 0x80 != 0
    }

    /// Bitfield2
    pub fn set_vowType7(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x80
        } else {
            self.Bitfield2 &= 0x7F
        }
    }
    /// Can it be used in multi? Can be used for both single and multi - マルチでも使用できるか。シングル、マルチ両方で使える
    /// Bitfield3
    pub fn get_enable_multi(&self) -> bool {
        &self.Bitfield3 & 0x1 != 0
    }

    /// Bitfield3
    pub fn set_enable_multi(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x1
        } else {
            self.Bitfield3 &= 0xFE
        }
    }
    /// Is it only for multi? It cannot be used when it is a single. It can be used when it is multi. - マルチ専用か。シングルのときには使えない。マルチのときは使える。
    /// Bitfield3
    pub fn get_enable_multi_only(&self) -> bool {
        &self.Bitfield3 & 0x2 != 0
    }

    /// Bitfield3
    pub fn set_enable_multi_only(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x2
        } else {
            self.Bitfield3 &= 0xFD
        }
    }
    /// Is it magic to enchant? - エンチャントする魔法か
    /// Bitfield3
    pub fn get_isEnchant(&self) -> bool {
        &self.Bitfield3 & 0x4 != 0
    }

    /// Bitfield3
    pub fn set_isEnchant(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x4
        } else {
            self.Bitfield3 &= 0xFB
        }
    }
    /// Is it magic to enchant guards and shields? - ガード・盾エンチャントする魔法か
    /// Bitfield3
    pub fn get_isShieldEnchant(&self) -> bool {
        &self.Bitfield3 & 0x8 != 0
    }

    /// Bitfield3
    pub fn set_isShieldEnchant(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x8
        } else {
            self.Bitfield3 &= 0xF7
        }
    }
    /// Can surviving characters be used? - 生存キャラが使用可能か
    /// Bitfield3
    pub fn get_enable_live(&self) -> bool {
        &self.Bitfield3 & 0x10 != 0
    }

    /// Bitfield3
    pub fn set_enable_live(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x10
        } else {
            self.Bitfield3 &= 0xEF
        }
    }
    /// Can gray characters be used? - グレイキャラが使用可能か
    /// Bitfield3
    pub fn get_enable_gray(&self) -> bool {
        &self.Bitfield3 & 0x20 != 0
    }

    /// Bitfield3
    pub fn set_enable_gray(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x20
        } else {
            self.Bitfield3 &= 0xDF
        }
    }
    /// Can white ghost characters be used? - 白ゴーストキャラが使用可能か
    /// Bitfield3
    pub fn get_enable_white(&self) -> bool {
        &self.Bitfield3 & 0x40 != 0
    }

    /// Bitfield3
    pub fn set_enable_white(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x40
        } else {
            self.Bitfield3 &= 0xBF
        }
    }
    /// Can black ghost characters be used? - 黒ゴーストキャラが使用可能か
    /// Bitfield3
    pub fn get_enable_black(&self) -> bool {
        &self.Bitfield3 & 0x80 != 0
    }

    /// Bitfield3
    pub fn set_enable_black(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x80
        } else {
            self.Bitfield3 &= 0x7F
        }
    }
    /// Is it unavailable offline? - オフラインで使用不可か
    /// Bitfield4
    pub fn get_disableOffline(&self) -> bool {
        &self.Bitfield4 & 0x1 != 0
    }

    /// Bitfield4
    pub fn set_disableOffline(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x1
        } else {
            self.Bitfield4 &= 0xFE
        }
    }
    /// Do you deliver resonance magic? - 共鳴魔法配信するか
    /// Bitfield4
    pub fn get_castResonanceMagic(&self) -> bool {
        &self.Bitfield4 & 0x2 != 0
    }

    /// Bitfield4
    pub fn set_castResonanceMagic(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x2
        } else {
            self.Bitfield4 &= 0xFD
        }
    }
    /// Whether the toughness calculation is performed even if the armor SA is the initial value. Check the toughness specification .xlsx for details - 防具SAが初期値でも強靭度計算が行われるかどうか。詳細は強靭度仕様書.xlsxを確認してください
    /// Bitfield4
    pub fn get_isValidTough_ProtSADmg(&self) -> bool {
        &self.Bitfield4 & 0x4 != 0
    }

    /// Bitfield4
    pub fn set_isValidTough_ProtSADmg(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x4
        } else {
            self.Bitfield4 &= 0xFB
        }
    }
    /// Is it magic to warp? Magic with a check mark here is prohibited by the special effect "Warp Prohibition". - ワープする魔法か。ここにチェックが入っている魔法は特殊効果「ワープ禁止」により使用が禁止されます
    /// Bitfield4
    pub fn get_isWarpMagic(&self) -> bool {
        &self.Bitfield4 & 0x8 != 0
    }

    /// Bitfield4
    pub fn set_isWarpMagic(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x8
        } else {
            self.Bitfield4 &= 0xF7
        }
    }
    /// Can it be used while riding? - 騎乗中使用可能か
    /// Bitfield4
    pub fn get_enableRiding(&self) -> bool {
        &self.Bitfield4 & 0x10 != 0
    }

    /// Bitfield4
    pub fn set_enableRiding(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x10
        } else {
            self.Bitfield4 &= 0xEF
        }
    }
    /// Is it prohibited to use while not riding? - 非騎乗中使用禁止か
    /// Bitfield4
    pub fn get_disableRiding(&self) -> bool {
        &self.Bitfield4 & 0x20 != 0
    }

    /// Bitfield4
    pub fn set_disableRiding(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x20
        } else {
            self.Bitfield4 &= 0xDF
        }
    }
    /// Can it be used in an attack-prohibited area? - 攻撃禁止区域で使用できるか
    /// Bitfield4
    pub fn get_isUseNoAttackRegion(&self) -> bool {
        &self.Bitfield4 & 0x40 != 0
    }

    /// Bitfield4
    pub fn set_isUseNoAttackRegion(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x40
        } else {
            self.Bitfield4 &= 0xBF
        }
    }
    ///
    /// Bitfield4
    pub fn get_pad_1(&self) -> bool {
        &self.Bitfield4 & 0x80 != 0
    }

    /// Bitfield4
    pub fn set_pad_1(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x80
        } else {
            self.Bitfield4 &= 0x7F
        }
    }
    /// Pledge 8 - 誓約8
    /// Bitfield5
    pub fn get_vowType8(&self) -> bool {
        &self.Bitfield5 & 0x1 != 0
    }

    /// Bitfield5
    pub fn set_vowType8(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x1
        } else {
            self.Bitfield5 &= 0xFE
        }
    }
    /// Pledge 9 - 誓約9
    /// Bitfield5
    pub fn get_vowType9(&self) -> bool {
        &self.Bitfield5 & 0x2 != 0
    }

    /// Bitfield5
    pub fn set_vowType9(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x2
        } else {
            self.Bitfield5 &= 0xFD
        }
    }
    /// Pledge 10 - 誓約10
    /// Bitfield5
    pub fn get_vowType10(&self) -> bool {
        &self.Bitfield5 & 0x4 != 0
    }

    /// Bitfield5
    pub fn set_vowType10(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x4
        } else {
            self.Bitfield5 &= 0xFB
        }
    }
    /// Pledge 11 - 誓約11
    /// Bitfield5
    pub fn get_vowType11(&self) -> bool {
        &self.Bitfield5 & 0x8 != 0
    }

    /// Bitfield5
    pub fn set_vowType11(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x8
        } else {
            self.Bitfield5 &= 0xF7
        }
    }
    /// Pledge 12 - 誓約12
    /// Bitfield5
    pub fn get_vowType12(&self) -> bool {
        &self.Bitfield5 & 0x10 != 0
    }

    /// Bitfield5
    pub fn set_vowType12(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x10
        } else {
            self.Bitfield5 &= 0xEF
        }
    }
    /// Pledge 13 - 誓約13
    /// Bitfield5
    pub fn get_vowType13(&self) -> bool {
        &self.Bitfield5 & 0x20 != 0
    }

    /// Bitfield5
    pub fn set_vowType13(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x20
        } else {
            self.Bitfield5 &= 0xDF
        }
    }
    /// Pledge 14 - 誓約14
    /// Bitfield5
    pub fn get_vowType14(&self) -> bool {
        &self.Bitfield5 & 0x40 != 0
    }

    /// Bitfield5
    pub fn set_vowType14(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x40
        } else {
            self.Bitfield5 &= 0xBF
        }
    }
    /// Pledge 15 - 誓約15
    /// Bitfield5
    pub fn get_vowType15(&self) -> bool {
        &self.Bitfield5 & 0x80 != 0
    }

    /// Bitfield5
    pub fn set_vowType15(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x80
        } else {
            self.Bitfield5 &= 0x7F
        }
    }
}
