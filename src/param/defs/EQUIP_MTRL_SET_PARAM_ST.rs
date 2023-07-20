/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_MTRL_SET_PARAM_ST {
    /// NAME: Required Material Item ID 01 - 必要素材アイテムID01
    /// DESC: Material item ID required to strengthen armor. - 武具強化に必要な素材アイテムIDです。
    pub materialId01: i32,

    /// NAME: Required Material Item ID 02 - 必要素材アイテムID02
    /// DESC: Material item ID required to strengthen armor. - 武具強化に必要な素材アイテムIDです。
    pub materialId02: i32,

    /// NAME: Required Material Item ID 03 - 必要素材アイテムID03
    /// DESC: Material item ID required to strengthen armor. - 武具強化に必要な素材アイテムIDです。
    pub materialId03: i32,

    /// NAME: Required Material Item ID 04 - 必要素材アイテムID04
    /// DESC: Material item ID required to strengthen armor. - 武具強化に必要な素材アイテムIDです。
    pub materialId04: i32,

    /// NAME: Required Material Item ID 05 - 必要素材アイテムID05
    /// DESC: Material item ID required to strengthen armor. - 武具強化に必要な素材アイテムIDです。
    pub materialId05: i32,

    /// NAME: Required Material Item ID 06 - 必要素材アイテムID06
    /// DESC: Material item ID required to strengthen armor. - 武具強化に必要な素材アイテムIDです。
    pub materialId06: i32,

    /// NAME: Padding - パディング
    /// DESC: Padding. For when the material item ID increases - パディング。素材アイテムIDが増えたとき用
    pub pad_id: [u8; 8],

    /// NAME: Required number 01 - 必要個数01
    /// DESC: The number of material items required to strengthen armor. - 武具強化に必要な素材アイテムの個数です。
    pub itemNum01: i8,

    /// NAME: Required number 02 - 必要個数02
    /// DESC: The number of material items required to strengthen armor. - 武具強化に必要な素材アイテムの個数です。
    pub itemNum02: i8,

    /// NAME: Required number 03 - 必要個数03
    /// DESC: The number of material items required to strengthen armor. - 武具強化に必要な素材アイテムの個数です。
    pub itemNum03: i8,

    /// NAME: Required number 04 - 必要個数04
    /// DESC: The number of material items required to strengthen armor. - 武具強化に必要な素材アイテムの個数です。
    pub itemNum04: i8,

    /// NAME: Required number 05 - 必要個数05
    /// DESC: The number of material items required to strengthen armor. - 武具強化に必要な素材アイテムの個数です。
    pub itemNum05: i8,

    /// NAME: Required number 06 - 必要個数06
    /// DESC: The number of material items required to strengthen armor. - 武具強化に必要な素材アイテムの個数です。
    pub itemNum06: i8,

    /// NAME: Padding - パディング
    /// DESC: Padding. For when the number of items increases - パディング。アイテムの個数が増えたとき用
    pub pad_num: [u8; 2],

    /// NAME: Required Material Item Category 01 - 必要素材アイテムカテゴリ01
    /// DESC: This is a category of material items required for strengthening armor. - 武具強化に必要な素材アイテムのカテゴリです。
    pub materialCate01: u8,

    /// NAME: Required Material Item Category 02 - 必要素材アイテムカテゴリ02
    /// DESC: This is a category of material items required for strengthening armor. - 武具強化に必要な素材アイテムのカテゴリです。
    pub materialCate02: u8,

    /// NAME: Required Material Item Category 03 - 必要素材アイテムカテゴリ03
    /// DESC: This is a category of material items required for strengthening armor. - 武具強化に必要な素材アイテムのカテゴリです。
    pub materialCate03: u8,

    /// NAME: Required Material Item Category 04 - 必要素材アイテムカテゴリ04
    /// DESC: This is a category of material items required for strengthening armor. - 武具強化に必要な素材アイテムのカテゴリです。
    pub materialCate04: u8,

    /// NAME: Required Material Item Category 05 - 必要素材アイテムカテゴリ05
    /// DESC: This is a category of material items required for strengthening armor. - 武具強化に必要な素材アイテムのカテゴリです。
    pub materialCate05: u8,

    /// NAME: Required Material Item Category 06 - 必要素材アイテムカテゴリ06
    /// DESC: This is a category of material items required for strengthening armor. - 武具強化に必要な素材アイテムのカテゴリです。
    pub materialCate06: u8,

    /// NAME: Padding - パディング
    /// DESC: Padding. For when the number of categories increases - パディング。カテゴリが増えたとき用
    pub pad_cate: [u8; 2],

    /// NAME: Disable number display 01 - 個数表示を無効化01
    /// DESC: Disable the number display (for enhanced shops) - 個数表示を無効化するか(強化ショップ用)
    pub Bitfield1: u8,

    /// NAME: Padding - パディング
    /// DESC: It's padding. - パディングです。
    pub pad: [u8; 3],
}

impl Paramdef for EQUIP_MTRL_SET_PARAM_ST {
    const NAME: &'static str = "EQUIP_MTRL_SET_PARAM_ST";
    const VERSION: u16 = 2;
}
impl EQUIP_MTRL_SET_PARAM_ST {
    /// Disable the number display (for enhanced shops) - 個数表示を無効化するか(強化ショップ用)
    /// Bitfield1
    pub fn get_isDisableDispNum01(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_isDisableDispNum01(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// Whether to disable the number display - 個数表示を無効化するか
    /// Bitfield1
    pub fn get_isDisableDispNum02(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_isDisableDispNum02(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Whether to disable the number display - 個数表示を無効化するか
    /// Bitfield1
    pub fn get_isDisableDispNum03(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_isDisableDispNum03(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// Whether to disable the number display - 個数表示を無効化するか
    /// Bitfield1
    pub fn get_isDisableDispNum04(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_isDisableDispNum04(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
    /// Whether to disable the number display - 個数表示を無効化するか
    /// Bitfield1
    pub fn get_isDisableDispNum05(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_isDisableDispNum05(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xEF
        }
    }
    /// Whether to disable the number display - 個数表示を無効化するか
    /// Bitfield1
    pub fn get_isDisableDispNum06(&self) -> bool {
        &self.Bitfield1 & 0x20 != 0
    }

    /// Bitfield1
    pub fn set_isDisableDispNum06(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x20
        } else {
            self.Bitfield1 &= 0xDF
        }
    }
}
