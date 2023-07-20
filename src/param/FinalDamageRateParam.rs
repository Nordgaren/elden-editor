/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::FINAL_DAMAGE_RATE_PARAM_ST::FINAL_DAMAGE_RATE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: FINAL_DAMAGE_RATE_PARAM_ST

pub struct FinalDamageRateParam {
    _data: FINAL_DAMAGE_RATE_PARAM_ST,
}
impl Param for FinalDamageRateParam {
    type Def = FINAL_DAMAGE_RATE_PARAM_ST;
    const NAME: &'static str = "FinalDamageRateParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for FinalDamageRateParam {
    type Target = FINAL_DAMAGE_RATE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for FinalDamageRateParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::FinalDamageRateParam::FinalDamageRateParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<FinalDamageRateParam>(), 28)
    }
}
