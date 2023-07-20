/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CUTSCENE_MAP_ID_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Map number to play - 再生を行なうマップ番号
    /// DESC: Please enter the map number to be played back as an 8-digit number. This is the map number used as the reference in the cutscene. If you do not specify the correct map number, the playback position will shift. - 再生を行なうマップ番号を8桁の数字で入力して下さい。カットシーンで基準としているマップの番号になります。正しいマップ番号を指定しないと再生位置がずれます。
    pub PlayMapId: u32,

    /// NAME: Map number 1 required for display - 表示に必要なマップ番号１
    /// DESC: Please enter the map number required for display as an 8-digit number. This parameter is used by the guest as a list to determine if the cutscene can be played. If you don't need it, you can leave it as 0 or blank. - 表示に必要なマップ番号を8桁の数字で入力して下さい。このパラメータは、ゲスト側がカットシーンを再生可能か判断するためのリストとして利用します。必要ない場合は、0もしくは未記入で良いです。
    pub RequireMapId0: u32,

    /// NAME: Map number 2 required for display - 表示に必要なマップ番号２
    /// DESC: Please enter the map number required for display as an 8-digit number. This parameter is used by the guest as a list to determine if the cutscene can be played. If you don't need it, you can leave it as 0 or blank. - 表示に必要なマップ番号を8桁の数字で入力して下さい。このパラメータは、ゲスト側がカットシーンを再生可能か判断するためのリストとして利用します。必要ない場合は、0もしくは未記入で良いです。
    pub RequireMapId1: u32,

    /// NAME: Map number 3 required for display - 表示に必要なマップ番号３
    /// DESC: Please enter the map number required for display as an 8-digit number. This parameter is used by the guest as a list to determine if the cutscene can be played. If you don't need it, you can leave it as 0 or blank. - 表示に必要なマップ番号を8桁の数字で入力して下さい。このパラメータは、ゲスト側がカットシーンを再生可能か判断するためのリストとして利用します。必要ない場合は、0もしくは未記入で良いです。
    pub RequireMapId2: u32,

    /// NAME: Hit part ID for calculating camera position during loading - 読み込み中カメラ位置算出用ヒットパーツID
    /// DESC: Hit part ID for calculating camera position during loading - 読み込み中カメラ位置算出用ヒットパーツID
    pub RefCamPosHitPartsID: i32,

    /// NAME: Reserve - 予備
    pub reserved_2: [u8; 12],

    /// NAME: Waiting time when cannot be displayed [seconds] - 表示不可能時待機時間[秒]
    /// DESC: The number of seconds used to display the progress of the loading screen progress bar that is displayed when the guest side cannot play in the multi. [GR] SEQ22843 [Cutscene] Players who have not read the map number required for display during cutscene playback will see the screen go dark. - マルチにおいてゲスト側が再生できない状態の時に表示されるロード画面プログレスバーの進捗表示に使われる秒数です。【GR】SEQ22843 【カットシーン】カットシーン再生時に表示に必要なマップ番号を読み込んでいないプレイヤーは画面暗転する対応
    pub ClientDisableViewTimeForProgress: u16,

    /// NAME: reserved - reserved
    /// DESC: reserved - reserved
    pub reserved: [u8; 2],

    /// NAME: Hit parts waiting to be read 0 - 読み込み待ちヒットパーツ0
    /// DESC: Hit parts waiting to be read 0 - 読み込み待ちヒットパーツ0
    pub HitParts_0: i32,

    /// NAME: Hit parts waiting to be read 1 - 読み込み待ちヒットパーツ1
    /// DESC: Hit parts waiting to be read 1 - 読み込み待ちヒットパーツ1
    pub HitParts_1: i32,
}

impl Paramdef for CUTSCENE_MAP_ID_PARAM_ST {
    const NAME: &'static str = "CUTSCENE_MAP_ID_PARAM_ST";
    const VERSION: u16 = 2;
}
impl CUTSCENE_MAP_ID_PARAM_ST {
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
    /// Parameters marked with a circle are excluded from all packages (because they are for debugging). - ○をつけたパラメータは全パッケージから除外します（デバッグ用なので）
    /// Bitfield1
    pub fn get_disableParam_Debug(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_disableParam_Debug(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Reserve for package output 1 - パッケージ出力用リザーブ1
    /// Bitfield1
    pub fn get_disableParamReserve1(&self) -> u8 {
        &self.Bitfield1 & 0xFC
    }

    /// Bitfield1 MAX: 63
    pub fn set_disableParamReserve1(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 2) & 0xFC;
            let newVal = &self.Bitfield1 & 0x3 | val;
            self.Bitfield1 = newVal
        } else {
            self.Bitfield1 &= 0x3
        }
    }
}
