use crate::{
    class::constant_pool::{ConstantJvmUtf8, LoadableJvmConstant},
    types::{JvmDouble, JvmFloat, JvmInt, JvmLong, JvmTypeDescriptor},
};

use super::heap::{ArrayRef, ClassRef};

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Int(JvmInt),
    Long(JvmLong),
    Float(JvmFloat),
    Double(JvmDouble),
    Array(ArrayRef),
    Class(ClassRef),
    InternedString(ConstantJvmUtf8),
}

impl From<LoadableJvmConstant> for RuntimeType {
    fn from(value: LoadableJvmConstant) -> Self {
        match value {
            LoadableJvmConstant::Class(_) => Self::Class(ClassRef::new_null()),
            LoadableJvmConstant::String(string) => Self::InternedString(string),
            LoadableJvmConstant::Integer(v) => Self::Int(v),
            LoadableJvmConstant::Float(v) => Self::Float(v),
            LoadableJvmConstant::Long(v) => Self::Long(v),
            LoadableJvmConstant::Double(v) => Self::Double(v),
            LoadableJvmConstant::MethodHandle(_) => {
                todo!("MethodHandle from constant not yet implemented")
            }
            LoadableJvmConstant::MethodType { .. } => {
                todo!("MethodType from constant not yet implemented")
            }
            LoadableJvmConstant::Dynamic { .. } => {
                todo!("Dynamic from constant not yet implemented")
            }
        }
    }
}

impl RuntimeType {
    pub fn default_of(type_descriptor: &JvmTypeDescriptor) -> Self {
        match type_descriptor {
            JvmTypeDescriptor::Byte
            | JvmTypeDescriptor::Char
            | JvmTypeDescriptor::Int
            | JvmTypeDescriptor::Short
            | JvmTypeDescriptor::Boolean => Self::Int(0),
            JvmTypeDescriptor::Long => Self::Long(0),
            JvmTypeDescriptor::Double => Self::Float(0f32),
            JvmTypeDescriptor::Float => Self::Double(0f64),
            JvmTypeDescriptor::Class(_) => Self::Class(ClassRef::new_null()),
            JvmTypeDescriptor::Array(_) => Self::Array(ArrayRef::new_null()),
        }
    }
}
