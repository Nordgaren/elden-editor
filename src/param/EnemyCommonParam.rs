/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ENEMY_COMMON_PARAM_ST.rs");

/// Type: ENEMY_COMMON_PARAM_ST

pub type EnemyCommonParam = ParamStruct<ENEMY_COMMON_PARAM_ST>;
impl Param for ParamStruct<ENEMY_COMMON_PARAM_ST> {
    const NAME: &'static str = "EnemyCommonParam";
    const TYPE_NAME: &'static str = "ENEMY_COMMON_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::EnemyCommonParam::EnemyCommonParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<EnemyCommonParam>(), 256)
    }
}
