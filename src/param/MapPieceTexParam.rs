/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MAP_PIECE_TEX_PARAM_ST::MAP_PIECE_TEX_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MAP_PIECE_TEX_PARAM_ST

pub struct MapPieceTexParam {
    _data: MAP_PIECE_TEX_PARAM_ST,
}
impl Param for MapPieceTexParam {
    type Def = MAP_PIECE_TEX_PARAM_ST;
    const NAME: &'static str = "MapPieceTexParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MapPieceTexParam {
    type Target = MAP_PIECE_TEX_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MapPieceTexParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MapPieceTexParam::MapPieceTexParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapPieceTexParam>(), 16)
    }
}
