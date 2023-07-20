/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::ATTACK_ELEMENT_CORRECT_PARAM_ST::ATTACK_ELEMENT_CORRECT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ATTACK_ELEMENT_CORRECT_PARAM_ST

pub struct AttackElementCorrectParam {
    _data: ATTACK_ELEMENT_CORRECT_PARAM_ST,
}
impl Param for AttackElementCorrectParam {
    type Def = ATTACK_ELEMENT_CORRECT_PARAM_ST;
    const NAME: &'static str = "AttackElementCorrectParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for AttackElementCorrectParam {
    type Target = ATTACK_ELEMENT_CORRECT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for AttackElementCorrectParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::AttackElementCorrectParam::AttackElementCorrectParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AttackElementCorrectParam>(), 128)
    }
}
