mod jvm_ref;

pub use jvm_ref::{ArrayRef, ClassRef};

#[derive(Debug, Clone)]
pub enum AllocatableType {
    Array(ArrayRef),
    Class(ClassRef),
}

#[derive(Debug)]
pub struct JvmHeap {}

impl JvmHeap {
    pub fn new() -> Self {
        Self {}
    }
}
