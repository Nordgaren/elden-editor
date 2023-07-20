/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MENUPROPERTY_LAYOUT {

	/// NAME: Layout path - レイアウトパス
	pub LayoutPath:[u8;16],

	/// NAME: Property ID - プロパティID
	pub PropertyID:i32,

	/// NAME: Item name Text ID - 項目名テキストID
	/// DESC: If a valid text ID is set, this will be displayed in preference to the property name. - 有効なテキストIDが設定されている場合、プロパティ名よりもこちらを優先して表示します。
	pub CaptionTextID:i32,

	/// NAME: Help text ID - ヘルプテキストID
	/// DESC: Only if this is a valid text ID will it be selectable in the field help. - ここが有効なテキストIDの場合のみ、項目ヘルプで選択できるようになります。
	pub HelpTextID:i32,

	/// NAME: reserve - 予約
	pub reserved:[u8;4],
}

impl Paramdef for MENUPROPERTY_LAYOUT {
const NAME: &'static str = "MENUPROPERTY_LAYOUT";
const VERSION: u16 = 2;
}
