use std::sync::Arc;

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
        name: Arc<String>,
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
        name: Arc<String>,
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
        name: Arc<String>,
        is_static: bool,
    ) -> Self {
        Self::Native(NativeMethod {
            return_type,
            parameters,
            name,
            is_static,
        })
    }

    pub fn is_static(&self) -> bool {
        match self {
            Method::Normal(method) => method.is_static,
            Method::Abstract(method) => method.is_static,
            Method::Native(method) => method.is_static,
        }
    }

    pub fn parameters(&self) -> &[JvmTypeDescriptor] {
        match self {
            Method::Normal(m) => &m.parameters,
            Method::Abstract(m) => &m.parameters,
            Method::Native(m) => &m.parameters,
        }
    }

    pub fn ret_type(&self) -> &Option<JvmTypeDescriptor> {
        match self {
            Method::Normal(m) => &m.return_type,
            Method::Abstract(m) => &m.return_type,
            Method::Native(m) => &m.return_type,
        }
    }

    pub fn start_pc(&self) -> Option<usize> {
        match self {
            Method::Normal(normal_method) => Some(normal_method.cp_start),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalMethod {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: Arc<String>,
    is_static: bool,
    cp_start: usize,
    cp_end: usize,
}

#[derive(Debug, Clone)]
pub struct AbstractMethod {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: Arc<String>,
    is_static: bool,
}

#[derive(Debug, Clone)]
pub struct NativeMethod {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: Arc<String>,
    is_static: bool,
}

#[derive(Debug, Clone)]
pub enum ReturnResult {
    Complete(RuntimeType),
    Exception(ClassRef),
}
