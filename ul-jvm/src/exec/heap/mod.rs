mod jvm_ref;

pub use jvm_ref::{ArrayRef, ObjectRef, StrongArrayRef, StrongObjectRef};
use parking_lot::Mutex;

use crate::types::{JvmInt, JvmTypeDescriptor};

use super::{
    array::Array,
    class::{Class, ClassInstance},
};

#[derive(Debug)]
pub enum AllocatableType {
    Array(StrongArrayRef),
    Class(StrongObjectRef),
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

    pub fn new_object(&self, class: Class) -> ObjectRef {
        let strong_ref = StrongObjectRef::new(class.instanciate_uninit());
        let ret_ref = strong_ref.new_ref();

        self.values.lock().push(AllocatableType::Class(strong_ref));

        ret_ref
    }

    pub fn store_object(&self, instance: ClassInstance) -> StrongObjectRef {
        let strong_ref = StrongObjectRef::new(instance);

        self.values
            .lock()
            .push(AllocatableType::Class(strong_ref.duplicate()));

        strong_ref
    }
}
