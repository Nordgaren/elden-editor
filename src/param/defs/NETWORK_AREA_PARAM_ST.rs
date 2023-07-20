/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NETWORK_AREA_PARAM_ST {
    /// NAME: Cell size X - セルサイズX
    /// DESC: Cell size X - セルサイズX
    pub cellSizeX: f32,

    /// NAME: Cell size Y - セルサイズY
    /// DESC: Cell size Y - セルサイズY
    pub cellSizeY: f32,

    /// NAME: Cell size Z - セルサイズZ
    /// DESC: Cell size Z - セルサイズZ
    pub cellSizeZ: f32,

    /// NAME: Cell offset X - セルオフセットX
    /// DESC: Cell offset X - セルオフセットX
    pub cellOffsetX: f32,

    /// NAME: Cell offset Y - セルオフセットY
    /// DESC: Cell offset Y - セルオフセットY
    pub cellOffsetY: f32,

    /// NAME: Cell offset Z - セルオフセットZ
    /// DESC: Cell offset Z - セルオフセットZ
    pub cellOffsetZ: f32,

    /// NAME: Effective bloodstain / death illusion - 血痕・死亡幻影有効
    /// DESC: Effective bloodstain / death illusion - 血痕・死亡幻影有効
    pub Bitfield1: u8,

    /// NAME: dummy - ダミー
    pub dummy: [u8; 3],
}

impl Paramdef for NETWORK_AREA_PARAM_ST {
    const NAME: &'static str = "NETWORK_AREA_PARAM_ST";
    const VERSION: u16 = 4;
}
impl NETWORK_AREA_PARAM_ST {
    /// Effective bloodstain / death illusion - 血痕・死亡幻影有効
    /// Bitfield1
    pub fn get_enableBloodstain(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_enableBloodstain(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// Blood character valid - 血文字有効
    /// Bitfield1
    pub fn get_enableBloodMessage(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_enableBloodMessage(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Phantom effective - 幻影有効
    /// Bitfield1
    pub fn get_enableGhost(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_enableGhost(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// Multiplayer enabled - マルチプレイ有効
    /// Bitfield1
    pub fn get_enableMultiPlay(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_enableMultiPlay(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
    /// Is it a search target for ring search? (Area called Kanemori Ash Spirit / Relief Blue Spirit) - 指輪検索の検索対象か？（鐘守灰霊・救援青霊として呼ばれるエリア）
    /// Bitfield1
    pub fn get_enableRingSearch(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_enableRingSearch(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xEF
        }
    }
    /// Is it the target of intrusion search? - 乱入検索の対象か？
    /// Bitfield1
    pub fn get_enableBreakInSearch(&self) -> bool {
        &self.Bitfield1 & 0x20 != 0
    }

    /// Bitfield1
    pub fn set_enableBreakInSearch(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x20
        } else {
            self.Bitfield1 &= 0xDF
        }
    }
}
