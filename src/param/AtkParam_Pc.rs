/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::defs::ATK_PARAM_ST::ATK_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ATK_PARAM_ST

pub struct AtkParam_Pc {
    _data: ATK_PARAM_ST,
}
impl Param for AtkParam_Pc {
    type Def = ATK_PARAM_ST;
    const NAME: &'static str = "AtkParam_Pc";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for AtkParam_Pc {
    type Target = ATK_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for AtkParam_Pc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::AtkParam_Pc::AtkParam_Pc;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AtkParam_Pc>(), 456)
    }
}
