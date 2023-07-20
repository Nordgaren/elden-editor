/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::GRAPHICS_COMMON_PARAM_ST::GRAPHICS_COMMON_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: GRAPHICS_COMMON_PARAM_ST

pub struct GraphicsCommonParam {
    _data: GRAPHICS_COMMON_PARAM_ST,
}
impl Param for GraphicsCommonParam {
    type Def = GRAPHICS_COMMON_PARAM_ST;
    const NAME: &'static str = "GraphicsCommonParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for GraphicsCommonParam {
    type Target = GRAPHICS_COMMON_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for GraphicsCommonParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::GraphicsCommonParam::GraphicsCommonParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GraphicsCommonParam>(), 256)
    }
}
