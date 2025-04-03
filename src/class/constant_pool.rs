use either::Either;
use serde::Serialize;

use crate::types::{JvmDouble, JvmFloat, JvmInt, JvmLong, JvmMethodDescriptor, JvmTypeDescriptor};

use super::parser::ModifiedUtf8String;
use std::sync::Arc;

pub type ConstantJvmUtf8 = Arc<ModifiedUtf8String>;
pub type ConstantInteger = JvmInt;
pub type ConstantFloat = JvmFloat;
pub type ConstantLong = JvmLong;
pub type ConstantDouble = JvmDouble;

#[derive(Debug, Clone, Serialize)]
pub struct ConstantClass {
    pub name: ConstantJvmUtf8,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstantFieldref {
    pub class: ConstantClass,
    pub name: ConstantJvmUtf8,
    pub ty: JvmTypeDescriptor,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstantMethodref {
    pub class: ConstantClass,
    pub name: ConstantJvmUtf8,
    pub ty: JvmMethodDescriptor,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstantInterfaceMethodref {
    pub class: ConstantClass,
    pub name: ConstantJvmUtf8,
    pub ty: JvmMethodDescriptor,
}

#[derive(Debug, Clone, Serialize)]
pub struct DynamicInvoke {
    pub bootstrap_method_attr_index: u16, // TODO: bootstrap methods
    pub name: ConstantJvmUtf8,
    pub ty: JvmMethodDescriptor,
}

#[derive(Debug, Clone, Serialize)]
pub enum ConstantMethodHandle {
    GetField(ConstantFieldref),
    GetStatic(ConstantFieldref),
    PutField(ConstantFieldref),
    PutStatic(ConstantFieldref),
    InvokeVirtual(ConstantMethodref),
    NewInvokeSpecial(ConstantMethodref),
    InvokeStatic(Either<ConstantMethodref, ConstantInterfaceMethodref>),
    InvokeSpecial(Either<ConstantMethodref, ConstantInterfaceMethodref>),
    InvokeInterface(ConstantInterfaceMethodref),
}

#[derive(Debug, Clone, Serialize)]
pub enum LoadableJvmConstant {
    Class(ConstantClass),
    // THOSE ARE NOT LOADABLE
    // Fieldref(ConstantFieldref),
    // Methodref(ConstantMethodref),
    // InterfaceMethodref(ConstantInterfaceMethodref),
    String(ConstantJvmUtf8),
    Integer(ConstantInteger),
    Float(ConstantFloat),
    Long(ConstantLong),
    Double(ConstantDouble),
    MethodHandle(ConstantMethodHandle),
    MethodType {
        descriptor: JvmMethodDescriptor,
    },
    Dynamic {
        bootstrap_method_attr_index: u16, // TODO: bootstrap methods
        name: ConstantJvmUtf8,
        ty: JvmTypeDescriptor,
    },
    // THOSE ARE NOT LOADABLE
    // DynamicInvoke {
    //     bootstrap_method_attr_index: u16, // TODO: bootstrap methods
    //     name: ConstantJvmUtf8,
    //     ty: JvmMethodDescriptor,
    // },
    // Module {
    //     name: ConstantJvmUtf8,
    // },
    // Package {
    //     name: ConstantJvmUtf8,
    // },
}
