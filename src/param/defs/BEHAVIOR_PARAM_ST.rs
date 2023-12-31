/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BEHAVIOR_PARAM_ST {
    /// NAME: Behavior variation ID - 行動バリエーションID
    /// DESC: Used when calculating the ID for attack parameters. It is not used directly on the actual machine. - 攻撃パラメータ用のIDを算出する際に使用します。実機上では直接使用しません。
    pub variationId: i32,

    /// NAME: Action judgment ID - 行動判定ID
    /// DESC: Used when calculating the ID for attack parameters. This ID matches the action judgment ID entered in TimeActEditor. It is not used directly on the actual machine. - 攻撃パラメータ用のIDを算出する際に使用します。このIDはTimeActEditorで入力される行動判定IDと一致させます。実機上では直接使用しません。
    pub behaviorJudgeId: i32,

    /// NAME: For ID rules - IDルール用
    /// DESC: For ID calculation rules - ID算出ルール用
    pub ezStateBehaviorType_old: u8,

    /// NAME: Reference ID type - 参照IDタイプ
    /// DESC: Specify the reference ID so that it is correct. - 参照IDを間違わないように指定.
    pub refType: u8,

    /// NAME: pad - pad
    pub pad2: [u8; 2],

    /// NAME: Reference ID - 参照ID
    /// DESC: It can be used properly according to the attack power, missile, ID of special effect parameter, and refType. - 攻撃力、飛び道具、特殊効果パラメータのID、refTypeによって使い分けられる。
    pub refId: i32,

    /// NAME: Consumption SA - 消費SA
    /// DESC: Set the amount of SA consumed during action. - 行動時の消費SA量を設定.
    pub consumeSA: f32,

    /// NAME: Consumption stamina - 消費スタミナ
    /// DESC: Set the amount of stamina consumed during action. - 行動時の消費スタミナ量を設定.
    pub stamina: i32,

    /// NAME: Weapon durability consumption (only when using missiles) - 武器耐久度消費（飛び道具時のみ）
    /// DESC: Set the durability of weapons consumed during action. - 行動時の消費武器耐久度を設定.
    pub consumeDurability: i32,

    /// NAME: category - カテゴリ
    /// DESC: Since there are effects (enchantment weapons, etc.) whose parameters fluctuate depending on skills, magic, items, etc., set each action so that the determined effect can correspond to the effect such as "power up only weapon attack". Set "None" for items that do not need to be set, such as varistor. - スキルや、魔法、アイテムなどで、パラメータが変動する効果（エンチャントウェポンなど）があるので、│定した効果が、「武器攻撃のみをパワーアップする」といった効果に対応できるように行動ごとに設定するバリスタなど、設定の必要のないものは「なし」を設定する
    pub category: u8,

    /// NAME: Consumer nature - 消費人間性
    /// DESC: Set the amount of humanity consumed during action - 行動時の消費人間性量を設定
    pub heroPoint: u8,

    /// NAME: pad - pad
    pub pad1: [u8; 2],
}

impl Paramdef for BEHAVIOR_PARAM_ST {
    const NAME: &'static str = "BEHAVIOR_PARAM_ST";
    const VERSION: u16 = 2;
}
