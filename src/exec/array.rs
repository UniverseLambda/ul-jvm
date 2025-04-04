use crate::types::{JvmInt, JvmTypeDescriptor};

use super::runtime_type::RuntimeType;

#[derive(Debug, Clone)]
pub struct Array {
    pub compound_type: JvmTypeDescriptor,
    pub array: Box<[RuntimeType]>,
}

impl Array {
    pub fn new_default(compound_type: JvmTypeDescriptor, len: JvmInt) -> Self {
        let mut content = Vec::with_capacity(len as usize);
        let default_value = RuntimeType::default_of(&compound_type);

        for _ in 0..len {
            content.push(default_value.clone());
        }

        Self {
            compound_type,
            array: content.into_boxed_slice(),
        }
    }
}
