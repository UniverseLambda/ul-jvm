use std::sync::Arc;

use crate::types::JvmTypeDescriptor;

use super::{heap::ObjectRef, runtime_type::RuntimeType};

#[derive(Debug, Clone)]
pub struct Method {
    return_type: Option<JvmTypeDescriptor>,
    parameters: Vec<JvmTypeDescriptor>,
    name: Arc<String>,
    spec: MethodSpec,
}

#[derive(Debug, Clone)]
pub enum MethodSpec {
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
        local_count: usize,
    ) -> Self {
        Self {
            return_type,
            parameters,
            name,
            spec: MethodSpec::Normal(NormalMethod {
                is_static,
                cp_start,
                cp_end,
                local_count,
            }),
        }
    }

    pub fn new_abstract(
        return_type: Option<JvmTypeDescriptor>,
        parameters: Vec<JvmTypeDescriptor>,
        name: Arc<String>,
    ) -> Self {
        Self {
            return_type,
            parameters,
            name,
            spec: MethodSpec::Abstract(AbstractMethod {}),
        }
    }

    pub fn new_native(
        return_type: Option<JvmTypeDescriptor>,
        parameters: Vec<JvmTypeDescriptor>,
        name: Arc<String>,
        is_static: bool,
    ) -> Self {
        Self {
            return_type,
            parameters,
            name,
            spec: MethodSpec::Native(NativeMethod { is_static }),
        }
    }

    pub fn is_static(&self) -> bool {
        match &self.spec {
            MethodSpec::Normal(method) => method.is_static,
            MethodSpec::Native(method) => method.is_static,
            MethodSpec::Abstract(_) => false,
        }
    }

    pub fn is_native(&self) -> bool {
        matches!(self.spec, MethodSpec::Native(_));
        match self.spec {
            MethodSpec::Native(_) => true,
            _ => false,
        }
    }

    pub fn parameters(&self) -> &[JvmTypeDescriptor] {
        &self.parameters
    }

    pub fn ret_type(&self) -> &Option<JvmTypeDescriptor> {
        &self.return_type
    }

    pub fn start_pc(&self) -> Option<usize> {
        match &self.spec {
            MethodSpec::Normal(normal_method) => Some(normal_method.cp_start),
            _ => None,
        }
    }

    pub fn local_count(&self) -> usize {
        match &self.spec {
            MethodSpec::Normal(m) => m.local_count,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalMethod {
    is_static: bool,
    cp_start: usize,
    cp_end: usize,
    local_count: usize,
}

#[derive(Debug, Clone)]
pub struct AbstractMethod {}

#[derive(Debug, Clone)]
pub struct NativeMethod {
    is_static: bool,
}

#[derive(Debug, Clone)]
pub enum ReturnResult {
    Complete(RuntimeType),
    Exception(ObjectRef),
}
