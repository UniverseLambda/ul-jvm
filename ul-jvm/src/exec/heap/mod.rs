mod jvm_ref;

pub use jvm_ref::{ArrayRef, ClassRef};
use jvm_ref::{StrongArrayRef, StrongClassRef};
use parking_lot::Mutex;

use crate::types::{JvmInt, JvmTypeDescriptor};

use super::{array::Array, class::Class};

#[derive(Debug)]
pub enum AllocatableType {
    Array(StrongArrayRef),
    Class(StrongClassRef),
}

#[derive(Debug, Default)]
pub struct JvmHeap {
    values: Mutex<Vec<AllocatableType>>,
}

impl JvmHeap {
    pub fn new() -> Self {
        Self {
            values: Mutex::new(Vec::new()),
        }
    }

    pub fn new_array(&self, compound_type: JvmTypeDescriptor, size: JvmInt) -> ArrayRef {
        let strong_ref = StrongArrayRef::new(Array::new_default(compound_type, size));
        let ret_ref = strong_ref.new_ref();

        self.values.lock().push(AllocatableType::Array(strong_ref));

        ret_ref
    }

    pub fn new_object(&self, class: Class) -> ClassRef {
        let strong_ref = StrongClassRef::new(class.instanciate_uninit().assume_init());
        let ret_ref = strong_ref.new_ref();

        self.values.lock().push(AllocatableType::Class(strong_ref));

        ret_ref
    }
}
