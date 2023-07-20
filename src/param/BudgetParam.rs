/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::BUDGET_PARAM_ST::BUDGET_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: BUDGET_PARAM_ST

pub struct BudgetParam {
    _data: BUDGET_PARAM_ST,
}
impl Param for BudgetParam {
    type Def = BUDGET_PARAM_ST;
    const NAME: &'static str = "BudgetParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for BudgetParam {
    type Target = BUDGET_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for BudgetParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::BudgetParam::BudgetParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BudgetParam>(), 132)
    }
}
