/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_PARAM_ACCESSORY_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Call ID - 呼び出しID
    /// DESC: ID called from decorations - 装飾品から呼び出すID
    pub refId: i32,

    /// NAME: SFX variation ID - SFXバリエーションID
    /// DESC: Specify SFX variation (used to identify SFX in combination with TimeActEditor ID) - ＳＦＸのバリエーションを指定（TimeActEditorのＩＤと組み合わせて、ＳＦＸを特定するのに使用する）
    pub sfxVariationId: i32,

    /// NAME: Weight [kg] - 重量[kg]
    /// DESC: Weight [kg] - 重量[kg]
    pub weight: f32,

    /// NAME: Action ID - 行動ID
    /// DESC: Action ID (= Skill) - 行動ID(=Skill)
    pub behaviorId: i32,

    /// NAME: Basic price - 基本価格
    /// DESC: Basic price - 基本価格
    pub basicPrice: i32,

    /// NAME: Sale price - 売却価格
    /// DESC: Selling price - 販売価格
    pub sellValue: i32,

    /// NAME: sortID - sortID
    pub sortId: i32,

    /// NAME: QWCID - QWCID
    pub qwcId: i32,

    /// NAME: Equipment model number - 装備モデル番号
    /// DESC: Equipment model number - 装備モデルの番号
    pub equipModelId: u16,

    /// NAME: Icon ID - アイコンID
    /// DESC: Menu icon ID - メニューアイコンID
    pub iconId: u16,

    /// NAME: Shop level - ショップレベル
    /// DESC: Level that can be sold at the store - お店で販売できるレベル
    pub shopLv: i16,

    /// NAME: Trophy - トロフィー
    pub trophySGradeId: i16,

    /// NAME: Trophy SEQ number - トロフィーSEQ番号
    /// DESC: Trophy SEQ number - トロフィーのSEQ番号
    pub trophySeqId: i16,

    /// NAME: Equipment model type - 装備モデル種別
    /// DESC: Equipment model type - 装備モデルの種別
    pub equipModelCategory: u8,

    /// NAME: Equipment model gender - 装備モデル性別
    /// DESC: Gender of equipment model - 装備モデルの性別
    pub equipModelGender: u8,

    /// NAME: Decoration category - 装飾カテゴリ
    /// DESC: Armor category - 防具のカテゴリ
    pub accessoryCategory: u8,

    /// NAME: ID category - IDカテゴリ
    /// DESC: ↓ ID category [Attack, Projectile, Special] - ↓のIDのカテゴリ[攻撃、飛び道具、特殊]
    pub refCategory: u8,

    /// NAME: Special effects category - 特殊効果カテゴリ
    /// DESC: Since there are effects (enchantment weapons, etc.) whose parameters fluctuate depending on skills, magic, items, etc., set each action so that the determined effect can correspond to the effect such as "power up only weapon attack". Set "None" for items that do not need to be set, such as varistor. - スキルや、魔法、アイテムなどで、パラメータが変動する効果（エンチャントウェポンなど）があるので、│定した効果が、「武器攻撃のみをパワーアップする」といった効果に対応できるように行動ごとに設定するバリスタなど、設定の必要のないものは「なし」を設定する
    pub spEffectCategory: u8,

    /// NAME: Sort item type ID - ソートアイテム種別ID
    /// DESC: Sort item type ID. In the sort "Item type order", the same ID is displayed together as the same group. - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
    pub sortGroupId: u8,

    /// NAME: Item lottery ID_for map at the time of vagrant - ベイグラント時アイテム抽選ID_マップ用
    /// DESC: -1: No vagrant 0: No lottery 1 ~: With lottery - -1：ベイグラントなし 0：抽選なし 1～：抽選あり
    pub vagrantItemLotId: i32,

    /// NAME: Vagrant Bonus Enemy Drop Item Lottery ID_Map - ベイグラントボーナス敵ドロップアイテム抽選ID_マップ用
    /// DESC: -1: No drop 0: No lottery 1 ~: With lottery - -1：ドロップなし 0：抽選なし 1～：抽選あり
    pub vagrantBonusEneDropItemLotId: i32,

    /// NAME: Vagrant item Enemy drop item Lottery ID_for map - ベイグラントアイテム敵ドロップアイテム抽選ID_マップ用
    /// DESC: -1: No drop 0: No lottery 1 ~: With lottery - -1：ドロップなし 0：抽選なし 1～：抽選あり
    pub vagrantItemEneDropItemLotId: i32,

    /// NAME: Can i deposit - 預けれるか
    /// DESC: Can you deposit it in the warehouse? - 倉庫へ預けれるか
    pub Bitfield2: u8,

    /// NAME: Rarity - レア度
    /// DESC: Rarity used in item acquisition logs - アイテム取得ログで使うレア度
    pub rarity: u8,

    /// NAME: pad - pad
    /// DESC: (Old log icon ID) - （旧ログ用アイコンID）
    pub pad2: [u8; 2],

    /// NAME: Selling price - 販売価格
    /// DESC: Selling price - 販売価格
    pub saleValue: i32,

    /// NAME: Wearing group ID - 装着グループID
    /// DESC: Items of the same group cannot be equipped at the same time - 同じグループの物は同時装備不可能
    pub accessoryGroup: i16,

    /// NAME: pad - pad
    /// DESC: pad - pad
    pub pad3: [u8; 1],

    /// NAME: Comptrophy SEQ number - コンプトロフィーSEQ番号
    /// DESC: SEQ number of complete trophy - コンプリート系トロフィのSEQ番号
    pub compTrophySedId: i8,

    /// NAME: Resident special effect ID1 - 常駐特殊効果ID1
    /// DESC: Resident special effect ID1 - 常駐特殊効果ID1
    pub residentSpEffectId1: i32,

    /// NAME: Resident special effect ID2 - 常駐特殊効果ID2
    /// DESC: Resident special effect ID2 - 常駐特殊効果ID2
    pub residentSpEffectId2: i32,

    /// NAME: Resident special effect ID3 - 常駐特殊効果ID3
    /// DESC: Resident special effect ID3 - 常駐特殊効果ID3
    pub residentSpEffectId3: i32,

    /// NAME: Resident special effect ID4 - 常駐特殊効果ID4
    /// DESC: Resident special effect ID4 - 常駐特殊効果ID4
    pub residentSpEffectId4: i32,

    /// NAME: pad - pad
    /// DESC: pad - pad
    pub pad1: [u8; 4],
}

impl Paramdef for EQUIP_PARAM_ACCESSORY_ST {
    const NAME: &'static str = "EQUIP_PARAM_ACCESSORY_ST";
    const VERSION: u16 = 2;
}
impl EQUIP_PARAM_ACCESSORY_ST {
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
    /// Can you deposit it in the warehouse? - 倉庫へ預けれるか
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
    /// Will it break when equipped and removed? - 装備して外す時に壊れるか
    /// Bitfield2
    pub fn get_isEquipOutBrake(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_isEquipOutBrake(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
    /// Bitfield2
    pub fn get_disableMultiDropShare(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_disableMultiDropShare(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
    /// Bitfield2
    pub fn get_isDiscard(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_isDiscard(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    /// Can I put the item on the spot? TRUE = can be placed - アイテムをその場に置けるか？TRUE=置ける
    /// Bitfield2
    pub fn get_isDrop(&self) -> bool {
        &self.Bitfield2 & 0x10 != 0
    }

    /// Bitfield2
    pub fn set_isDrop(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x10
        } else {
            self.Bitfield2 &= 0xEF
        }
    }
    /// Whether to display in the item acquisition log when acquiring the item (not entered: ○) - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
    /// Bitfield2
    pub fn get_showLogCondType(&self) -> bool {
        &self.Bitfield2 & 0x20 != 0
    }

    /// Bitfield2
    pub fn set_showLogCondType(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x20
        } else {
            self.Bitfield2 &= 0xDF
        }
    }
    /// Whether to display it in the item acquisition dialog when acquiring an item (not entered: new only) - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
    /// Bitfield2
    pub fn get_showDialogCondType(&self) -> u8 {
        &self.Bitfield2 & 0xC0
    }

    /// Bitfield2 MAX: 3
    pub fn set_showDialogCondType(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 6) & 0xC0;
            let newVal = &self.Bitfield2 & 0x3F | val;
            self.Bitfield2 = newVal
        } else {
            self.Bitfield2 &= 0x3F
        }
    }
}
