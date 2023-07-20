/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST::KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST

pub struct KnowledgeLoadScreenItemParam {
    _data: KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST,
}
impl Param for KnowledgeLoadScreenItemParam {
    type Def = KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST;
    const NAME: &'static str = "KnowledgeLoadScreenItemParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for KnowledgeLoadScreenItemParam {
    type Target = KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for KnowledgeLoadScreenItemParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
