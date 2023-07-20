/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 6
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BONFIRE_WARP_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Event flag ID - イベントフラグID
    /// DESC: Release condition event flag ID - 解除条件イベントフラグID
    pub eventflagId: u32,

    /// NAME: Bonfire entity ID - 篝火エンティティID
    /// DESC: Bonfire entity ID - 篝火エンティティID
    pub bonfireEntityId: u32,

    /// NAME: pad - パッド
    /// DESC: Padding. The place where the deleted old work origin data was defined - パディング。削除した旧作由来データが定義されてた場所
    pub pad4: [u8; 2],

    /// NAME: Sort ID - ソートID
    /// DESC: Bonfire Warp Subcategory Sort ID. Specify the order (ascending order) in the same subcategory - 篝火ワープサブカテゴリソートID。同じサブカテゴリ内の並び順（昇順）を指定する
    pub bonfireSubCategorySortId: u16,

    /// NAME: Warp Prohibition Icon ID - ワープ禁止アイコンID
    /// DESC: Icon ID when warp is prohibited - ワープ禁止時のアイコンID
    pub forbiddenIconId: u16,

    /// NAME: Display zoom step - 表示ズームステップ
    /// DESC: Zoom step to display the bonfire (0 when zoomed out most, +1 for each zoom). Displayed when "<< Display zoom step >> ≤ Current zoom step". Default is 0 (always displayed) - 篝火を表示するズームステップ（一番ズームアウトした状態が0、ズームするごとに+1）。「《表示ズームステップ》≦ 現在のズームステップ 」のときに表示される。デフォルトは 0（常に表示）
    pub dispMinZoomStep: u8,

    /// NAME: Selectable zoom steps - 選択可能ズームステップ
    /// DESC: A zoom step that allows you to select and snap a bonfire (0 for the most zoomed out state, +1 for each zoom). Can be selected and snapped when "<< Selectable zoom step >> ≤ current enlargement stage". The default is 0 (always selectable / snappable) - 篝火を選択及びスナップ可能なズームステップ（一番ズームアウトした状態が0、ズームするごとに+1）。「《選択可能ズームステップ》≦ 現在の拡大段階 」のときに選択及びスナップ可能。デフォルトは 0（常に選択・スナップ可能）
    pub selectMinZoomStep: u8,

    /// NAME: Subcategory ID - サブカテゴリID
    /// DESC: Bonfire Warp Subcategory Parameter ID (-1: Invalid). Set which subcategory it belongs to. If it is invalid, it will not be displayed in the bonfire list. - 篝火ワープサブカテゴリパラメータID(-1:無効)。どのサブカテゴリに属するかを設定する。無効なら篝火一覧に表示されない
    pub bonfireSubCategoryId: i32,

    /// NAME: Cleared event flag ID - クリア済イベントフラグID
    /// DESC: Cleared event flag ID (0: always treated as cleared) - クリア済みイベントフラグID(0:常にクリア済み扱い)
    pub clearedEventFlagId: u32,

    /// NAME: Icon ID - アイコンID
    /// DESC: Icon ID - アイコンID
    pub iconId: u16,

    /// NAME: Display setting M00 - 表示設定M00
    /// DESC: Whether to display with M00 - M00で表示するか
    pub Bitfield2: u8,

    /// NAME: pad - パッド
    /// DESC: pad2 - pad2
    pub pad2: [u8; 1],

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
    /// DESC: pad3 - pad3
    pub pad3: [u8; 1],

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

    /// NAME: Before ignition SFX Damipoly ID0 - 点火前SFXダミポリID0
    /// DESC: Damipoli ID that issues SFX before bonfire ignition - 篝火点火前にSFXを出すダミポリID
    pub noIgnitionSfxDmypolyId_0: i32,

    /// NAME: Before ignition SFXID0 - 点火前SFXID0
    /// DESC: SFX ID issued before bonfire ignition. It goes out when ignited. In case of -1, SFX is not issued. - 篝火点火前に出すSFXID。点火したら消える。-1の場合はSFXを出さない。
    pub noIgnitionSfxId_0: i32,

    /// NAME: Before ignition SFX Damipoly ID1 - 点火前SFXダミポリID1
    /// DESC: Damipoli ID that issues SFX before bonfire ignition - 篝火点火前にSFXを出すダミポリID
    pub noIgnitionSfxDmypolyId_1: i32,

    /// NAME: Before ignition SFXID1 - 点火前SFXID1
    /// DESC: SFX ID issued before bonfire ignition. It goes out when ignited. In case of -1, SFX is not issued. - 篝火点火前に出すSFXID。点火したら消える。-1の場合はSFXを出さない。
    pub noIgnitionSfxId_1: i32,

    /// NAME: unkA8 - unkA8
    pub unkA8: i32,

    /// NAME: unkAC - unkAC
    pub unkAC: i32,

    /// NAME: unkB0 - unkB0
    pub unkB0: i32,

    /// NAME: unkB4 - unkB4
    pub unkB4: i32,

    /// NAME: unkB8 - unkB8
    pub unkB8: i32,

    /// NAME: unkBC - unkBC
    pub unkBC: i32,

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
}

impl Paramdef for BONFIRE_WARP_PARAM_ST {
    const NAME: &'static str = "BONFIRE_WARP_PARAM_ST";
    const VERSION: u16 = 6;
}
impl BONFIRE_WARP_PARAM_ST {
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
    /// Whether to display with M00 - M00で表示するか
    /// Bitfield2
    pub fn get_dispMask00(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_dispMask00(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Whether to display with M01 - M01で表示するか
    /// Bitfield2
    pub fn get_dispMask01(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_dispMask01(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// pad1: 6 - pad1:6
    /// Bitfield2
    pub fn get_pad1(&self) -> u8 {
        &self.Bitfield2 & 0xFC
    }

    /// Bitfield2 MAX: 63
    pub fn set_pad1(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 2) & 0xFC;
            let newVal = &self.Bitfield2 & 0x3 | val;
            self.Bitfield2 = newVal
        } else {
            self.Bitfield2 &= 0x3
        }
    }
}
