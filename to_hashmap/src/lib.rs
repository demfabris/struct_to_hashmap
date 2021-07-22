use std::collections::HashMap;

#[derive(Debug)]
pub enum WrapperKind {
    Number,
    Text,
    Date,
}

#[derive(Debug)]
pub struct Wrapper {
    pub kind: Option<WrapperKind>,
}

pub trait Wrapped {
    fn as_wrapper(&self) -> Wrapper;
}

impl Wrapped for i32 {
    fn as_wrapper(&self) -> Wrapper {
        Wrapper {
            kind: Some(WrapperKind::Number),
        }
    }
}

pub trait ToHashMap {
    fn to_hashmap(&self) -> HashMap<String, Wrapper>;
}
