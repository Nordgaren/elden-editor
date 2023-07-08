/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_PIECE_TEX_PARAM_ST.rs");

/// Type: MAP_PIECE_TEX_PARAM_ST

pub type MapPieceTexParam = ParamStruct<MAP_PIECE_TEX_PARAM_ST>;
impl Param for ParamStruct<MAP_PIECE_TEX_PARAM_ST> {
    const NAME: &'static str = "MapPieceTexParam";
    const TYPE_NAME: &'static str = "MAP_PIECE_TEX_PARAM_ST";
    const VERSION: u16 = 1;
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
