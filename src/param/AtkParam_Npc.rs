/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::defs::ATK_PARAM_ST::ATK_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ATK_PARAM_ST

pub struct AtkParam_Npc {
    _data: ATK_PARAM_ST,
}
impl Param for AtkParam_Npc {
    type Def = ATK_PARAM_ST;
    const NAME: &'static str = "AtkParam_Npc";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for AtkParam_Npc {
    type Target = ATK_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for AtkParam_Npc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::AtkParam_Npc::AtkParam_Npc;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AtkParam_Npc>(), 456)
    }
}
