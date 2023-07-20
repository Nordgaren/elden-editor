/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct COOL_TIME_PARAM_ST {
    /// NAME: Time limit (0 cooperating spirits) - 制限時間（協力霊数0）
    /// DESC: Time limit [sec] (number of cooperating spirits 0) - 制限時間[sec]（協力霊数0）
    pub limitationTime_0: f32,

    /// NAME: Monitoring time (0 number of cooperating spirits) - 監視時間（協力霊数0）
    /// DESC: Monitoring time [sec] (number of cooperating spirits 0) - 監視時間[sec]（協力霊数0）
    pub observeTime_0: f32,

    /// NAME: Time limit (number of cooperating spirits 1) - 制限時間（協力霊数1）
    /// DESC: Time limit [sec] (number of cooperating spirits 1) - 制限時間[sec]（協力霊数1）
    pub limitationTime_1: f32,

    /// NAME: Monitoring time (number of cooperating spirits 1) - 監視時間（協力霊数1）
    /// DESC: Monitoring time [sec] (number of cooperating spirits 1) - 監視時間[sec]（協力霊数1）
    pub observeTime_1: f32,

    /// NAME: Time limit (2 cooperating spirits) - 制限時間（協力霊数2）
    /// DESC: Time limit [sec] (number of cooperating spirits 2) - 制限時間[sec]（協力霊数2）
    pub limitationTime_2: f32,

    /// NAME: Monitoring time (2 cooperating spirits) - 監視時間（協力霊数2）
    /// DESC: Monitoring time [sec] (number of cooperating spirits 2) - 監視時間[sec]（協力霊数2）
    pub observeTime_2: f32,

    /// NAME: Time limit (3 cooperating spirits) - 制限時間（協力霊数3）
    /// DESC: Time limit [sec] (number of cooperating spirits 3) - 制限時間[sec]（協力霊数3）
    pub limitationTime_3: f32,

    /// NAME: Monitoring time (3 cooperating spirits) - 監視時間（協力霊数3）
    /// DESC: Monitoring time [sec] (number of cooperating spirits 3) - 監視時間[sec]（協力霊数3）
    pub observeTime_3: f32,
}

impl Paramdef for COOL_TIME_PARAM_ST {
    const NAME: &'static str = "COOL_TIME_PARAM_ST";
    const VERSION: u16 = 1;
}
