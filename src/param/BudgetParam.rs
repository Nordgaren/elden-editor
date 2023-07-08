/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BUDGET_PARAM_ST.rs");

/// Type: BUDGET_PARAM_ST

pub type BudgetParam = ParamStruct<BUDGET_PARAM_ST>;
impl Param for ParamStruct<BUDGET_PARAM_ST> {
	const NAME: &'static str = "BudgetParam";
	const TYPE_NAME: &'static str = "BUDGET_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BudgetParam::BudgetParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BudgetParam>(), 132)
	}
}
