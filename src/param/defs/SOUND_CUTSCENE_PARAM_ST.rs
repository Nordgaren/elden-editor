/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 5
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SOUND_CUTSCENE_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Reverb type in cutscenes - カットシーン中のリバーブタイプ
    /// DESC: Specifies the reverb type to apply during the cutscene - カットシーン中に適応するリバーブタイプを指定します
    pub ReverbType: u8,

    /// NAME: pad0 - pad0
    pub pad0: [u8; 3],

    /// NAME: Normal BGM behavior at the start of cutscenes - カットシーン開始時通常BGM挙動
    /// DESC: Specifies normal BGM behavior at the start of cutscenes - カットシーン開始時通常BGM挙動を指定します
    pub BgmBehaviorTypeOnStart: i16,

    /// NAME: One-shot BGM behavior at the start of the cutscene - カットシーン開始時ワンショットBGM挙動
    /// DESC: Specifies the one-shot BGM behavior at the start of the cutscene - カットシーン開始時ワンショットBGM挙動を指定します
    pub OneShotBgmBehaviorOnStart: i16,

    /// NAME: SEID (category: p) specification to post at the end of the cutscene (-1: do not post) - カットシーン終了時にポストするSEID（カテゴリ：p)指定(-1:ポストしない)
    /// DESC: SEID (category: p) specification to post at the end of the cutscene (-1: do not post) - カットシーン終了時にポストするSEID（カテゴリ：p)指定(-1:ポストしない)
    pub PostPlaySeId: i32,

    /// NAME: Post at the end of the cutscene SEID_Skip (category: p) specified (-1: do not post) - カットシーン終了時にポストするSEID_スキップ時（カテゴリ：p)指定(-1:ポストしない)
    /// DESC: SEID_ for skipping to post at the end of the cutscene (category: p) specified (-1: not posted) - カットシーン終了時にポストするSEID_スキップ時用（カテゴリ：p)指定(-1:ポストしない)
    pub PostPlaySeIdForSkip: i32,

    /// NAME: Cutscene drawing time to unmute immediately after entering [seconds] (less than 0: not canceled in drawing time) - 入場直後ミュート解除するカットシーン描画時間[秒](0より小さい：描画時間で解除しない)
    /// DESC: Cutscene drawing time to unmute immediately after entering [seconds] (less than 0: not canceled in drawing time) - 入場直後のミュート解除するカットシーン描画時間[秒](0より小さい：描画時間で解除しない)
    pub EnterMapMuteStopTimeOnDrawCutscene: f32,

    /// NAME: Reserve - リザーブ
    /// DESC: Reserve - リザーブ
    pub reserved: [u8; 12],
}

impl Paramdef for SOUND_CUTSCENE_PARAM_ST {
    const NAME: &'static str = "SOUND_CUTSCENE_PARAM_ST";
    const VERSION: u16 = 5;
}
impl SOUND_CUTSCENE_PARAM_ST {
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
