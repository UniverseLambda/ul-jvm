use crate::{
    class::constant_pool::{ConstantJvmUtf8, LoadableJvmConstant},
    types::{JvmDouble, JvmFloat, JvmInt, JvmLong, JvmTypeDescriptor, NativeJvmType},
};

use super::heap::{ArrayRef, ObjectRef};

#[derive(Debug, Clone)]
pub enum RuntimeType {
    Int(JvmInt),
    Long(JvmLong),
    Float(JvmFloat),
    Double(JvmDouble),
    Array(ArrayRef),
    Class(ObjectRef),
    InternedString(ConstantJvmUtf8),
    ReturnAddress(usize),
}

impl RuntimeType {
    pub fn is_two_slots(&self) -> bool {
        matches!(self, Self::Long(_) | Self::Double(_))
    }
}

impl From<LoadableJvmConstant> for RuntimeType {
    fn from(value: LoadableJvmConstant) -> Self {
        match value {
            LoadableJvmConstant::Class(_) => Self::Class(ObjectRef::new_null()),
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
            JvmTypeDescriptor::Class(_) => Self::Class(ObjectRef::new_null()),
            JvmTypeDescriptor::Array(_) => Self::Array(ArrayRef::new_null()),
        }
    }

    pub fn try_into_native<N: NativeJvmType>(&self) -> Option<N> {
        N::try_from_rt(self)
    }
}
