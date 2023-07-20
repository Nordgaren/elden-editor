/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::defs::WWISE_VALUE_TO_STR_CONVERT_PARAM_ST::WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST

pub struct WwiseValueToStrParam_Switch_PlayerShoes {
    _data: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
}
impl Param for WwiseValueToStrParam_Switch_PlayerShoes {
    type Def = WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;
    const NAME: &'static str = "WwiseValueToStrParam_Switch_PlayerShoes";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for WwiseValueToStrParam_Switch_PlayerShoes {
    type Target = WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for WwiseValueToStrParam_Switch_PlayerShoes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::WwiseValueToStrParam_Switch_PlayerShoes::WwiseValueToStrParam_Switch_PlayerShoes;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WwiseValueToStrParam_Switch_PlayerShoes>(), 36)
    }
}
