/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST.rs");

/// Type: KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST

pub type KnowledgeLoadScreenItemParam = ParamStruct<KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST>;
impl Param for ParamStruct<KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST> {
    const NAME: &'static str = "KnowledgeLoadScreenItemParam";
    const TYPE_NAME: &'static str = "KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::KnowledgeLoadScreenItemParam::KnowledgeLoadScreenItemParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<KnowledgeLoadScreenItemParam>(), 16)
    }
}
