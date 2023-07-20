/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SWORD_ARTS_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Swashbuckler ID - 剣戟ID
    /// DESC: To pass to the behavior script to determine which swashbuckler - ビヘイビアスクリプトに渡してどの剣戟か判定するためのもの
    pub swordArtsType: u8,

    /// NAME: Arts speed - アーツ速度
    /// DESC: Which cancellation timing do you see? 0: Normal (left hand attack) / 1: Early / 2: Slow - どのキャンセルタイミングを見るか。0：通常（左手攻撃）／1：早い／2：遅い
    pub artsSpeedType: u8,

    /// NAME: Related status - 関連ステータス
    /// DESC: Which system of arts points to refer to - どの系統のアーツポイントを参照するか
    pub refStatus: i8,

    /// NAME: Whether to display the arts of the right hand when the left hand (holding one hand) - 左手（片手持ち）時に右手のアーツを表示するか
    /// DESC: When set to the arts of the left-handed weapon, the arts of the right-handed weapon are displayed in FE. Assumed to be used for "weapon maneuvers" etc. - 左手武器のアーツに設定されている場合、右手武器のアーツをFEに表示します。「武器戦技」などに使われる想定
    pub Bitfield2: u8,

    /// NAME: Consumption point L1 - 消費ポイント L1
    /// DESC: Points to spend when putting out arts by L1 - L1によりアーツを出したときに消費するポイント
    pub usePoint_L1: i8,

    /// NAME: Consumption point L2 - 消費ポイント L2
    /// DESC: Points to spend when putting out arts by L2 - L2によりアーツを出したときに消費するポイント
    pub usePoint_L2: i8,

    /// NAME: Consumption point R1 - 消費ポイント R1
    /// DESC: Points to spend when putting out arts by R1 - R1によりアーツを出したときに消費するポイント
    pub usePoint_R1: i8,

    /// NAME: Consumption point R2 - 消費ポイント R2
    /// DESC: Points to spend when putting out arts by R2 - R2によりアーツを出したときに消費するポイント
    pub usePoint_R2: i8,

    /// NAME: Text ID - テキストID
    /// DESC: Text ID for arts description - アーツ説明用のテキストID
    pub textId: i32,

    /// NAME: MP L1 consumed - 消費MP L1
    /// DESC: MP consumed when issuing arts by L1 - L1によりアーツを出したときに消費するMP
    pub useMagicPoint_L1: i16,

    /// NAME: MP L2 consumed - 消費MP L2
    /// DESC: MP consumed when issuing arts by L2 - L2によりアーツを出したときに消費するMP
    pub useMagicPoint_L2: i16,

    /// NAME: MP R1 consumed - 消費MP R1
    /// DESC: MP consumed when issuing arts by R1 - R1によりアーツを出したときに消費するMP
    pub useMagicPoint_R1: i16,

    /// NAME: MP R2 consumed - 消費MP R2
    /// DESC: MP consumed when issuing arts by R2 - R2によりアーツを出したときに消費するMP
    pub useMagicPoint_R2: i16,

    /// NAME: Shield type icon (overwrite) - 盾種別アイコン（上書き）
    /// DESC: If you do not overwrite it, the icon will be displayed based on the swashbuckler ID of the weapon para. - 上書きしない場合は、武器パラの剣戟IDを元にアイコン表示されます。
    pub shieldIconType: i8,

    /// NAME: pad - pad
    pub pad: [u8; 1],

    /// NAME: Icon ID - アイコンID
    /// DESC: ID of the icon displayed in FE etc. - FEなどで表示するアイコンのID
    pub iconId: u16,

    /// NAME: AI usage judgment ID - AI使用判断ID
    /// DESC: AI usage judgment ID - AI使用判断ID
    pub aiUsageId: i32,
}

impl Paramdef for SWORD_ARTS_PARAM_ST {
    const NAME: &'static str = "SWORD_ARTS_PARAM_ST";
    const VERSION: u16 = 3;
}
impl SWORD_ARTS_PARAM_ST {
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
    /// When set to the arts of the left-handed weapon, the arts of the right-handed weapon are displayed in FE. Assumed to be used for "weapon maneuvers" etc. - 左手武器のアーツに設定されている場合、右手武器のアーツをFEに表示します。「武器戦技」などに使われる想定
    /// Bitfield2
    pub fn get_isRefRightArts(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_isRefRightArts(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Whether to gray out when displaying the arts name of the left hand (one-handed) - 左手（片手持ち）のアーツ名を表示するときにグレーアウトするか
    /// Bitfield2
    pub fn get_isGrayoutLeftHand(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_isGrayoutLeftHand(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Whether to gray out when displaying the arts name of the right hand (one-handed) - 右手（片手持ち）のアーツ名を表示するときにグレーアウトするか
    /// Bitfield2
    pub fn get_isGrayoutRightHand(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_isGrayoutRightHand(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Whether to gray out when displaying the arts name of both hands - 両手持ちのアーツ名を表示するときにグレーアウトするか
    /// Bitfield2
    pub fn get_isGrayoutBothHand(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_isGrayoutBothHand(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    ///
    /// Bitfield2
    pub fn get_reserve2(&self) -> u8 {
        &self.Bitfield2 & 0xF0
    }

    /// Bitfield2 MAX: 15
    pub fn set_reserve2(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 4) & 0xF0;
            let newVal = &self.Bitfield2 & 0xF | val;
            self.Bitfield2 = newVal
        } else {
            self.Bitfield2 &= 0xF
        }
    }
}
