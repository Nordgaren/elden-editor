/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BONFIRE_WARP_TAB_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Text ID - テキストID
    /// DESC: Tab Display Name Text ID [MenuText] - タブの表示名テキストID[MenuText]
    pub textId: i32,

    /// NAME: Sort ID - ソートID
    /// DESC: Tab display order sort ID. Line up from the left with aim - タブの表示順ソートID。照準で左から並ぶ
    pub sortId: i32,

    /// NAME: Icon ID - アイコンID
    /// DESC: Tab icon ID. Menu resource compliance - タブのアイコンID。メニューリソース準拠
    pub iconId: u16,

    /// NAME: pad - パッド
    pub pad: [u8; 2],
}

impl Paramdef for BONFIRE_WARP_TAB_PARAM_ST {
    const NAME: &'static str = "BONFIRE_WARP_TAB_PARAM_ST";
    const VERSION: u16 = 2;
}
impl BONFIRE_WARP_TAB_PARAM_ST {
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
