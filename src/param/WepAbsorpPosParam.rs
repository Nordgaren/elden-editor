/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::WEP_ABSORP_POS_PARAM_ST::WEP_ABSORP_POS_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: WEP_ABSORP_POS_PARAM_ST

pub struct WepAbsorpPosParam {
    _data: WEP_ABSORP_POS_PARAM_ST,
}
impl Param for WepAbsorpPosParam {
    type Def = WEP_ABSORP_POS_PARAM_ST;
    const NAME: &'static str = "WepAbsorpPosParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for WepAbsorpPosParam {
    type Target = WEP_ABSORP_POS_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for WepAbsorpPosParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::WepAbsorpPosParam::WepAbsorpPosParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WepAbsorpPosParam>(), 96)
    }
}
