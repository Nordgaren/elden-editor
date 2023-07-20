/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CS_GRAPHICS_CONFIG_PARAM_ST::CS_GRAPHICS_CONFIG_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CS_GRAPHICS_CONFIG_PARAM_ST

pub struct GraphicsConfig {
    _data: CS_GRAPHICS_CONFIG_PARAM_ST,
}
impl Param for GraphicsConfig {
    type Def = CS_GRAPHICS_CONFIG_PARAM_ST;
    const NAME: &'static str = "GraphicsConfig";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for GraphicsConfig {
    type Target = CS_GRAPHICS_CONFIG_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for GraphicsConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::GraphicsConfig::GraphicsConfig;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GraphicsConfig>(), 16)
    }
}
