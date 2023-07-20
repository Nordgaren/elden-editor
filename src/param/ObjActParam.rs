/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::OBJ_ACT_PARAM_ST::OBJ_ACT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: OBJ_ACT_PARAM_ST

pub struct ObjActParam {
    _data: OBJ_ACT_PARAM_ST,
}
impl Param for ObjActParam {
    type Def = OBJ_ACT_PARAM_ST;
    const NAME: &'static str = "ObjActParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for ObjActParam {
    type Target = OBJ_ACT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for ObjActParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ObjActParam::ObjActParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ObjActParam>(), 96)
    }
}
