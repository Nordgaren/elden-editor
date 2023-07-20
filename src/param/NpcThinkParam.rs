/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::NPC_THINK_PARAM_ST::NPC_THINK_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: NPC_THINK_PARAM_ST

pub struct NpcThinkParam {
    _data: NPC_THINK_PARAM_ST,
}
impl Param for NpcThinkParam {
    type Def = NPC_THINK_PARAM_ST;
    const NAME: &'static str = "NpcThinkParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for NpcThinkParam {
    type Target = NPC_THINK_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for NpcThinkParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::NpcThinkParam::NpcThinkParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<NpcThinkParam>(), 228)
    }
}
