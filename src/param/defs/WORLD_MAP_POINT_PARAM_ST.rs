/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 6
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct WORLD_MAP_POINT_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Release event flag ID - 開放イベントフラグID
    /// DESC: Event flag ID of open condition - 開放条件のイベントフラグID
    pub eventFlagId: u32,

    /// NAME: Far-viewing platform discovery event flag ID - 遠見台発見イベントフラグID
    /// DESC: Event flag ID to be set when found on the distant view - 遠見台で発見した際に立てるイベントフラグID
    pub distViewEventFlagId: u32,

    /// NAME: Icon ID - アイコンID
    /// DESC: Icon ID - アイコンID
    pub iconId: u16,

    /// NAME: BGM location information (inside the entrance area) - BGM用場所情報（入場領域内）
    /// DESC: Bgm location type - Bgm場所タイプ
    pub bgmPlaceType: i16,

    /// NAME: Is it a range icon? - 範囲アイコンか
    /// DESC: Is it an icon that represents a range? Same size as the map - 範囲を表すアイコンか。地図に対して等倍になる
    pub Bitfield2: u8,

    /// NAME: Overwrite distant view mark_area number - 遠見台目印上書き_エリア番号
    /// DESC: AA part of mAA_BB_CC_DD - mAA_BB_CC_DD の AA 部分
    pub areaNo_forDistViewMark: u8,

    /// NAME: Overwrite distant view mark_Grid X number - 遠見台目印上書き_グリッドX番号
    /// DESC: BB part of mAA_BB_CC_DD - mAA_BB_CC_DD の BB 部分
    pub gridXNo_forDistViewMark: u8,

    /// NAME: Overwrite distant view mark_Grid Z number - 遠見台目印上書き_グリッドZ番号
    /// DESC: CC part of mAA_BB_CC_DD - mAA_BB_CC_DD の CC 部分
    pub gridZNo_forDistViewMark: u8,

    /// NAME: Cleared event flag ID - クリア済イベントフラグID
    /// DESC: Cleared event flag ID (0: always treated as cleared) - クリア済みイベントフラグID(0:常にクリア済み扱い)
    pub clearedEventFlagId: u32,

    /// NAME: Display setting M00 - 表示設定M00
    /// DESC: Whether to display with M00 - M00で表示するか
    pub Bitfield3: u8,

    /// NAME: pad - パッド
    /// DESC: pad2 - pad2
    pub pad2: [u8; 1],

    /// NAME: Icon ID when distant view is found - 遠見台発見時アイコンID
    /// DESC: Icon ID when distant view is found - 遠見台発見時アイコンID
    pub distViewIconId: u16,

    /// NAME: Icon angle [deg] - アイコン角度[deg]
    /// DESC: Display icon rotation angle [deg] - 表示アイコンの回転角度[deg]
    pub angle: f32,

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
    pub pad: [u8; 1],

    /// NAME: X coordinate - X座標
    /// DESC: X coordinate - X座標
    pub posX: f32,

    /// NAME: Y coordinate - Y座標
    /// DESC: Y coordinate (not used) - Y座標（使っていない）
    pub posY: f32,

    /// NAME: Z coordinate - Z座標
    /// DESC: Z coordinate - Z座標
    pub posZ: f32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If the value is invalid (-1), nothing is displayed. - 表示するテキストID。無効値(-1)なら、何も表示しない
    pub textId1: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (0) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(0)なら、On扱いされる
    pub textEnableFlagId1: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (0) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(0)なら、Off扱いされる
    pub textDisableFlagId1: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-2), nothing is displayed. - 表示するテキストID。無効値(-2)なら、何も表示しない
    pub textId2: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (1) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(1)なら、On扱いされる
    pub textEnableFlagId2: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (1) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(1)なら、Off扱いされる
    pub textDisableFlagId2: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-3), nothing is displayed. - 表示するテキストID。無効値(-3)なら、何も表示しない
    pub textId3: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (2) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(2)なら、On扱いされる
    pub textEnableFlagId3: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (2) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(2)なら、Off扱いされる
    pub textDisableFlagId3: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-4), nothing is displayed. - 表示するテキストID。無効値(-4)なら、何も表示しない
    pub textId4: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (3) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(3)なら、On扱いされる
    pub textEnableFlagId4: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (3) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(3)なら、Off扱いされる
    pub textDisableFlagId4: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-5), nothing is displayed. - 表示するテキストID。無効値(-5)なら、何も表示しない
    pub textId5: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (4) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(4)なら、On扱いされる
    pub textEnableFlagId5: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (4) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(4)なら、Off扱いされる
    pub textDisableFlagId5: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-6), nothing is displayed. - 表示するテキストID。無効値(-6)なら、何も表示しない
    pub textId6: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (5) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(5)なら、On扱いされる
    pub textEnableFlagId6: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (5) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(5)なら、Off扱いされる
    pub textDisableFlagId6: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-7), nothing is displayed. - 表示するテキストID。無効値(-7)なら、何も表示しない
    pub textId7: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (6) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(6)なら、On扱いされる
    pub textEnableFlagId7: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (6) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(6)なら、Off扱いされる
    pub textDisableFlagId7: u32,

    /// NAME: Text ID - テキストID
    /// DESC: The text ID to display. If it is an invalid value (-8), nothing is displayed. - 表示するテキストID。無効値(-8)なら、何も表示しない
    pub textId8: i32,

    /// NAME: Occurrence event flag ID - 出現イベントフラグID
    /// DESC: Display text Event flag ID. Display if the event flag is On. If the event flag ID (7) is invalid, it will be treated as On. - テキストの表示イベントフラグID。イベントフラグがOnなら表示する。無効なイベントフラグID(7)なら、On扱いされる
    pub textEnableFlagId8: u32,

    /// NAME: Hidden event flag ID - 非表示イベントフラグID
    /// DESC: Hidden event flag ID of the text. If the event flag is On, it will not be displayed. It takes precedence over the display event flag ID. If the event flag ID (7) is invalid, it will be treated as Off. - テキストの非表示イベントフラグID。イベントフラグがOnなら表示しない。表示イベントフラグIDよりも優先される。無効なイベントフラグID(7)なら、Off扱いされる
    pub textDisableFlagId8: u32,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType1: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType2: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType3: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType4: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType5: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType6: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType7: u8,

    /// NAME: Text type - テキスト種別
    /// DESC: Text type (place name, NPC name, ...) - テキストの種別(地名,NPC名,...)
    pub textType8: u8,

    /// NAME: Distance stand ID0 - 遠見台ID0
    /// DESC: Distance stand ID - 遠見台ID
    pub distViewId: i32,

    /// NAME: Distance viewing marker overwrite _X coordinates - 遠見台目印上書き_X座標
    /// DESC: X coordinate - X座標
    pub posX_forDistViewMark: f32,

    /// NAME: Distance viewing mark overwrite Y coordinate - 遠見台目印上書きY座標
    /// DESC: Y coordinate - Y座標
    pub posY_forDistViewMark: f32,

    /// NAME: Distant mark overwrite Z coordinate - 遠見台目印上書きZ座標
    /// DESC: Z coordinate - Z座標
    pub posZ_forDistViewMark: f32,

    /// NAME: Distance stand ID1 - 遠見台ID1
    /// DESC: Distance stand ID - 遠見台ID
    pub distViewId1: i32,

    /// NAME: Distance stand ID2 - 遠見台ID2
    /// DESC: Distance stand ID - 遠見台ID
    pub distViewId2: i32,

    /// NAME: Distance stand ID3 - 遠見台ID3
    /// DESC: Distance stand ID - 遠見台ID
    pub distViewId3: i32,

    /// NAME: Display zoom step - 表示ズームステップ
    /// DESC: Zoom step to display map points (0 when zoomed out most, +1 for each zoom). Displayed when "<< Display zoom step >> ≤ Current zoom step". Default is 0 (always displayed) - 地図ポイントを表示するズームステップ（一番ズームアウトした状態が0、ズームするごとに+1）。「《表示ズームステップ》≦ 現在のズームステップ 」のときに表示される。デフォルトは 0（常に表示）
    pub dispMinZoomStep: u8,

    /// NAME: Selectable zoom steps - 選択可能ズームステップ
    /// DESC: Zoom step where map points can be selected (0 when zoomed out most, +1 for each zoom). Selectable when "<< Selectable zoom step >> ≤ Current enlargement stage". Default is 0 (always selectable) - 地図ポイントを選択可能なズームステップ（一番ズームアウトした状態が0、ズームするごとに+1）。「《選択可能ズームステップ》≦ 現在の拡大段階 」のときに選択可能。デフォルトは 0（常に選択可能）
    pub selectMinZoomStep: u8,

    /// NAME: Admission display format - 入場表示形式
    /// DESC: Admission display format. Map points to be displayed at the time of admission Types of admission FE - 入場表示形式。入場時に表示する地図ポイント入場FEの種類
    pub entryFEType: u8,

    /// NAME: pad - パッド
    /// DESC: pad3 - pad3
    pub pad4: [u8; 9],

    /// NAME: unkC0 - unkC0
    pub unkC0: i32,

    /// NAME: unkC4 - unkC4
    pub unkC4: i32,

    /// NAME: unkC8 - unkC8
    pub unkC8: i32,

    /// NAME: unkCC - unkCC
    pub unkCC: i32,

    /// NAME: unkD0 - unkD0
    pub unkD0: i32,

    /// NAME: unkD4 - unkD4
    pub unkD4: i32,

    /// NAME: unkD8 - unkD8
    pub unkD8: i32,

    /// NAME: unkDC - unkDC
    pub unkDC: i32,

    /// NAME: unkE0 - unkE0
    pub unkE0: i32,

    /// NAME: unkE4 - unkE4
    pub unkE4: i32,

    /// NAME: unkE8 - unkE8
    pub unkE8: i32,

    /// NAME: unkEC - unkEC
    pub unkEC: i32,

    /// NAME: unkF0 - unkF0
    pub unkF0: i32,

    /// NAME: unkF4 - unkF4
    pub unkF4: i32,

    /// NAME: unkF8 - unkF8
    pub unkF8: i32,

    /// NAME: unkFC - unkFC
    pub unkFC: i32,
}

impl Paramdef for WORLD_MAP_POINT_PARAM_ST {
    const NAME: &'static str = "WORLD_MAP_POINT_PARAM_ST";
    const VERSION: u16 = 6;
}
impl WORLD_MAP_POINT_PARAM_ST {
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
    /// Is it an icon that represents a range? Same size as the map - 範囲を表すアイコンか。地図に対して等倍になる
    /// Bitfield2
    pub fn get_isAreaIcon(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_isAreaIcon(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Whether to overwrite the coordinates when using it as a distant view marker - 遠見台目印として使うときに座標を上書きするか
    /// Bitfield2
    pub fn get_isOverrideDistViewMarkPos(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_isOverrideDistViewMarkPos(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Do you want to display it even when there is no text? Basically, points are not displayed without text. Display without text when this flag is enabled - テキストが無いときにも表示するか。基本的にはテキストがなければポイントは表示しない。このフラグが有効なときにはテキストがなくても表示する
    /// Bitfield2
    pub fn get_isEnableNoText(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_isEnableNoText(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    ///
    /// Bitfield2
    pub fn get_pad3(&self) -> u8 {
        &self.Bitfield2 & 0xF8
    }

    /// Bitfield2 MAX: 31
    pub fn set_pad3(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 3) & 0xF8;
            let newVal = &self.Bitfield2 & 0x7 | val;
            self.Bitfield2 = newVal
        } else {
            self.Bitfield2 &= 0x7
        }
    }
    /// Whether to display with M00 - M00で表示するか
    /// Bitfield3
    pub fn get_dispMask00(&self) -> bool {
        &self.Bitfield3 & 0x1 != 0
    }

    /// Bitfield3
    pub fn set_dispMask00(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x1
        } else {
            self.Bitfield3 &= 0xFE
        }
    }
    /// Whether to display with M01 - M01で表示するか
    /// Bitfield3
    pub fn get_dispMask01(&self) -> bool {
        &self.Bitfield3 & 0x2 != 0
    }

    /// Bitfield3
    pub fn set_dispMask01(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x2
        } else {
            self.Bitfield3 &= 0xFD
        }
    }
    /// pad2_0 - pad2_0
    /// Bitfield3
    pub fn get_pad2_0(&self) -> u8 {
        &self.Bitfield3 & 0xFC
    }

    /// Bitfield3 MAX: 63
    pub fn set_pad2_0(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 2) & 0xFC;
            let newVal = &self.Bitfield3 & 0x3 | val;
            self.Bitfield3 = newVal
        } else {
            self.Bitfield3 &= 0x3
        }
    }
}
