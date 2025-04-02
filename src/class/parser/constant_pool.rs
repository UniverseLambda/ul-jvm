use binrw::{BinRead, BinResult};
use serde::Serialize;

use super::ModifiedUtf8String;

#[binrw::parser(reader, endian)]
pub(super) fn parse_constant_pool(constant_pool_count: u16) -> BinResult<Vec<ConstantPoolInfo>> {
    let mut result = vec![ConstantPoolInfo::Ignored];

    while result.len() < constant_pool_count as usize {
        let info = ConstantPoolInfo::read_options(reader, endian, ())?;

        result.push(info.clone());

        if matches!(
            info,
            ConstantPoolInfo::Long { .. } | ConstantPoolInfo::Double { .. }
        ) {
            result.push(ConstantPoolInfo::Ignored);
        }
    }

    Ok(result)
}

#[derive(Debug, Clone, BinRead, Serialize)]
pub enum ConstantPoolInfo {
    #[br(magic = 0u8)]
    Ignored,
    #[br(magic = 7u8)]
    Class { name_index: u16 },
    #[br(magic = 9u8)]
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 10u8)]
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 11u8)]
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 8u8)]
    String { string_index: u16 },
    #[br(magic = 3u8)]
    Integer { bytes: u32 },
    #[br(magic = 4u8)]
    Float { bytes: u32 },
    #[br(magic = 5u8)]
    Long { high_bytes: u32, low_bytes: u32 },
    #[br(magic = 6u8)]
    Double { high_bytes: u32, low_bytes: u32 },
    #[br(magic = 12u8)]
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    #[br(magic = 1u8)]
    Utf8 {
        length: u16,
        #[br(args(length))]
        bytes: ModifiedUtf8String,
    },
    #[br(magic = 15u8)]
    MethodHandle {
        reference_kind: MethodKind,
        reference_index: u16,
    },
    #[br(magic = 16u8)]
    MethodType { descriptor_index: u16 },
    #[br(magic = 17u8)]
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 18u8)]
    DynamicInvoke {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 19u8)]
    Module { name_index: u16 },
    #[br(magic = 19u8)]
    Package { name_index: u16 },
}

#[derive(Debug, Clone, BinRead, Serialize)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    #[br(count = attribute_length)]
    pub info: Vec<u8>,
}

#[derive(Debug, Clone, BinRead, Serialize)]
pub enum MethodKind {
    #[br(magic = 1u8)]
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}
