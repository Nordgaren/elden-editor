/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::ROLLING_OBJ_LOT_PARAM_ST::ROLLING_OBJ_LOT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ROLLING_OBJ_LOT_PARAM_ST

pub struct RollingObjLotParam {
    _data: ROLLING_OBJ_LOT_PARAM_ST,
}
impl Param for RollingObjLotParam {
    type Def = ROLLING_OBJ_LOT_PARAM_ST;
    const NAME: &'static str = "RollingObjLotParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for RollingObjLotParam {
    type Target = ROLLING_OBJ_LOT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for RollingObjLotParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::RollingObjLotParam::RollingObjLotParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RollingObjLotParam>(), 64)
    }
}
