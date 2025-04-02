use constant_pool::ConstantPool;

pub mod parser;

pub struct JvmUnit {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub is_public: bool,
}

pub struct JvmClass {}

mod constant_pool {
    use crate::types::{
        JvmDouble, JvmFloat, JvmInt, JvmLong, JvmMethodDescriptor, JvmTypeDescriptor,
    };

    use super::parser::{MethodKind, ModifiedUtf8String};
    use std::sync::Arc;

    pub struct ConstantPool {}

    pub type ConstantJvmUtf8 = Arc<ModifiedUtf8String>;
    pub type ConstantInteger = JvmInt;
    pub type ConstantFloat = JvmFloat;
    pub type ConstantLong = JvmLong;
    pub type ConstantDouble = JvmDouble;

    #[derive(Debug, Clone)]
    pub struct ConstantClass {
        pub name: ConstantJvmUtf8,
    }

    pub enum JvmConstant {
        Class(ConstantClass),
        Fieldref {
            class: ConstantClass,
            name: ConstantJvmUtf8,
            r#type: JvmTypeDescriptor,
        },
        Methodref {
            class: ConstantClass,
            name: ConstantJvmUtf8,
            r#type: JvmMethodDescriptor,
        },
        InterfaceMethodref {
            class: ConstantClass,
            name: ConstantJvmUtf8,
            r#type: JvmMethodDescriptor,
        },
        String(ConstantJvmUtf8),
        Integer(ConstantInteger),
        Float(ConstantFloat),
        Long(ConstantLong),
        Double(ConstantDouble),
        // MethodHandle {
        //     kind: MethodKind,

        // }
        MethodType {
            descriptor: ConstantJvmUtf8,
        },
        Dynamic {
            bootstrap_method_attr_index: u16, // TODO: bootstrap methods
            name: ConstantJvmUtf8,
            r#type: JvmTypeDescriptor,
        },
        DynamicInvoke {
            bootstrap_method_attr_index: u16, // TODO: bootstrap methods
            name: ConstantJvmUtf8,
            r#type: JvmMethodDescriptor,
        },
        Module {
            name: ConstantJvmUtf8,
        },
        Package {
            name: ConstantJvmUtf8,
        },
    }
}
