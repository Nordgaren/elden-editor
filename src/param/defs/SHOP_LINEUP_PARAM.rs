/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SHOP_LINEUP_PARAM {
    /// NAME: Equipment ID - 装備ID
    /// DESC: ID of the equipment for sale - 販売している装備品のID
    pub equipId: i32,

    /// NAME: price - 価格
    /// DESC: Selling price to overwrite (-1: Do not overwrite) - 上書きする販売価格(-1:上書きしない)
    pub value: i32,

    /// NAME: Material ID required for purchase - 購入に必要な素材ID
    /// DESC: Material ID required for purchase (-1: None) - 購入に必要な素材ID(-1:なし)
    pub mtrlId: i32,

    /// NAME: Quantity retention event flag - 個数保持イベントフラグ
    /// DESC: Event flag value that holds the number - 個数を保持してあるイベントフラグ値
    pub eventFlag_forStock: u32,

    /// NAME: Sales ban event flag - 販売解禁イベントフラグ
    /// DESC: Sales ban event flag - 販売解禁イベントフラグ
    pub eventFlag_forRelease: u32,

    /// NAME: Number of units sold - 販売個数
    /// DESC: Number of units sold - 販売個数
    pub sellQuantity: i16,

    /// NAME: Padding - パディング
    pub pad3: [u8; 1],

    /// NAME: Equipment type - 装備タイプ
    /// DESC: Types of equipment for sale - 販売している装備品の種類
    pub equipType: u8,

    /// NAME: Price type - 価格タイプ
    /// DESC: Price type. Applies only when overwriting the selling price - 価格の種類。販売価格を上書きするときだけ適用される
    pub costType: u8,

    /// NAME: Padding - パディング
    pub pad1: [u8; 1],

    /// NAME: Number of sets sold - 販売セット数
    /// DESC: Number of sets sold. Number of pieces you can get with one purchase (default: 1) - 販売セット数。1回の購入で手に入る個数（デフォルト: 1）
    pub setNum: u16,

    /// NAME: Addition - 加算
    /// DESC: Correction (addition) to the selling price of equipment. Equipment para selling price x magnification + addition - 装備品の販売価格に対する補正（加算）。装備品パラの販売価格×倍率＋加算
    pub value_Add: i32,

    /// NAME: magnification - 倍率
    /// DESC: Correction (magnification) to the selling price of equipment. Equipment para selling price x magnification + addition - 装備品の販売価格に対する補正（倍率）。装備品パラの販売価格×倍率＋加算
    pub value_Magnification: f32,

    /// NAME: Icon ID - アイコンID
    /// DESC: Menu display_icon ID (-1: do not overwrite) - メニュー表示用_アイコンID(-1:上書きしない)
    pub iconId: i32,

    /// NAME: Text ID - テキストID
    /// DESC: Menu display_text ID (-1: do not overwrite) - メニュー表示用_テキストID(-1:上書きしない)
    pub nameMsgId: i32,

    /// NAME: Menu title text ID - メニュータイトルテキストID
    /// DESC: Text ID of the shop menu title (-1: Do not overwrite). This value of the first para found in the para ID range passed when launching the shop is referenced - ショップのメニュータイトルのテキストID(-1:上書きしない)。ショップを起動するときに渡されたパラID範囲の中で最初に見つかったパラのこの値が参照されます
    pub menuTitleMsgId: i32,

    /// NAME: Menu icon ID - メニューアイコンID
    /// DESC: Shop menu icon ID (-1: Do not overwrite). This value of the first para found in the para ID range passed when launching the shop is referenced - ショップのメニューアイコンID(-1:上書きしない)。ショップを起動するときに渡されたパラID範囲の中で最初に見つかったパラのこの値が参照されます
    pub menuIconId: i16,

    /// NAME: Padding - パディング
    pub pad2: [u8; 2],
}

impl Paramdef for SHOP_LINEUP_PARAM {
    const NAME: &'static str = "SHOP_LINEUP_PARAM";
    const VERSION: u16 = 3;
}
