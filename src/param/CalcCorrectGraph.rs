/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CACL_CORRECT_GRAPH_ST.rs");

/// Type: CACL_CORRECT_GRAPH_ST

pub type CalcCorrectGraph = ParamStruct<CACL_CORRECT_GRAPH_ST>;
impl Param for ParamStruct<CACL_CORRECT_GRAPH_ST> {
    const NAME: &'static str = "CalcCorrectGraph";
    const TYPE_NAME: &'static str = "CACL_CORRECT_GRAPH_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::CalcCorrectGraph::CalcCorrectGraph;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CalcCorrectGraph>(), 80)
    }
}
