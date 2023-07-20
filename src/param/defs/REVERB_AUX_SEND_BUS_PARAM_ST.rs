/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 202
#[repr(C)]
pub struct REVERB_AUX_SEND_BUS_PARAM_ST {
    /// NAME: ReverbAuxSendBus name - ReverbAuxSendBus名
    /// DESC: ReverbAuxSendBus name - ReverbAuxSendBus名
    pub ReverbAuxSendBusName: [u8; 32],
}

impl Paramdef for REVERB_AUX_SEND_BUS_PARAM_ST {
    const NAME: &'static str = "REVERB_AUX_SEND_BUS_PARAM_ST";
    const VERSION: u16 = 1;
}
