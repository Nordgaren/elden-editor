/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WORLD_MAP_PLACE_NAME_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Map fragment parameter ID - 地図断片パラメータID
    /// DESC: Map fragment parameter ID. Display text if you have this map fragment - 地図断片パラメータID。この地図断片を持っていればテキストを表示する
    pub worldMapPieceId: i32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. PlaceName.xlsm - 表示するテキストID。PlaceName.xlsm
    pub textId: i32,

    /// NAME: Padding - パディング
    pub pad1: [u8; 4],

    /// NAME: Area number - エリア番号
    /// DESC: AA part of mAA_BB_CC_DD - mAA_BB_CC_DD の AA 部分
    pub areaNo: u8,

    /// NAME: Grid X number - グリッドX番号
    /// DESC: BB part of mAA_BB_CC_DD - mAA_BB_CC_DD の BB 部分
    pub gridXNo: u8,

    /// NAME: Grid Z number - グリッドZ番号
    /// DESC: CC part of mAA_BB_CC_DD - mAA_BB_CC_DD の CC 部分
    pub gridZNo: u8,

    /// NAME: Padding - パディング
    /// DESC: Padding - パディング
    pub pad2: [u8; 1],

    /// NAME: X coordinate - X座標
    /// DESC: X coordinate - X座標
    pub posX: f32,

    /// NAME: Y coordinate - Y座標
    /// DESC: Y coordinate (not used) - Y座標（使っていない）
    pub posY: f32,

    /// NAME: Z coordinate - Z座標
    /// DESC: Z coordinate - Z座標
    pub posZ: f32,
}

impl Paramdef for WORLD_MAP_PLACE_NAME_PARAM_ST {
    const NAME: &'static str = "WORLD_MAP_PLACE_NAME_PARAM_ST";
    const VERSION: u16 = 2;
}
impl WORLD_MAP_PLACE_NAME_PARAM_ST {
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
}
