use crate::types::JvmTypeDescriptor;

use super::{heap::ClassRef, runtime_type::RuntimeType};

#[derive(Debug, Clone)]
pub enum Method {
    Normal(NormalMethod),
    Abstract(AbstractMethod),
    Native(NativeMethod),
}

impl Method {
    pub fn new_normal(
        return_type: Option<JvmTypeDescriptor>,
        parameters: Vec<JvmTypeDescriptor>,
        name: String,
        is_static: bool,
        cp_start: usize,
        cp_end: usize,
    ) -> Self {
        Self::Normal(NormalMethod {
            return_type,
            parameters,
            name,
            is_static,
            cp_start,
            cp_end,
        })
    }

    pub fn new_abstract(
        return_type: Option<JvmTypeDescriptor>,
        parameters: Vec<JvmTypeDescriptor>,
        name: String,
        is_static: bool,
    ) -> Self {
        Self::Abstract(AbstractMethod {
            return_type,
            parameters,
            name,
            is_static,
        })
    }

    pub fn new_native(
        return_type: Option<JvmTypeDescriptor>,
        parameters: Vec<JvmTypeDescriptor>,
        name: String,
        is_static: bool,
    ) -> Self {
        Self::Native(NativeMethod {
            return_type,
            parameters,
            name,
            is_static,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NormalMethod {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: String,
    is_static: bool,
    cp_start: usize,
    cp_end: usize,
}

#[derive(Debug, Clone)]
pub struct AbstractMethod {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: String,
    is_static: bool,
}

#[derive(Debug, Clone)]
pub struct NativeMethod {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: String,
    is_static: bool,
}

pub trait JvmMethod {
    fn return_type() -> Option<JvmTypeDescriptor>;
    fn parameters() -> Vec<JvmTypeDescriptor>;
    fn is_static() -> bool;

    fn call_this(this: ClassRef, args: Vec<RuntimeType>) -> anyhow::Result<ReturnResult>;
    fn call_static(args: Vec<RuntimeType>) -> anyhow::Result<ReturnResult>;
}

#[derive(Debug, Clone)]
pub enum ReturnResult {
    Complete(RuntimeType),
    Exception(ClassRef),
}
