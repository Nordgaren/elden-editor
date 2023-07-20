/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct ASSET_MODEL_SFX_PARAM_ST {
    /// NAME: 0: SfxID - 0:SfxID
    /// DESC: 0: SfxID - 0:SfxID
    pub sfxId_0: i32,

    /// NAME: 0: Damipoli ID - 0：ダミポリID
    /// DESC: 0: Damipoli ID - 0：ダミポリID
    pub dmypolyId_0: i32,

    /// NAME: 0: Reservation - 0:予約
    /// DESC: 0: Reservation - 0:予約
    pub reserve_0: [u8; 8],

    /// NAME: 1: SfxID - 1:SfxID
    /// DESC: 1: SfxID - 1:SfxID
    pub sfxId_1: i32,

    /// NAME: 1: Damipoli ID - 1：ダミポリID
    /// DESC: 1: Damipoli ID - 1：ダミポリID
    pub dmypolyId_1: i32,

    /// NAME: 1: Reservation - 1:予約
    /// DESC: 1: Reservation - 1:予約
    pub reserve_1: [u8; 8],

    /// NAME: 2: SfxID - 2:SfxID
    /// DESC: 2: SfxID - 2:SfxID
    pub sfxId_2: i32,

    /// NAME: 2: Damipoli ID - 2：ダミポリID
    /// DESC: 2: Damipoli ID - 2：ダミポリID
    pub dmypolyId_2: i32,

    /// NAME: 2: Reservation - 2:予約
    /// DESC: 2: Reservation - 2:予約
    pub reserve_2: [u8; 8],

    /// NAME: 3: SfxID - 3:SfxID
    /// DESC: 3: SfxID - 3:SfxID
    pub sfxId_3: i32,

    /// NAME: 3: Damipoli ID - 3：ダミポリID
    /// DESC: 3: Damipoli ID - 3：ダミポリID
    pub dmypolyId_3: i32,

    /// NAME: 3: Reservation - 3:予約
    /// DESC: 3: Reservation - 3:予約
    pub reserve_3: [u8; 8],

    /// NAME: 4: SfxID - 4:SfxID
    /// DESC: 4: SfxID - 4:SfxID
    pub sfxId_4: i32,

    /// NAME: 4: Damipoli ID - 4：ダミポリID
    /// DESC: 4: Damipoli ID - 4：ダミポリID
    pub dmypolyId_4: i32,

    /// NAME: 4: Reservation - 4:予約
    /// DESC: 4: Reservation - 4:予約
    pub reserve_4: [u8; 8],

    /// NAME: 5: SfxID - 5:SfxID
    /// DESC: 5: SfxID - 5:SfxID
    pub sfxId_5: i32,

    /// NAME: 5: Damipoli ID - 5：ダミポリID
    /// DESC: 5: Damipoli ID - 5：ダミポリID
    pub dmypolyId_5: i32,

    /// NAME: 5: Reservation - 5:予約
    /// DESC: 5: Reservation - 5:予約
    pub reserve_5: [u8; 8],

    /// NAME: 6: SfxID - 6:SfxID
    /// DESC: 6: SfxID - 6:SfxID
    pub sfxId_6: i32,

    /// NAME: 6: Damipoli ID - 6：ダミポリID
    /// DESC: 6: Damipoli ID - 6：ダミポリID
    pub dmypolyId_6: i32,

    /// NAME: 6: Reservation - 6:予約
    /// DESC: 6: Reservation - 6:予約
    pub reserve_6: [u8; 8],

    /// NAME: 7: SfxID - 7:SfxID
    /// DESC: 7: SfxID - 7:SfxID
    pub sfxId_7: i32,

    /// NAME: 7: Damipoli ID - 7：ダミポリID
    /// DESC: 7: Damipoli ID - 7：ダミポリID
    pub dmypolyId_7: i32,

    /// NAME: Disables the effect on irradiance volume - イラディアンスボリュームへの影響を無効化
    /// DESC: When enabled, the effect on irradiance volume shooting is disabled. - 有効にするとイラディアンスボリューム撮影への影響を無効化します
    pub isDisableIV: u8,

    /// NAME: 7: Reservation - 7:予約
    /// DESC: 7: Reservation - 7:予約
    pub reserve_7: [u8; 7],
}

impl Paramdef for ASSET_MODEL_SFX_PARAM_ST {
    const NAME: &'static str = "ASSET_MODEL_SFX_PARAM_ST";
    const VERSION: u16 = 0;
}
