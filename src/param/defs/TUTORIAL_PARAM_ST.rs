/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct TUTORIAL_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Display type - 表示タイプ
    /// DESC: Specifies the type of tutorial menu to display - 表示するチュートリアルメニューの種類を指定します
    pub menuType: u8,

    /// NAME: Display timing - 表示タイミング
    /// DESC: Display timing (default: 0: "-"). You will see this tutorial when you open this menu. Specify 0: "-" if you do not want to display it when you open the menu. - 表示タイミング（デフォルト: 0:"-"）。このメニューを開いたときに、このチュートリアルを表示します。メニューを開いたときに表示しない場合は 0:"-" を指定します
    pub triggerType: u8,

    /// NAME: Impressions - 表示回数
    /// DESC: Number of times to display (default: 0: once in the game) - 表示する回数（デフォルト: 0:ゲーム中1回）
    pub repeatType: u8,

    /// NAME: Padding - パディング
    pub pad1: [u8; 1],

    /// NAME: Image ID - イメージID
    /// DESC: Specifies the tutorial image ID to display (default: 0). Specify 0 if you do not want to display the image - 表示するチュートリアル画像IDを指定します（デフォルト: 0）。画像を表示しない場合は、0 を指定します
    pub imageId: u16,

    /// NAME: Padding - パディング
    pub pad2: [u8; 2],

    /// NAME: Display permission flag - 表示許可フラグ
    /// DESC: Event flag ID for display permission (default: 0). It will not be displayed until this flag is set. Specify 0 if you always want to allow - 表示許可用のイベントフラグID（デフォルト: 0）。このフラグが立つまでは表示されません。常に許可したい場合は 0 を指定します
    pub unlockEventFlagId: u32,

    /// NAME: Text ID - テキストID
    /// DESC: ID of the text to be displayed in the tutorial [TutorialText.xlsm]. This text ID is used for both "Title" and "Body" - チュートリアルに表示するテキストのID[TutorialText.xlsm]。「タイトル」も「本文」もこのテキストIDが使われる
    pub textId: i32,

    /// NAME: at the earliest - 最短
    /// DESC: Shortest display guaranteed time [seconds]. Even if it is closed due to an event etc., it will be closed after displaying at least this time. It's only for toast, so it's ignored in modals - 最短表示保障時間[秒]。イベントなどから閉じられたとしても、少なくともこの時間は表示してから閉じられるます。トースト専用なのでモーダルでは無視されます
    pub displayMinTime: f32,

    /// NAME: Longest - 最長
    /// DESC: Display time [seconds]. It will close automatically after this time has passed since the toast was displayed. It's only for toast, so it's ignored in modals - 表示時間[秒]。トーストが表示されてからこの時間経過すると自動的に閉じられます。トースト専用なのでモーダルでは無視されます
    pub displayTime: f32,

    /// NAME: Padding - パディング
    pub pad3: [u8; 4],
}

impl Paramdef for TUTORIAL_PARAM_ST {
    const NAME: &'static str = "TUTORIAL_PARAM_ST";
    const VERSION: u16 = 1;
}
impl TUTORIAL_PARAM_ST {
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
