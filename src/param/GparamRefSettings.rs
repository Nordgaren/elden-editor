/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GPARAM_REF_SETTINGS_PARAM_ST.rs");

/// Type: GPARAM_REF_SETTINGS_PARAM_ST

pub type GparamRefSettings = ParamStruct<GPARAM_REF_SETTINGS_PARAM_ST>;
impl Param for ParamStruct<GPARAM_REF_SETTINGS_PARAM_ST> {
    const NAME: &'static str = "GparamRefSettings";
    const TYPE_NAME: &'static str = "GPARAM_REF_SETTINGS_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::GparamRefSettings::GparamRefSettings;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GparamRefSettings>(), 32)
    }
}
