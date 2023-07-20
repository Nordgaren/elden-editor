/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CHR_ACTIVATE_CONDITION_PARAM_ST::CHR_ACTIVATE_CONDITION_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CHR_ACTIVATE_CONDITION_PARAM_ST

pub struct ChrActivateConditionParam {
    _data: CHR_ACTIVATE_CONDITION_PARAM_ST,
}
impl Param for ChrActivateConditionParam {
    type Def = CHR_ACTIVATE_CONDITION_PARAM_ST;
    const NAME: &'static str = "ChrActivateConditionParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for ChrActivateConditionParam {
    type Target = CHR_ACTIVATE_CONDITION_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for ChrActivateConditionParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ChrActivateConditionParam::ChrActivateConditionParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ChrActivateConditionParam>(), 8)
    }
}
