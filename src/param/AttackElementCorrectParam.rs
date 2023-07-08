/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ATTACK_ELEMENT_CORRECT_PARAM_ST.rs");

/// Type: ATTACK_ELEMENT_CORRECT_PARAM_ST

pub type AttackElementCorrectParam = ParamStruct<ATTACK_ELEMENT_CORRECT_PARAM_ST>;
impl Param for ParamStruct<ATTACK_ELEMENT_CORRECT_PARAM_ST> {
    const NAME: &'static str = "AttackElementCorrectParam";
    const TYPE_NAME: &'static str = "ATTACK_ELEMENT_CORRECT_PARAM_ST";
    const VERSION: u16 = 2;
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
