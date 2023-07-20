/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::RUNTIME_BONE_CONTROL_PARAM_ST::RUNTIME_BONE_CONTROL_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: RUNTIME_BONE_CONTROL_PARAM_ST

pub struct RuntimeBoneControlParam {
    _data: RUNTIME_BONE_CONTROL_PARAM_ST,
}
impl Param for RuntimeBoneControlParam {
    type Def = RUNTIME_BONE_CONTROL_PARAM_ST;
    const NAME: &'static str = "RuntimeBoneControlParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for RuntimeBoneControlParam {
    type Target = RUNTIME_BONE_CONTROL_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for RuntimeBoneControlParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::RuntimeBoneControlParam::RuntimeBoneControlParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RuntimeBoneControlParam>(), 112)
    }
}
