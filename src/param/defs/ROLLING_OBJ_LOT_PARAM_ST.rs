/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ROLLING_OBJ_LOT_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: AssetId_0 - AssetId_0
    /// DESC: AssetId_0 (-1: Ignore) - AssetId_0 (-1：無視)
    pub AssetId_0: i32,

    /// NAME: AssetId_1_1 - AssetId_1
    /// DESC: AssetId_1 (-1: Ignore) - AssetId_1 (-1：無視)
    pub AssetId_1: i32,

    /// NAME: AssetId_2 - AssetId_2
    /// DESC: AssetId_2 (-1: Ignore) - AssetId_2 (-1：無視)
    pub AssetId_2: i32,

    /// NAME: AssetId_3 - AssetId_3
    /// DESC: AssetId_3 (-1: Ignore) - AssetId_3 (-1：無視)
    pub AssetId_3: i32,

    /// NAME: AssetId_4 - AssetId_4
    /// DESC: AssetId_4 (-1: Ignore) - AssetId_4 (-1：無視)
    pub AssetId_4: i32,

    /// NAME: AssetId_5 - AssetId_5
    /// DESC: AssetId_5 (-1: Ignore) - AssetId_5 (-1：無視)
    pub AssetId_5: i32,

    /// NAME: AssetId_6 - AssetId_6
    /// DESC: AssetId_6 (-1: Ignore) - AssetId_6 (-1：無視)
    pub AssetId_6: i32,

    /// NAME: AssetId_7 - AssetId_7
    /// DESC: AssetId_7 (-1: Ignore) - AssetId_7 (-1：無視)
    pub AssetId_7: i32,

    /// NAME: Appearance weight _0 - 出現ウェイト_0
    /// DESC: Appearance ratio point (weight) _0: 0 is ignored - 出現の比率ポイント(ウェイト)_0: 0だと無視
    pub CreateWeight_0: u8,

    /// NAME: Appearance weight _1 - 出現ウェイト_1
    /// DESC: Appearance ratio point (weight) _1 - 出現の比率ポイント(ウェイト)_1
    pub CreateWeight_1: u8,

    /// NAME: Appearance weight _2 - 出現ウェイト_2
    /// DESC: Appearance ratio point (weight) _2 - 出現の比率ポイント(ウェイト)_2
    pub CreateWeight_2: u8,

    /// NAME: Appearance weight _3 - 出現ウェイト_3
    /// DESC: Appearance ratio point (weight) _3 - 出現の比率ポイント(ウェイト)_3
    pub CreateWeight_3: u8,

    /// NAME: Appearance weight _4 - 出現ウェイト_4
    /// DESC: Appearance ratio point (weight) _4 - 出現の比率ポイント(ウェイト)_4
    pub CreateWeight_4: u8,

    /// NAME: Appearance weight _5 - 出現ウェイト_5
    /// DESC: Appearance ratio point (weight) _5 - 出現の比率ポイント(ウェイト)_5
    pub CreateWeight_5: u8,

    /// NAME: Appearance weight _6 - 出現ウェイト_6
    /// DESC: Appearance ratio point (weight) _6 - 出現の比率ポイント(ウェイト)_6
    pub CreateWeight_6: u8,

    /// NAME: Appearance weight _7 - 出現ウェイト_7
    /// DESC: Appearance ratio point (weight) _7 - 出現の比率ポイント(ウェイト)_7
    pub CreateWeight_7: u8,

    /// NAME: Reserve - リザーブ
    /// DESC: Reserve - リザーブ
    pub Reserve_0: [u8; 20],
}

impl Paramdef for ROLLING_OBJ_LOT_PARAM_ST {
    const NAME: &'static str = "ROLLING_OBJ_LOT_PARAM_ST";
    const VERSION: u16 = 2;
}
impl ROLLING_OBJ_LOT_PARAM_ST {
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
