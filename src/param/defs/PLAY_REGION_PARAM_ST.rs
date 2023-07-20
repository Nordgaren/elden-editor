/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 9
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct PLAY_REGION_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Simple match area ID - 簡易マッチエリアID
    /// DESC: Simple match area ID - 簡易マッチエリアID
    pub matchAreaId: i32,

    /// NAME: Multiplayer start limit event flag ID - マルチプレイ開始制限イベントフラグID
    /// DESC: Multiplayer start limit event flag ID - マルチプレイ開始制限イベントフラグID
    pub multiPlayStartLimitEventFlagId: u32,

    /// NAME: Other distances that cannot be invaded by spirits - その他霊侵入不可能距離
    /// DESC: Other distances that cannot be invaded by spirits. The intrusion position is searched for the intrusion points within the "other ghost invasion impossible distance" to the "other ghost invasion range upper limit" from the host position. - その他霊侵入不可能距離。ホスト位置から「その他霊侵入不可能距離」～「その他霊侵入範囲上限」以内の侵入ポイントを対象に侵入位置検索を行う。
    pub otherDisableDistance: f32,

    /// NAME: PC position save limit event flag ID - PC位置セーブ制限イベントフラグID
    /// DESC: PC position save restriction event flag ID (flag ON: PC position save valid flag OFF: PC position save invalid 0: PC position save always valid) - PC位置セーブ制限イベントフラグID　（フラグON：PC位置セーブ有効　フラグOFF：PC位置セーブ無効　0：PC位置セーブ常に有効）
    pub pcPositionSaveLimitEventFlagId: u32,

    /// NAME: Boss area ID - ボスエリアID
    /// DESC: Areas with the same ID are treated as the same boss area. - このIDが同じものを設定された領域同士は、同一のボスエリアとして扱う。
    pub bossAreaId: u32,

    /// NAME: NPC White Spirit Summon Ritual Summon NPC Entity ID Free Frame ID - NPC白霊召喚儀式の召還NPCのエンティティIDの自由枠ID
    /// DESC: NPC White Spirit Summon Ritual Summon The beginning of the free frame ID used as the NPC's entity ID - NPC白霊召喚儀式の召還NPCのエンティティIDとして使われる自由枠IDの先頭
    pub cultNpcWhiteGhostEntityId_byFree: i16,

    /// NAME: Is it a map guardian area? - マップ守護領域か？
    /// DESC: Whether to increase or decrease the frame of the map protection area - マップ守護領域の枠の増減にするか
    pub bMapGuradianRegion: u8,

    /// NAME: Is it the old man sign area of yellow robe? - 黄衣の翁サイン領域か？
    /// DESC: Is it the old man sign area of yellow robe? - 黄衣の翁サイン領域か？
    pub Bitfield2: u8,

    /// NAME: Warp item permit bonfire ID1 - ワープアイテム許可篝火ID1
    /// DESC: Bonfire entity ID1 used to determine whether to allow the use of warp items - ワープアイテムの使用を許可する判定に使う篝火のエンティティID1
    pub warpItemUsePermitBonfireId_1: u32,

    /// NAME: Warp item permit bonfire ID2 - ワープアイテム許可篝火ID2
    /// DESC: Bonfire entity ID 2 used to determine permission to use warp items - ワープアイテムの使用を許可する判定に使う篝火のエンティティID2
    pub warpItemUsePermitBonfireId_2: u32,

    /// NAME: Warp item permit bonfire ID3 - ワープアイテム許可篝火ID3
    /// DESC: Bonfire entity ID 3 used to determine permission to use warp items - ワープアイテムの使用を許可する判定に使う篝火のエンティティID3
    pub warpItemUsePermitBonfireId_3: u32,

    /// NAME: Warp item permit bonfire ID4 - ワープアイテム許可篝火ID4
    /// DESC: Bonfire entity ID 4 used to determine permission to use warp items - ワープアイテムの使用を許可する判定に使う篝火のエンティティID4
    pub warpItemUsePermitBonfireId_4: u32,

    /// NAME: Warp item permit bonfire ID5 - ワープアイテム許可篝火ID5
    /// DESC: Bonfire entity ID 5 used to determine permission to use warp items - ワープアイテムの使用を許可する判定に使う篝火のエンティティID5
    pub warpItemUsePermitBonfireId_5: u32,

    /// NAME: Warp item prohibited event flag ID1 - ワープアイテム禁止イベントフラグID1
    /// DESC: Event flag ID 1 to determine the prohibition of warp items. Warp item permission Higher priority than judgment by bonfire ID - ワープアイテムの使用禁止を判定するイベントフラグID1。ワープアイテム許可篝火IDによる判定より優先度が上
    pub warpItemProhibitionEventFlagId_1: u32,

    /// NAME: Warp item prohibited event flag ID2 - ワープアイテム禁止イベントフラグID2
    /// DESC: Event flag ID 2 to determine the prohibition of warp items. Warp item permission Higher priority than judgment by bonfire ID - ワープアイテムの使用禁止を判定するイベントフラグID2。ワープアイテム許可篝火IDによる判定より優先度が上
    pub warpItemProhibitionEventFlagId_2: u32,

    /// NAME: Warp item prohibited event flag ID3 - ワープアイテム禁止イベントフラグID3
    /// DESC: Event flag ID 3 to determine the prohibition of warp items. Warp item permission Higher priority than judgment by bonfire ID - ワープアイテムの使用禁止を判定するイベントフラグID3。ワープアイテム許可篝火IDによる判定より優先度が上
    pub warpItemProhibitionEventFlagId_3: u32,

    /// NAME: Warp item prohibited event flag ID4 - ワープアイテム禁止イベントフラグID4
    /// DESC: Event flag ID 4 to determine the prohibition of warp items. Warp item permission Higher priority than judgment by bonfire ID - ワープアイテムの使用禁止を判定するイベントフラグID4。ワープアイテム許可篝火IDによる判定より優先度が上
    pub warpItemProhibitionEventFlagId_4: u32,

    /// NAME: Warp Item Ban Event Flag ID 5 - ワープアイテム禁止イベントフラグID5
    /// DESC: Event flag ID 5 to determine the prohibition of warp items. Warp item permission Higher priority than judgment by bonfire ID - ワープアイテムの使用禁止を判定するイベントフラグID5。ワープアイテム許可篝火IDによる判定より優先度が上
    pub warpItemProhibitionEventFlagId_5: u32,

    /// NAME: Effective bloodstain / death illusion - 血痕・死亡幻影有効
    /// DESC: Effective bloodstain / death illusion - 血痕・死亡幻影有効
    pub Bitfield3: u8,

    /// NAME: Is it automatic generation of intrusion points? - 侵入ポイント自動生成か
    /// DESC: Is it an intrusion point automatic generation? If it is set to ○, the intrusion position is searched by the automatically generated logic for the intrusion point. - 侵入ポイント自動生成か。○にした場合は自動生成された侵入ポイント用のロジックで侵入位置を検索。
    pub Bitfield4: u8,

    /// NAME: pad2 - pad2
    pub pad2: [u8; 2],

    /// NAME: Yellow robe's old man host restriction event flag - 黄衣の翁ホスト制限イベントフラグ
    /// DESC: Yellow Coat Host Restriction Event Flag: When this flag is turned on, multiplayer as a host of Yellow Coat is prohibited. It is assumed that the block clear flag will be inserted. 0: No limit - 黄衣の翁ホスト制限イベントフラグ：このフラグがONになると黄衣の翁のホストとしてのマルチプレイが禁止される。ブロッククリアフラグを入れる想定。0：制限しない
    pub multiPlayHASHostLimitEventFlagId: u32,

    /// NAME: Other ghost invasion range upper limit - その他霊侵入範囲上限
    /// DESC: Other ghost invasion range upper limit. The intrusion position is searched for the intrusion points within the "other ghost invasion impossible distance" to the "other ghost invasion range upper limit" from the host position. - その他霊侵入範囲上限。ホスト位置から「その他霊侵入不可能距離」～「その他霊侵入範囲上限」以内の侵入ポイントを対象に侵入位置検索を行う。
    pub otherMaxDistance: f32,

    /// NAME: Sign pool release event flag ID - サイン溜まり解放イベントフラグID
    /// DESC: Sign accumulation release event flag ID - サイン溜まり解放イベントフラグID
    pub signPuddleOpenEventFlagId: u32,

    /// NAME: Map display_area number (mXX_00_00_00) - 地図表示用_エリア番号（mXX_00_00_00）
    /// DESC: Area number (mXX_00_00_00). Data for specifying the display position in the map menu - エリア番号（mXX_00_00_00）。地図メニューでの表示位置を指定するためのデータ
    pub areaNo: u8,

    /// NAME: Map display_grid X number (m00_XX_00_00) - 地図表示用_グリッドX番号（m00_XX_00_00）
    /// DESC: Grid X number (m00_XX_00_00). Data for specifying the display position in the map menu - グリッドX番号（m00_XX_00_00）。地図メニューでの表示位置を指定するためのデータ
    pub gridXNo: u8,

    /// NAME: Map display_grid Z number (m00_00_XX_00) - 地図表示用_グリッドZ番号（m00_00_XX_00）
    /// DESC: Grid Z number (m00_00_XX_00). Data for specifying the display position in the map menu - グリッドZ番号（m00_00_XX_00）。地図メニューでの表示位置を指定するためのデータ
    pub gridZNo: u8,

    /// NAME: pad4 - pad4
    pub pad4: [u8; 1],

    /// NAME: _X coordinates for map display - 地図表示用_X座標
    /// DESC: X coordinates (relative coordinates from the specified map). Data for specifying the display position in the map menu - X座標（指定したマップからの相対座標）。地図メニューでの表示位置を指定するためのデータ
    pub posX: f32,

    /// NAME: _Y coordinates for map display - 地図表示用_Y座標
    /// DESC: Y coordinates (relative coordinates from the specified map). Data for specifying the display position in the map menu. Not actually used, but keep XYZ aligned - Y座標（指定したマップからの相対座標）。地図メニューでの表示位置を指定するためのデータ。実際には使われていないがXYZ一揃えにしておく
    pub posY: f32,

    /// NAME: _Z coordinates for map display - 地図表示用_Z座標
    /// DESC: Z coordinates (relative coordinates from the specified map). Data for specifying the display position in the map menu - Z座標（指定したマップからの相対座標）。地図メニューでの表示位置を指定するためのデータ
    pub posZ: f32,

    /// NAME: Intrusion restriction event flag ID1 - 侵入制限イベントフラグID1
    /// DESC: Intrusion restriction event flag ID1 - 侵入制限イベントフラグID1
    pub breakInLimitEventFlagId_1: u32,

    /// NAME: White sign limit event flag ID1 - 白サイン制限イベントフラグID1
    /// DESC: White sign limit event flag ID1 - 白サイン制限イベントフラグID1
    pub whiteSignLimitEventFlagId_1: u32,

    /// NAME: Sign accumulation registration restriction Event flag ID - サイン溜まり登録制限イベントフラグID
    /// DESC: Sign accumulation registration restriction Event flag ID (Flag ON: Sign accumulation registration is permitted Flag OFF: Sign accumulation registration is prohibited 0: Sign accumulation registration is always permitted) - サイン溜まり登録制限イベントフラグID　（フラグON：サイン溜まり登録を許可　フラグOFF：サイン溜まり登録を禁止　0：サイン溜まり登録を常に許可）
    pub matchAreaSignCreateLimitEventFlagId: u32,

    /// NAME: Multipurpose ID 01 - マルチ目的ID01
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_1: u32,

    /// NAME: Multipurpose ID 02 - マルチ目的ID02
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_2: u32,

    /// NAME: Multipurpose ID 03 - マルチ目的ID03
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_3: u32,

    /// NAME: Multipurpose ID 04 - マルチ目的ID04
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_4: u32,

    /// NAME: Multipurpose ID 05 - マルチ目的ID05
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_5: u32,

    /// NAME: Multipurpose ID 06 - マルチ目的ID06
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_6: u32,

    /// NAME: Multipurpose ID 07 - マルチ目的ID07
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_7: u32,

    /// NAME: Multipurpose ID08 - マルチ目的ID08
    /// DESC: Multi-purpose ID to be displayed in the list when setting the purpose - 目的設定時にリストに表示するマルチ目的ID
    pub signAimId_8: u32,

    /// NAME: Red sign limit event flag ID1 - 赤サイン制限イベントフラグID1
    /// DESC: Red sign limit event flag ID1 - 赤サイン制限イベントフラグID1
    pub redSignLimitEventFlagId_1: u32,

    /// NAME: Intrusion restriction event flag ID2 - 侵入制限イベントフラグID2
    /// DESC: Intrusion restriction event flag ID2 - 侵入制限イベントフラグID2
    pub breakInLimitEventFlagId_2: u32,

    /// NAME: Intrusion restriction event flag ID3 - 侵入制限イベントフラグID3
    /// DESC: Intrusion restriction event flag ID3 - 侵入制限イベントフラグID3
    pub breakInLimitEventFlagId_3: u32,

    /// NAME: White sign limit event flag ID2 - 白サイン制限イベントフラグID2
    /// DESC: White sign limit event flag ID2 - 白サイン制限イベントフラグID2
    pub whiteSignLimitEventFlagId_2: u32,

    /// NAME: White sign limit event flag ID3 - 白サイン制限イベントフラグID3
    /// DESC: White sign limit event flag ID3 - 白サイン制限イベントフラグID3
    pub whiteSignLimitEventFlagId_3: u32,

    /// NAME: Red sign limit event flag ID2 - 赤サイン制限イベントフラグID2
    /// DESC: Red sign limit event flag ID2 - 赤サイン制限イベントフラグID2
    pub redSignLimitEventFlagId_2: u32,

    /// NAME: Red sign limit event flag ID3 - 赤サイン制限イベントフラグID3
    /// DESC: Red sign limit event flag ID3 - 赤サイン制限イベントフラグID3
    pub redSignLimitEventFlagId_3: u32,

    /// NAME: Area boss ID01 - 領域内ボスID01
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_1: u32,

    /// NAME: Area boss ID02 - 領域内ボスID02
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_2: u32,

    /// NAME: Area boss ID03 - 領域内ボスID03
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_3: u32,

    /// NAME: Area boss ID04 - 領域内ボスID04
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_4: u32,

    /// NAME: Area Boss ID 05 - 領域内ボスID05
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_5: u32,

    /// NAME: Area Boss ID 06 - 領域内ボスID06
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_6: u32,

    /// NAME: Area Boss ID 07 - 領域内ボスID07
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_7: u32,

    /// NAME: Area boss ID08 - 領域内ボスID08
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_8: u32,

    /// NAME: Area Boss ID 09 - 領域内ボスID09
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_9: u32,

    /// NAME: Area boss ID 10 - 領域内ボスID10
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_10: u32,

    /// NAME: In-area boss ID 11 - 領域内ボスID11
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_11: u32,

    /// NAME: In-area boss ID 12 - 領域内ボスID12
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_12: u32,

    /// NAME: Area boss ID 13 - 領域内ボスID13
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_13: u32,

    /// NAME: Area boss ID14 - 領域内ボスID14
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_14: u32,

    /// NAME: Area boss ID15 - 領域内ボスID15
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_15: u32,

    /// NAME: Area boss ID 16 - 領域内ボスID16
    /// DESC: Area boss ID. It is used to select the target boss when "Is the intrusion point automatically generated?" Is ○. - 領域内ボスID。「侵入ポイント自動生成か」が○のときに目的とするボスを選ぶのに使われる。
    pub bossId_16: u32,

    /// NAME: Map display_event flag ID - 地図表示用_イベントフラグID
    /// DESC: Map display_event flag ID (0: always displayed). Only when this event flag is set, it will be displayed in a lively manner in the map menu. - 地図表示用_イベントフラグID(0:常に表示)。このイベントフラグが立っているときだけ、マップメニューに賑わい表示される
    pub mapMenuUnlockEventId: u32,

    /// NAME: pad5 - pad5
    pub pad5: [u8; 32],
}

impl Paramdef for PLAY_REGION_PARAM_ST {
    const NAME: &'static str = "PLAY_REGION_PARAM_ST";
    const VERSION: u16 = 9;
}
impl PLAY_REGION_PARAM_ST {
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
    /// Is it the old man sign area of yellow robe? - 黄衣の翁サイン領域か？
    /// Bitfield2
    pub fn get_bYellowCostumeRegion(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_bYellowCostumeRegion(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Flag state that limits "multiplayer start restriction event flag ID" - 「マルチプレイ開始制限イベントフラグID」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_multiPlayStartLimitEventFlagId_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_multiPlayStartLimitEventFlagId_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Flag state that limits "intrusion restriction event flag ID1" - 「侵入制限イベントフラグID1」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_breakInLimitEventFlagId_1_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_breakInLimitEventFlagId_1_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Flag state that limits "white sign restriction event flag ID1" - 「白サイン制限イベントフラグID1」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_whiteSignLimitEventFlagId_1_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_whiteSignLimitEventFlagId_1_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    /// Flag state that limits "red sign limit event flag ID1" - 「赤サイン制限イベントフラグID1」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_redSignLimitEventFlagId_1_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x10 != 0
    }

    /// Bitfield2
    pub fn set_redSignLimitEventFlagId_1_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x10
        } else {
            self.Bitfield2 &= 0xEF
        }
    }
    /// Flag state that limits "intrusion restriction event flag ID2" - 「侵入制限イベントフラグID2」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_breakInLimitEventFlagId_2_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x20 != 0
    }

    /// Bitfield2
    pub fn set_breakInLimitEventFlagId_2_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x20
        } else {
            self.Bitfield2 &= 0xDF
        }
    }
    /// Flag state that limits "intrusion restriction event flag ID3" - 「侵入制限イベントフラグID3」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_breakInLimitEventFlagId_3_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x40 != 0
    }

    /// Bitfield2
    pub fn set_breakInLimitEventFlagId_3_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x40
        } else {
            self.Bitfield2 &= 0xBF
        }
    }
    /// Flag state that limits "white sign restriction event flag ID2" - 「白サイン制限イベントフラグID2」の制限を行うフラグ状態
    /// Bitfield2
    pub fn get_whiteSignLimitEventFlagId_2_targetFlagState(&self) -> bool {
        &self.Bitfield2 & 0x80 != 0
    }

    /// Bitfield2
    pub fn set_whiteSignLimitEventFlagId_2_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x80
        } else {
            self.Bitfield2 &= 0x7F
        }
    }
    /// Effective bloodstain / death illusion - 血痕・死亡幻影有効
    /// Bitfield3
    pub fn get_enableBloodstain(&self) -> bool {
        &self.Bitfield3 & 0x1 != 0
    }

    /// Bitfield3
    pub fn set_enableBloodstain(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x1
        } else {
            self.Bitfield3 &= 0xFE
        }
    }
    /// Blood character valid - 血文字有効
    /// Bitfield3
    pub fn get_enableBloodMessage(&self) -> bool {
        &self.Bitfield3 & 0x2 != 0
    }

    /// Bitfield3
    pub fn set_enableBloodMessage(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x2
        } else {
            self.Bitfield3 &= 0xFD
        }
    }
    /// Phantom effective - 幻影有効
    /// Bitfield3
    pub fn get_enableGhost(&self) -> bool {
        &self.Bitfield3 & 0x4 != 0
    }

    /// Bitfield3
    pub fn set_enableGhost(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x4
        } else {
            self.Bitfield3 &= 0xFB
        }
    }
    /// Whether to display on map M00 - 地図M00で表示するか
    /// Bitfield3
    pub fn get_dispMask00(&self) -> bool {
        &self.Bitfield3 & 0x8 != 0
    }

    /// Bitfield3
    pub fn set_dispMask00(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x8
        } else {
            self.Bitfield3 &= 0xF7
        }
    }
    /// Whether to display on map M01 - 地図M01で表示するか
    /// Bitfield3
    pub fn get_dispMask01(&self) -> bool {
        &self.Bitfield3 & 0x10 != 0
    }

    /// Bitfield3
    pub fn set_dispMask01(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x10
        } else {
            self.Bitfield3 &= 0xEF
        }
    }
    /// Flag state that limits "white sign restriction event flag ID3" - 「白サイン制限イベントフラグID3」の制限を行うフラグ状態
    /// Bitfield3
    pub fn get_whiteSignLimitEventFlagId_3_targetFlagState(&self) -> bool {
        &self.Bitfield3 & 0x20 != 0
    }

    /// Bitfield3
    pub fn set_whiteSignLimitEventFlagId_3_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x20
        } else {
            self.Bitfield3 &= 0xDF
        }
    }
    /// Flag state that limits "red sign limit event flag ID2" - 「赤サイン制限イベントフラグID2」の制限を行うフラグ状態
    /// Bitfield3
    pub fn get_redSignLimitEventFlagId_2_targetFlagState(&self) -> bool {
        &self.Bitfield3 & 0x40 != 0
    }

    /// Bitfield3
    pub fn set_redSignLimitEventFlagId_2_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x40
        } else {
            self.Bitfield3 &= 0xBF
        }
    }
    /// Flag state that limits "red sign limit event flag ID3" - 「赤サイン制限イベントフラグID3」の制限を行うフラグ状態
    /// Bitfield3
    pub fn get_redSignLimitEventFlagId_3_targetFlagState(&self) -> bool {
        &self.Bitfield3 & 0x80 != 0
    }

    /// Bitfield3
    pub fn set_redSignLimitEventFlagId_3_targetFlagState(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x80
        } else {
            self.Bitfield3 &= 0x7F
        }
    }
    /// Is it an intrusion point automatic generation? If it is set to ○, the intrusion position is searched by the automatically generated logic for the intrusion point. - 侵入ポイント自動生成か。○にした場合は自動生成された侵入ポイント用のロジックで侵入位置を検索。
    /// Bitfield4
    pub fn get_isAutoIntrudePoint(&self) -> bool {
        &self.Bitfield4 & 0x1 != 0
    }

    /// Bitfield4
    pub fn set_isAutoIntrudePoint(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x1
        } else {
            self.Bitfield4 &= 0xFE
        }
    }
    ///
    /// Bitfield4
    pub fn get_pad1(&self) -> u8 {
        &self.Bitfield4 & 0xFE
    }

    /// Bitfield4 MAX: 127
    pub fn set_pad1(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 1) & 0xFE;
            let newVal = &self.Bitfield4 & 0x1 | val;
            self.Bitfield4 = newVal
        } else {
            self.Bitfield4 &= 0x1
        }
    }
}
