use std::ops::{Deref, DerefMut};

// Make a single generic wrapper for named params
pub struct ParamStruct<T> {
    data: T,
}

// Add a Deref implementation so ParamStruct<T> derefs to T
impl<T> Deref for ParamStruct<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ParamStruct<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data } }

pub trait Param {
    const NAME: &'static str;
    const TYPE_NAME: &'static str;
    const VERSION: u16;

    fn name() -> &'static str {
        Self::NAME
    }
    // So you can query the type constant from an `impl Param`
    fn param_type_name() -> &'static str {
        Self::TYPE_NAME
    }
    fn version() -> u16 {
        Self::VERSION
    }
    // etc...
}


