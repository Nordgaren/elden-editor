/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CHARMAKEMENU_LISTITEM_PARAM_ST {
    /// NAME: value - 値
    /// DESC: The value handled by the program. Make serial numbers within one group - プログラム側に扱う値。1つのグループ内で通し番号にする
    pub value: i32,

    /// NAME: Item text ID - 項目テキストID
    /// DESC: ID of the text to be displayed - 表示するテキストのID
    pub captionId: i32,

    /// NAME: Icon ID - アイコンID
    /// DESC: ID of the icon to be displayed - 表示するアイコンのID
    pub iconId: u8,

    /// NAME: reserve - 予約
    pub reserved: [u8; 7],
}

impl Paramdef for CHARMAKEMENU_LISTITEM_PARAM_ST {
    const NAME: &'static str = "CHARMAKEMENU_LISTITEM_PARAM_ST";
    const VERSION: u16 = 2;
}
