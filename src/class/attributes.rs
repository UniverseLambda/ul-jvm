use crate::types::JvmTypeDescriptor;

use super::{
    constant_pool::{
        ConstantClass, ConstantJvmUtf8, ConstantMethodHandle, ConstantMethodref,
        LoadableJvmConstant,
    },
    jvm_unit::JvmVisibility,
};

#[derive(Debug, Clone)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub stack_map_table: Option<StackMapTable>,
    pub line_number_table: Vec<LineNumberTable>,
    pub local_variable_table: Vec<LocalVariableTable>,
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantClass,
}

#[derive(Debug, Clone)]
pub struct StackMapTable {
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    Same {
        id: u8,
    },
    SameLocals1StackItemFrame {
        id: u8,
        stack: [VerificationTypeInfo; 1],
    },
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1],
    },
    ChopFrame {
        id: u8,
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        id: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: u16,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { object: ConstantClass },
    Uninitialized { offset: u16 },
}

#[derive(Debug, Clone)]
pub struct BootstrapMethods {
    pub bootstrap_methods: Vec<BootstrapMethodsEntry>,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethodsEntry {
    pub bootstrap_method_ref: ConstantMethodHandle,
    pub bootstrap_arguments: Vec<LoadableJvmConstant>,
}

#[derive(Debug, Clone)]
pub struct NestHost {
    pub host_class_index: ConstantClass,
}

#[derive(Debug, Clone)]
pub struct NestMembers {
    pub classes: Vec<ConstantClass>,
}

#[derive(Debug, Clone)]
pub struct PermittedSubclasses {
    pub classes: Vec<ConstantClass>,
}
// JAVA CRITICAL

// Directly declared inside JvmUnitMethod
// #[derive(Debug, Clone)]
// pub struct Exceptions {
//     pub exception_index_table: Vec<ConstantClass>,
// }

#[derive(Debug, Clone)]
pub struct InnerClasses {
    pub classes: Vec<InnerClassesEntry>,
}

#[derive(Debug, Clone)]
pub struct InnerClassesEntry {
    pub inner_class_info_index: ConstantClass,
    pub outer_class_info_index: Option<ConstantClass>,
    pub inner_name_index: Option<ConstantJvmUtf8>,
    pub vis: JvmVisibility,
    pub is_static: bool,
    pub is_final: bool,
    pub is_interface: bool,
    pub is_abstract: bool,
    pub is_synthetic: bool,
    pub is_annotation: bool,
    pub is_enum: bool,
}

#[derive(Debug, Clone)]
pub struct EnclosingMethod {
    pub class: ConstantClass,
    pub method: Option<ConstantMethodref>,
}

#[derive(Debug, Clone)]
pub struct Synthetic;

#[derive(Debug, Clone)]
pub struct Signature {
    pub signature: ConstantJvmUtf8,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub sourcefile: ConstantJvmUtf8,
}

#[derive(Debug, Clone)]
pub struct LineNumberTable {
    pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug, Clone)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTable {
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantJvmUtf8,
    pub descriptor: JvmTypeDescriptor,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantJvmUtf8,
    pub signature: ConstantJvmUtf8, // TODO: Signature
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub components: Vec<RecordComponentInfo>,
}

#[derive(Debug, Clone)]
pub struct RecordComponentInfo {
    pub name: ConstantJvmUtf8,
    pub descriptor: JvmTypeDescriptor,
}

// USEFUL

#[derive(Debug, Clone)]
pub struct SourceDebugExtension {
    pub debug_extension: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Deprecated {}

#[derive(Debug, Clone)]
pub struct MethodParameter {
    pub name: ConstantJvmUtf8,
    pub is_final: bool,
    pub is_synthetic: bool,
    pub is_mandated: bool,
}
