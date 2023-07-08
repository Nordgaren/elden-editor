/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CS_KEY_ASSIGN_MENUITEM_PARAM.rs");

/// Type: CS_KEY_ASSIGN_MENUITEM_PARAM

pub type KeyAssignMenuItemParam = ParamStruct<CS_KEY_ASSIGN_MENUITEM_PARAM>;
impl Param for ParamStruct<CS_KEY_ASSIGN_MENUITEM_PARAM> {
    const NAME: &'static str = "KeyAssignMenuItemParam";
    const TYPE_NAME: &'static str = "CS_KEY_ASSIGN_MENUITEM_PARAM";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::KeyAssignMenuItemParam::KeyAssignMenuItemParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<KeyAssignMenuItemParam>(), 24)
    }
}
