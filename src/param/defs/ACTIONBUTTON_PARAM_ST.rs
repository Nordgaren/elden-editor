/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ACTIONBUTTON_PARAM_ST {
    /// NAME: Range type - 範囲タイプ
    /// DESC: Range shape (cylinder, prism, capsule) - 範囲形状(円柱、角柱、カプセル)
    pub regionType: u8,

    /// NAME: category - カテゴリ
    /// DESC: category. The number on the left side of the name is the priority when multiple action buttons overlap (the closer it is to 0, the higher the priority is displayed). - カテゴリ。名前の左側の数字は複数のアクションボタンが重なっていた場合の優先度(0に近い程優先表示)。
    pub category: u8,

    /// NAME: Padding 1 - パディング1
    pub padding1: [u8; 2],

    /// NAME: Damipoli 1 - ダミポリ1
    /// DESC: Specify the Damipoli ID that is the center of the bottom of the range. If there is no Damipoly or -1 is entered, the center coordinates will be the reference. - 範囲の底面の中心となるダミポリIDを指定する　ダミポリがない場合 or -1が入力されている場合は、中心座標が基準になる
    pub dummyPoly1: i32,

    /// NAME: Damipoli 2 - ダミポリ2
    /// DESC: Used only when the range type is a capsule. Additional Damipoly (capsule) that creates a line segment with two Damipoly - 範囲タイプがカプセルの場合のみ使用　ダミポリ2つで線分を作る追加ダミポリ(カプセル)
    pub dummyPoly2: i32,

    /// NAME: radius - 半径
    /// DESC: Radius (cylinder / capsule) - 半径(円柱・カプセル)
    pub radius: f32,

    /// NAME: angle - 角度
    /// DESC: Angle (cylinder) - 角度(円柱)
    pub angle: i32,

    /// NAME: depth - 奥行き
    /// DESC: Depth (prism) - 奥行き(角柱)
    pub depth: f32,

    /// NAME: width - 幅
    /// DESC: Width (prism) - 幅(角柱)
    pub width: f32,

    /// NAME: height - 高さ
    /// DESC: Height (cylinder / prism) - 高さ(円柱・角柱)
    pub height: f32,

    /// NAME: Bottom height offset - 底面高さオフセット
    /// DESC: How much to raise or lower the Y coordinate of the bottom (cylinder / prism) - 底面のY座標をどれだけ上下させるか(円柱・角柱)
    pub baseHeightOffset: f32,

    /// NAME: Angle difference judgment type - 角度差判定タイプ
    /// DESC: Angle difference judgment type (cylinder / prism) - 角度差判定タイプ(円柱・角柱)
    pub angleCheckType: u8,

    /// NAME: Padding 2 - パディング2
    pub padding2: [u8; 3],

    /// NAME: Allowable angle difference - 許容角度差
    /// DESC: Allowable angle difference (cylinder / prism) - 許容角度差(円柱・角柱)
    pub allowAngle: i32,

    /// NAME: Action spot Damipoli - アクションスポットダミポリ
    /// DESC: If there is no Damipoli that specifies the Damipoli ID that will be the position of the action spot, or if -1 is entered, the center coordinates will be the reference. - アクションスポットの位置となるダミポリIDを指定する ダミポリがない場合 or -1が入力されている場合は、中心座標が基準となる
    pub spotDummyPoly: i32,

    /// NAME: Text box type - テキストボックスタイプ
    /// DESC: Text box type - テキストボックスタイプ
    pub textBoxType: u8,

    /// NAME: Padding 3 - パディング3
    pub padding3: [u8; 2],

    /// NAME: Padding 5 - パディング5
    pub Bitfield1: u8,

    /// NAME: Text ID - テキストID
    /// DESC: Text ID to display - 表示するテキストID
    pub textId: i32,

    /// NAME: Invalid flag - 無効フラグ
    /// DESC: If this flag is ON, the action button will not appear and no judgment will be made. - このフラグがONだとアクションボタンが出ず、判定も行われない
    pub invalidFlag: u32,

    /// NAME: Gray out flag - グレーアウトフラグ
    /// DESC: If this flag is ON, the action button will be grayed out and no judgment will be made. - このフラグがONだとアクションボタンがグレーアウトし、判定も行われない
    pub grayoutFlag: u32,

    /// NAME: Replacement action button ID when riding - 騎乗時差し替えアクションボタンID
    /// DESC: Replace with this action button ID parameter while riding (-1: No replacement) - 騎乗中はこのアクションボタンIDのパラメータに差し替える（-1：差し替え無し）
    pub overrideActionButtonIdForRide: i32,

    /// NAME: Invalid time after execution - 実行後無効時間
    /// DESC: Invalid time after execution (-infinite by value) - 実行後無効時間(-値で無限)
    pub execInvalidTime: f32,

    /// NAME: Padding 6 - パディング6
    pub padding6: [u8; 28],
}

impl Paramdef for ACTIONBUTTON_PARAM_ST {
    const NAME: &'static str = "ACTIONBUTTON_PARAM_ST";
    const VERSION: u16 = 2;
}
impl ACTIONBUTTON_PARAM_ST {
    ///
    /// Bitfield1
    pub fn get_padding5(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_padding5(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// If this item is YES, the action button will not appear when riding and no judgment will be made. - この項目がYESだと騎乗時にアクションボタンが出なくなり、判定も行われない
    /// Bitfield1
    pub fn get_isInvalidForRide(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_isInvalidForRide(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// If this item is YES, the action button will be grayed out when riding and no judgment will be made. - この項目がYESだと騎乗時にアクションボタンがグレーアウトし、判定も行われない
    /// Bitfield1
    pub fn get_isGrayoutForRide(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_isGrayoutForRide(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// If this item is YES, the action button will not appear when crouching, and no judgment will be made. - この項目がYESだとしゃがみ時にアクションボタンが出なくなり、判定も行われない
    /// Bitfield1
    pub fn get_isInvalidForCrouching(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_isInvalidForCrouching(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
    /// If this item is YES, the action button will be grayed out when crouching and no judgment will be made. - この項目がYESだとしゃがみ時にアクションボタンがグレーアウトし、判定も行われない
    /// Bitfield1
    pub fn get_isGrayoutForCrouching(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_isGrayoutForCrouching(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xEF
        }
    }
    ///
    /// Bitfield1
    pub fn get_padding4(&self) -> u8 {
        &self.Bitfield1 & 0xE0
    }

    /// Bitfield1 MAX: 7
    pub fn set_padding4(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 5) & 0xE0;
            let newVal = &self.Bitfield1 & 0x1F | val;
            self.Bitfield1 = newVal
        } else {
            self.Bitfield1 &= 0x1F
        }
    }
}
