/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 202
#[repr(C)]
pub struct DEFAULT_KEY_ASSIGN {
    /// NAME: Pad 0 - パッド0
    /// DESC: Lower suppression pad 0 - 下位抑制パッド0
    pub Bitfield1: u8,

    /// NAME: Pad 8 - パッド8
    /// DESC: Lower suppression pad 8 - 下位抑制パッド8
    pub Bitfield2: u8,

    /// NAME: Pad 16 - パッド16
    /// DESC: Lower suppression pad 16 - 下位抑制パッド16
    pub Bitfield3: u8,

    /// NAME: Pad 24 - パッド24
    /// DESC: Lower suppression pad 24 - 下位抑制パッド24
    pub Bitfield4: u8,

    /// NAME: dummy - ダミー
    pub dummy: [u8; 12],

    /// NAME: Pad physical key - パッド物理キー
    /// DESC: Pad physical key - パッド物理キー
    pub phyisicalKey_0: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_0: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_0: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_0: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield5: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_0: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_0: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_0: f32,

    /// NAME: Pad physical key - パッド物理キー
    /// DESC: Pad physical key - パッド物理キー
    pub phyisicalKey_1: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_1: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_1: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_1: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield6: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_1: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_1: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_1: f32,

    /// NAME: Pad physical key - パッド物理キー
    /// DESC: Pad physical key - パッド物理キー
    pub phyisicalKey_2: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_2: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_2: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_2: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield7: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_2: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_2: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_2: f32,

    /// NAME: Pad physical key - パッド物理キー
    /// DESC: Pad physical key - パッド物理キー
    pub phyisicalKey_3: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_3: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_3: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_3: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield8: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_3: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_3: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_3: f32,

    /// NAME: PC physical key - PC物理キー
    /// DESC: PC physical key - PC物理キー
    pub phyisicalKey_4: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_4: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_4: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_4: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield9: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_4: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_4: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_4: f32,

    /// NAME: PC physical key - PC物理キー
    /// DESC: PC physical key - PC物理キー
    pub phyisicalKey_5: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_5: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_5: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_5: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield10: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_5: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_5: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_5: f32,

    /// NAME: PC physical key - PC物理キー
    /// DESC: PC physical key - PC物理キー
    pub phyisicalKey_6: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_6: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_6: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_6: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield11: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_6: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_6: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_6: f32,

    /// NAME: PC physical key - PC物理キー
    /// DESC: PC physical key - PC物理キー
    pub phyisicalKey_7: i32,

    /// NAME: How to be pushed - 押され方
    /// DESC: How to be pushed - 押され方
    pub traitsType_7: u8,

    /// NAME: Analog-to-digital conversion method - アナログ→デジタル変換方法
    /// DESC: Analog-to-digital conversion method - アナログ→デジタル変換方法
    pub a2dOperator_7: u8,

    /// NAME: Applicable target - 適用ターゲット
    /// DESC: Reflection target - 反映ターゲット
    pub applyTarget_7: u8,

    /// NAME: Digital / analog - デジタル・アナログ
    /// DESC: Digital or analog - デジタルorアナログ
    pub Bitfield12: u8,

    /// NAME: time - 時間
    /// DESC: time - 時間
    pub time1_7: f32,

    /// NAME: Interval time for repeat - リピート用インターバル時間
    /// DESC: Interval time for repeat - リピート用インターバル時間
    pub time2_7: f32,

    /// NAME: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    /// DESC: Analog-to-digital conversion threshold - アナログ→デジタル変換閾値
    pub a2dThreshold_7: f32,
}

impl Paramdef for DEFAULT_KEY_ASSIGN {
    const NAME: &'static str = "DEFAULT_KEY_ASSIGN";
    const VERSION: u16 = 1;
}
impl DEFAULT_KEY_ASSIGN {
    /// Lower suppression pad 0 - 下位抑制パッド0
    /// Bitfield1
    pub fn get_priority0(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_priority0(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// Pad for lower suppression GUI framework - 下位抑制GUIフレームワーク用パッド
    /// Bitfield1
    pub fn get_priority1(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_priority1(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Lower suppression pad 2 - 下位抑制パッド2
    /// Bitfield1
    pub fn get_priority2(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_priority2(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// Lower suppression debug menu mode switching pad - 下位抑制デバッグメニューモード切替パッド
    /// Bitfield1
    pub fn get_priority3(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_priority3(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
    /// Lower suppression pad 4 - 下位抑制パッド4
    /// Bitfield1
    pub fn get_priority4(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_priority4(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xEF
        }
    }
    /// Lower suppression pad Debug menu pad - 下位抑制パッドデバッグメニューパッド
    /// Bitfield1
    pub fn get_priority5(&self) -> bool {
        &self.Bitfield1 & 0x20 != 0
    }

    /// Bitfield1
    pub fn set_priority5(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x20
        } else {
            self.Bitfield1 &= 0xDF
        }
    }
    /// Lower suppression pad 6 - 下位抑制パッド6
    /// Bitfield1
    pub fn get_priority6(&self) -> bool {
        &self.Bitfield1 & 0x40 != 0
    }

    /// Bitfield1
    pub fn set_priority6(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x40
        } else {
            self.Bitfield1 &= 0xBF
        }
    }
    /// Lower suppression pad 7 - 下位抑制パッド7
    /// Bitfield1
    pub fn get_priority7(&self) -> bool {
        &self.Bitfield1 & 0x80 != 0
    }

    /// Bitfield1
    pub fn set_priority7(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x80
        } else {
            self.Bitfield1 &= 0x7F
        }
    }
    /// Lower suppression pad 8 - 下位抑制パッド8
    /// Bitfield2
    pub fn get_priority8(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_priority8(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// Lower suppression pad 9 - 下位抑制パッド9
    /// Bitfield2
    pub fn get_priority9(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_priority9(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Lower suppression pad 10 - 下位抑制パッド10
    /// Bitfield2
    pub fn get_priority10(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_priority10(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Lower suppression pad 11 - 下位抑制パッド11
    /// Bitfield2
    pub fn get_priority11(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_priority11(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    /// Lower suppression pad 12 - 下位抑制パッド12
    /// Bitfield2
    pub fn get_priority12(&self) -> bool {
        &self.Bitfield2 & 0x10 != 0
    }

    /// Bitfield2
    pub fn set_priority12(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x10
        } else {
            self.Bitfield2 &= 0xEF
        }
    }
    /// Lower suppression pad 13 - 下位抑制パッド13
    /// Bitfield2
    pub fn get_priority13(&self) -> bool {
        &self.Bitfield2 & 0x20 != 0
    }

    /// Bitfield2
    pub fn set_priority13(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x20
        } else {
            self.Bitfield2 &= 0xDF
        }
    }
    /// Lower suppression pad 14 - 下位抑制パッド14
    /// Bitfield2
    pub fn get_priority14(&self) -> bool {
        &self.Bitfield2 & 0x40 != 0
    }

    /// Bitfield2
    pub fn set_priority14(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x40
        } else {
            self.Bitfield2 &= 0xBF
        }
    }
    /// Lower suppression pad 15 - 下位抑制パッド15
    /// Bitfield2
    pub fn get_priority15(&self) -> bool {
        &self.Bitfield2 & 0x80 != 0
    }

    /// Bitfield2
    pub fn set_priority15(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x80
        } else {
            self.Bitfield2 &= 0x7F
        }
    }
    /// Lower suppression pad 16 - 下位抑制パッド16
    /// Bitfield3
    pub fn get_priority16(&self) -> bool {
        &self.Bitfield3 & 0x1 != 0
    }

    /// Bitfield3
    pub fn set_priority16(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x1
        } else {
            self.Bitfield3 &= 0xFE
        }
    }
    /// Lower suppression pad 17 - 下位抑制パッド17
    /// Bitfield3
    pub fn get_priority17(&self) -> bool {
        &self.Bitfield3 & 0x2 != 0
    }

    /// Bitfield3
    pub fn set_priority17(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x2
        } else {
            self.Bitfield3 &= 0xFD
        }
    }
    /// Lower suppression pad 18 - 下位抑制パッド18
    /// Bitfield3
    pub fn get_priority18(&self) -> bool {
        &self.Bitfield3 & 0x4 != 0
    }

    /// Bitfield3
    pub fn set_priority18(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x4
        } else {
            self.Bitfield3 &= 0xFB
        }
    }
    /// Lower suppression pad 19 - 下位抑制パッド19
    /// Bitfield3
    pub fn get_priority19(&self) -> bool {
        &self.Bitfield3 & 0x8 != 0
    }

    /// Bitfield3
    pub fn set_priority19(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x8
        } else {
            self.Bitfield3 &= 0xF7
        }
    }
    /// Lower suppression pad 20 - 下位抑制パッド20
    /// Bitfield3
    pub fn get_priority20(&self) -> bool {
        &self.Bitfield3 & 0x10 != 0
    }

    /// Bitfield3
    pub fn set_priority20(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x10
        } else {
            self.Bitfield3 &= 0xEF
        }
    }
    /// Lower suppression pad 21 - 下位抑制パッド21
    /// Bitfield3
    pub fn get_priority21(&self) -> bool {
        &self.Bitfield3 & 0x20 != 0
    }

    /// Bitfield3
    pub fn set_priority21(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x20
        } else {
            self.Bitfield3 &= 0xDF
        }
    }
    /// Lower suppression pad 22 - 下位抑制パッド22
    /// Bitfield3
    pub fn get_priority22(&self) -> bool {
        &self.Bitfield3 & 0x40 != 0
    }

    /// Bitfield3
    pub fn set_priority22(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x40
        } else {
            self.Bitfield3 &= 0xBF
        }
    }
    /// Lower suppression pad 23 - 下位抑制パッド23
    /// Bitfield3
    pub fn get_priority23(&self) -> bool {
        &self.Bitfield3 & 0x80 != 0
    }

    /// Bitfield3
    pub fn set_priority23(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x80
        } else {
            self.Bitfield3 &= 0x7F
        }
    }
    /// Lower suppression pad 24 - 下位抑制パッド24
    /// Bitfield4
    pub fn get_priority24(&self) -> bool {
        &self.Bitfield4 & 0x1 != 0
    }

    /// Bitfield4
    pub fn set_priority24(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x1
        } else {
            self.Bitfield4 &= 0xFE
        }
    }
    /// Lower suppression pad 25 - 下位抑制パッド25
    /// Bitfield4
    pub fn get_priority25(&self) -> bool {
        &self.Bitfield4 & 0x2 != 0
    }

    /// Bitfield4
    pub fn set_priority25(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x2
        } else {
            self.Bitfield4 &= 0xFD
        }
    }
    /// Lower suppression pad 26 - 下位抑制パッド26
    /// Bitfield4
    pub fn get_priority26(&self) -> bool {
        &self.Bitfield4 & 0x4 != 0
    }

    /// Bitfield4
    pub fn set_priority26(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x4
        } else {
            self.Bitfield4 &= 0xFB
        }
    }
    /// Lower suppression pad 27 - 下位抑制パッド27
    /// Bitfield4
    pub fn get_priority27(&self) -> bool {
        &self.Bitfield4 & 0x8 != 0
    }

    /// Bitfield4
    pub fn set_priority27(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x8
        } else {
            self.Bitfield4 &= 0xF7
        }
    }
    /// Lower suppression pad 28 - 下位抑制パッド28
    /// Bitfield4
    pub fn get_priority28(&self) -> bool {
        &self.Bitfield4 & 0x10 != 0
    }

    /// Bitfield4
    pub fn set_priority28(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x10
        } else {
            self.Bitfield4 &= 0xEF
        }
    }
    /// Lower suppression pad 29 - 下位抑制パッド29
    /// Bitfield4
    pub fn get_priority29(&self) -> bool {
        &self.Bitfield4 & 0x20 != 0
    }

    /// Bitfield4
    pub fn set_priority29(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x20
        } else {
            self.Bitfield4 &= 0xDF
        }
    }
    /// Lower suppression pad 30 - 下位抑制パッド30
    /// Bitfield4
    pub fn get_priority30(&self) -> bool {
        &self.Bitfield4 & 0x40 != 0
    }

    /// Bitfield4
    pub fn set_priority30(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x40
        } else {
            self.Bitfield4 &= 0xBF
        }
    }
    /// Lower suppression pad 31 - 下位抑制パッド31
    /// Bitfield4
    pub fn get_priority31(&self) -> bool {
        &self.Bitfield4 & 0x80 != 0
    }

    /// Bitfield4
    pub fn set_priority31(&mut self, state: bool) {
        if state {
            self.Bitfield4 |= 0x80
        } else {
            self.Bitfield4 &= 0x7F
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield5
    pub fn get_isAnalog_0(&self) -> bool {
        &self.Bitfield5 & 0x1 != 0
    }

    /// Bitfield5
    pub fn set_isAnalog_0(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x1
        } else {
            self.Bitfield5 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield5
    pub fn get_enableWin64_0(&self) -> bool {
        &self.Bitfield5 & 0x2 != 0
    }

    /// Bitfield5
    pub fn set_enableWin64_0(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x2
        } else {
            self.Bitfield5 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield5
    pub fn get_enablePS4_0(&self) -> bool {
        &self.Bitfield5 & 0x4 != 0
    }

    /// Bitfield5
    pub fn set_enablePS4_0(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x4
        } else {
            self.Bitfield5 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield5
    pub fn get_enableXboxOne_0(&self) -> bool {
        &self.Bitfield5 & 0x8 != 0
    }

    /// Bitfield5
    pub fn set_enableXboxOne_0(&mut self, state: bool) {
        if state {
            self.Bitfield5 |= 0x8
        } else {
            self.Bitfield5 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield6
    pub fn get_isAnalog_1(&self) -> bool {
        &self.Bitfield6 & 0x1 != 0
    }

    /// Bitfield6
    pub fn set_isAnalog_1(&mut self, state: bool) {
        if state {
            self.Bitfield6 |= 0x1
        } else {
            self.Bitfield6 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield6
    pub fn get_enableWin64_1(&self) -> bool {
        &self.Bitfield6 & 0x2 != 0
    }

    /// Bitfield6
    pub fn set_enableWin64_1(&mut self, state: bool) {
        if state {
            self.Bitfield6 |= 0x2
        } else {
            self.Bitfield6 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield6
    pub fn get_enablePS4_1(&self) -> bool {
        &self.Bitfield6 & 0x4 != 0
    }

    /// Bitfield6
    pub fn set_enablePS4_1(&mut self, state: bool) {
        if state {
            self.Bitfield6 |= 0x4
        } else {
            self.Bitfield6 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield6
    pub fn get_enableXboxOne_1(&self) -> bool {
        &self.Bitfield6 & 0x8 != 0
    }

    /// Bitfield6
    pub fn set_enableXboxOne_1(&mut self, state: bool) {
        if state {
            self.Bitfield6 |= 0x8
        } else {
            self.Bitfield6 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield7
    pub fn get_isAnalog_2(&self) -> bool {
        &self.Bitfield7 & 0x1 != 0
    }

    /// Bitfield7
    pub fn set_isAnalog_2(&mut self, state: bool) {
        if state {
            self.Bitfield7 |= 0x1
        } else {
            self.Bitfield7 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield7
    pub fn get_enableWin64_2(&self) -> bool {
        &self.Bitfield7 & 0x2 != 0
    }

    /// Bitfield7
    pub fn set_enableWin64_2(&mut self, state: bool) {
        if state {
            self.Bitfield7 |= 0x2
        } else {
            self.Bitfield7 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield7
    pub fn get_enablePS4_2(&self) -> bool {
        &self.Bitfield7 & 0x4 != 0
    }

    /// Bitfield7
    pub fn set_enablePS4_2(&mut self, state: bool) {
        if state {
            self.Bitfield7 |= 0x4
        } else {
            self.Bitfield7 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield7
    pub fn get_enableXboxOne_2(&self) -> bool {
        &self.Bitfield7 & 0x8 != 0
    }

    /// Bitfield7
    pub fn set_enableXboxOne_2(&mut self, state: bool) {
        if state {
            self.Bitfield7 |= 0x8
        } else {
            self.Bitfield7 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield8
    pub fn get_isAnalog_3(&self) -> bool {
        &self.Bitfield8 & 0x1 != 0
    }

    /// Bitfield8
    pub fn set_isAnalog_3(&mut self, state: bool) {
        if state {
            self.Bitfield8 |= 0x1
        } else {
            self.Bitfield8 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield8
    pub fn get_enableWin64_3(&self) -> bool {
        &self.Bitfield8 & 0x2 != 0
    }

    /// Bitfield8
    pub fn set_enableWin64_3(&mut self, state: bool) {
        if state {
            self.Bitfield8 |= 0x2
        } else {
            self.Bitfield8 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield8
    pub fn get_enablePS4_3(&self) -> bool {
        &self.Bitfield8 & 0x4 != 0
    }

    /// Bitfield8
    pub fn set_enablePS4_3(&mut self, state: bool) {
        if state {
            self.Bitfield8 |= 0x4
        } else {
            self.Bitfield8 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield8
    pub fn get_enableXboxOne_3(&self) -> bool {
        &self.Bitfield8 & 0x8 != 0
    }

    /// Bitfield8
    pub fn set_enableXboxOne_3(&mut self, state: bool) {
        if state {
            self.Bitfield8 |= 0x8
        } else {
            self.Bitfield8 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield9
    pub fn get_isAnalog_4(&self) -> bool {
        &self.Bitfield9 & 0x1 != 0
    }

    /// Bitfield9
    pub fn set_isAnalog_4(&mut self, state: bool) {
        if state {
            self.Bitfield9 |= 0x1
        } else {
            self.Bitfield9 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield9
    pub fn get_enableWin64_4(&self) -> bool {
        &self.Bitfield9 & 0x2 != 0
    }

    /// Bitfield9
    pub fn set_enableWin64_4(&mut self, state: bool) {
        if state {
            self.Bitfield9 |= 0x2
        } else {
            self.Bitfield9 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield9
    pub fn get_enablePS4_4(&self) -> bool {
        &self.Bitfield9 & 0x4 != 0
    }

    /// Bitfield9
    pub fn set_enablePS4_4(&mut self, state: bool) {
        if state {
            self.Bitfield9 |= 0x4
        } else {
            self.Bitfield9 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield9
    pub fn get_enableXboxOne_4(&self) -> bool {
        &self.Bitfield9 & 0x8 != 0
    }

    /// Bitfield9
    pub fn set_enableXboxOne_4(&mut self, state: bool) {
        if state {
            self.Bitfield9 |= 0x8
        } else {
            self.Bitfield9 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield10
    pub fn get_isAnalog_5(&self) -> bool {
        &self.Bitfield10 & 0x1 != 0
    }

    /// Bitfield10
    pub fn set_isAnalog_5(&mut self, state: bool) {
        if state {
            self.Bitfield10 |= 0x1
        } else {
            self.Bitfield10 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield10
    pub fn get_enableWin64_5(&self) -> bool {
        &self.Bitfield10 & 0x2 != 0
    }

    /// Bitfield10
    pub fn set_enableWin64_5(&mut self, state: bool) {
        if state {
            self.Bitfield10 |= 0x2
        } else {
            self.Bitfield10 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield10
    pub fn get_enablePS4_5(&self) -> bool {
        &self.Bitfield10 & 0x4 != 0
    }

    /// Bitfield10
    pub fn set_enablePS4_5(&mut self, state: bool) {
        if state {
            self.Bitfield10 |= 0x4
        } else {
            self.Bitfield10 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield10
    pub fn get_enableXboxOne_5(&self) -> bool {
        &self.Bitfield10 & 0x8 != 0
    }

    /// Bitfield10
    pub fn set_enableXboxOne_5(&mut self, state: bool) {
        if state {
            self.Bitfield10 |= 0x8
        } else {
            self.Bitfield10 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield11
    pub fn get_isAnalog_6(&self) -> bool {
        &self.Bitfield11 & 0x1 != 0
    }

    /// Bitfield11
    pub fn set_isAnalog_6(&mut self, state: bool) {
        if state {
            self.Bitfield11 |= 0x1
        } else {
            self.Bitfield11 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield11
    pub fn get_enableWin64_6(&self) -> bool {
        &self.Bitfield11 & 0x2 != 0
    }

    /// Bitfield11
    pub fn set_enableWin64_6(&mut self, state: bool) {
        if state {
            self.Bitfield11 |= 0x2
        } else {
            self.Bitfield11 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield11
    pub fn get_enablePS4_6(&self) -> bool {
        &self.Bitfield11 & 0x4 != 0
    }

    /// Bitfield11
    pub fn set_enablePS4_6(&mut self, state: bool) {
        if state {
            self.Bitfield11 |= 0x4
        } else {
            self.Bitfield11 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield11
    pub fn get_enableXboxOne_6(&self) -> bool {
        &self.Bitfield11 & 0x8 != 0
    }

    /// Bitfield11
    pub fn set_enableXboxOne_6(&mut self, state: bool) {
        if state {
            self.Bitfield11 |= 0x8
        } else {
            self.Bitfield11 &= 0xF7
        }
    }
    /// Digital or analog - デジタルorアナログ
    /// Bitfield12
    pub fn get_isAnalog_7(&self) -> bool {
        &self.Bitfield12 & 0x1 != 0
    }

    /// Bitfield12
    pub fn set_isAnalog_7(&mut self, state: bool) {
        if state {
            self.Bitfield12 |= 0x1
        } else {
            self.Bitfield12 &= 0xFE
        }
    }
    /// Will it be used in Win64? - Win64で使用されるか
    /// Bitfield12
    pub fn get_enableWin64_7(&self) -> bool {
        &self.Bitfield12 & 0x2 != 0
    }

    /// Bitfield12
    pub fn set_enableWin64_7(&mut self, state: bool) {
        if state {
            self.Bitfield12 |= 0x2
        } else {
            self.Bitfield12 &= 0xFD
        }
    }
    /// Will it be used on PS4 - PS4で使用されるか
    /// Bitfield12
    pub fn get_enablePS4_7(&self) -> bool {
        &self.Bitfield12 & 0x4 != 0
    }

    /// Bitfield12
    pub fn set_enablePS4_7(&mut self, state: bool) {
        if state {
            self.Bitfield12 |= 0x4
        } else {
            self.Bitfield12 &= 0xFB
        }
    }
    /// Will it be used on Xbox One - XboxOneで使用されるか
    /// Bitfield12
    pub fn get_enableXboxOne_7(&self) -> bool {
        &self.Bitfield12 & 0x8 != 0
    }

    /// Bitfield12
    pub fn set_enableXboxOne_7(&mut self, state: bool) {
        if state {
            self.Bitfield12 |= 0x8
        } else {
            self.Bitfield12 &= 0xF7
        }
    }
}
