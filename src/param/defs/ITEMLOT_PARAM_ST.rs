/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ITEMLOT_PARAM_ST {
    /// NAME: 1: Item ID - １：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId01: i32,

    /// NAME: 2: Item ID - ２：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId02: i32,

    /// NAME: 3: Item ID - ３：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId03: i32,

    /// NAME: 4: Item ID - ４：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId04: i32,

    /// NAME: 5: Item ID - ５：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId05: i32,

    /// NAME: 6: Item ID - ６：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId06: i32,

    /// NAME: 7: Item ID - ７：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId07: i32,

    /// NAME: 8: Item ID - ８：アイテムID
    /// DESC: Item ID that can be obtained - 取得できるアイテムのID
    pub lotItemId08: i32,

    /// NAME: 1: Item category - １：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory01: i32,

    /// NAME: 2: Item category - ２：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory02: i32,

    /// NAME: 3: Item category - ３：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory03: i32,

    /// NAME: 4: Item category - ４：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory04: i32,

    /// NAME: 5: Item category - ５：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory05: i32,

    /// NAME: 6: Item category - ６：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory06: i32,

    /// NAME: 7: Item category - ７：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory07: i32,

    /// NAME: 8: Item category - ８：アイテムカテゴリ
    /// DESC: Category of items that can be obtained - 取得できるアイテムのカテゴリ
    pub lotItemCategory08: i32,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint01: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint02: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint03: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint04: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint05: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint06: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint07: u16,

    /// NAME: Basic appearance point - 基本出現ポイント
    /// DESC: Appearance point at normal time - 通常時の出現ポイント
    pub lotItemBasePoint08: u16,

    /// NAME: Appearance points after accumulation - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint01: u16,

    /// NAME: Appearance points after accumulation - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint02: u16,

    /// NAME: Cumulative post-appearance points - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint03: u16,

    /// NAME: Appearance points after accumulation - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint04: u16,

    /// NAME: Appearance points after accumulation - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint05: u16,

    /// NAME: Appearance points after accumulation - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint06: u16,

    /// NAME: Cumulative post-appearance points - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint07: u16,

    /// NAME: Cumulative post-appearance points - 累積後出現ポイント
    /// DESC: Appearance point at maximum cumulative - 最大累積時の出現ポイント
    pub cumulateLotPoint08: u16,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId01: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId02: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId03: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId04: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId05: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId06: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId07: u32,

    /// NAME: Another crunchy flag ID - 別ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: common use) - 取得済みフラグとザクザク枠兼用(0:共通使用)
    pub getItemFlagId08: u32,

    /// NAME: Crunchy flag ID - ザクザクフラグID
    /// DESC: Combined use of acquired flag and crunchy frame (0: flag invalid) - 取得済みフラグとザクザク枠兼用(0:フラグ無効)
    pub getItemFlagId: u32,

    /// NAME: Lottery cumulative save flag ID - 抽選累積保存フラグID
    /// DESC: For saving the number of lottery (* 8 flag serial number used) - 抽選回数保存用(※8フラグ連番使用)
    pub cumulateNumFlagId: u32,

    /// NAME: Maximum number of lottery cumulative - 抽選累積最大数
    /// DESC: Maximum number of lottery cumulative (0: no cumulative) - 抽選累積最大数(0:累積なし)
    pub cumulateNumMax: u8,

    /// NAME: Rarity overwrite - レア度上書き
    /// DESC: Specify how valuable items are in the treasure chest. When -1, use the rarity of the equipment para without overwriting - 宝箱などに、どれくらい貴重なアイテムが入っているかを指定する。-1の時は上書きせず装備品パラのレア度を使用する
    pub lotItem_Rarity: i8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum01: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum02: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum03: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum04: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum05: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum06: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum07: u8,

    /// NAME: Quantity - 個数
    /// DESC: Number of items that can be acquired - 取得できるアイテムの個数
    pub lotItemNum08: u8,

    /// NAME: Luck parameter valid - 運パラメータ有効
    /// DESC: Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    pub Bitfield1: u16,

    /// NAME: Offset after X week - X週目以降オフセット
    /// DESC: Offset during lap play - 周回プレイ時のオフセット
    pub GameClearOffset: i8,

    /// NAME: Do you draw lots even with cooperating spirits? - 協力霊でも抽選するか
    /// DESC: Do you draw lots even when you are a cooperating spirit? - 自身が協力霊の時でも抽選するか
    pub Bitfield2: u8,

    /// NAME: PAD2 - PAD2
    /// DESC: PAD2 - PAD2
    pub PAD2: u16,
}

impl Paramdef for ITEMLOT_PARAM_ST {
    const NAME: &'static str = "ITEMLOT_PARAM_ST";
    const VERSION: u16 = 4;
}
impl ITEMLOT_PARAM_ST {
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck01(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck01(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFFFE
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck02(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck02(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFFFD
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck03(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck03(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFFFB
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck04(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck04(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xFFF7
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck05(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck05(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xFFEF
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck06(&self) -> bool {
        &self.Bitfield1 & 0x20 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck06(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x20
        } else {
            self.Bitfield1 &= 0xFFDF
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck07(&self) -> bool {
        &self.Bitfield1 & 0x40 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck07(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x40
        } else {
            self.Bitfield1 &= 0xFFBF
        }
    }
    /// Whether the probability of lottery reflects the player's luck - 抽選の確率をプレイヤーの運を反映させるか
    /// Bitfield1
    pub fn get_enableLuck08(&self) -> bool {
        &self.Bitfield1 & 0x80 != 0
    }

    /// Bitfield1
    pub fn set_enableLuck08(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x80
        } else {
            self.Bitfield1 &= 0xFF7F
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset01(&self) -> bool {
        &self.Bitfield1 & 0x100 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset01(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x100
        } else {
            self.Bitfield1 &= 0xFEFF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset02(&self) -> bool {
        &self.Bitfield1 & 0x200 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset02(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x200
        } else {
            self.Bitfield1 &= 0xFDFF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset03(&self) -> bool {
        &self.Bitfield1 & 0x400 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset03(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x400
        } else {
            self.Bitfield1 &= 0xFBFF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset04(&self) -> bool {
        &self.Bitfield1 & 0x800 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset04(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x800
        } else {
            self.Bitfield1 &= 0xF7FF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset05(&self) -> bool {
        &self.Bitfield1 & 0x1000 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset05(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1000
        } else {
            self.Bitfield1 &= 0xEFFF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset06(&self) -> bool {
        &self.Bitfield1 & 0x2000 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset06(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2000
        } else {
            self.Bitfield1 &= 0xDFFF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset07(&self) -> bool {
        &self.Bitfield1 & 0x4000 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset07(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4000
        } else {
            self.Bitfield1 &= 0xBFFF
        }
    }
    /// Whether to reset cumulatively - 累積リセットするか
    /// Bitfield1
    pub fn get_cumulateReset08(&self) -> bool {
        &self.Bitfield1 & 0x8000 != 0
    }

    /// Bitfield1
    pub fn set_cumulateReset08(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8000
        } else {
            self.Bitfield1 &= 0x7FFF
        }
    }
    /// Do you draw lots even when you are a cooperating spirit? - 自身が協力霊の時でも抽選するか
    /// Bitfield2
    pub fn get_canExecByFriendlyGhost(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_canExecByFriendlyGhost(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Do you draw lots even when you are a hostile spirit? - 自身が敵対霊の時でも抽選するか
    /// Bitfield2
    pub fn get_canExecByHostileGhost(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_canExecByHostileGhost(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// PAD1 - PAD1
    /// Bitfield2
    pub fn get_PAD1(&self) -> u8 {
        &self.Bitfield2 & 0xFC
    }

    /// Bitfield2 MAX: 63
    pub fn set_PAD1(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 2) & 0xFC;
            let newVal = &self.Bitfield2 & 0x3 | val;
            self.Bitfield2 = newVal
        } else {
            self.Bitfield2 &= 0x3
        }
    }
}
