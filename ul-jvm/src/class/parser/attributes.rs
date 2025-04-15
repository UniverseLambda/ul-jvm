// TODO: Finish implementing those (https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7)
/*
Critical for the JVM:
- ConstantValue: WRITTEN
- Code: WRITTEN
- StackMapTable: WRITTEN
- BootstrapMethods: WRITTEN
- NestHost: WRITTEN
- NestMembers: WRITTEN
- PermittedSubclasses: WRITTEN
Critical for Java:
- Exceptions: WRITTEN
- InnerClasses: WRITTEN
- EnclosingMethod: WRITTEN
- Synthetic: WRITTEN
- Signature: WRITTEN
- Record: WRITTEN
- SourceFile: WRITTEN
- LineNumberTable: WRITTEN
- LocalVariableTable: WRITTEN
- LocalVariableTypeTable: WRITTEN
Exposed by Java, might be useful for libraries:
- SourceDebugExtension: WRITTEN
- Deprecated: WRITTEN
- RuntimeVisibleAnnotations
- RuntimeInvisibleAnnotations
- RuntimeVisibleParameterAnnotations
- RuntimeInvisibleParameterAnnotations
- RuntimeVisibleTypeAnnotations
- RuntimeInvisibleTypeAnnotations
- AnnotationDefault
- MethodParameters: WRITTEN
- Module
- ModulePackages
- ModuleMainClass
*/

use super::AttributeInfo;
use binrw::{BinRead, VecArgs};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, BinRead)]
#[br(big)]
pub struct ConstantValue {
    pub constantvalue_index: u16,
}

#[derive(Debug, Clone, BinRead)]
#[br(big)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    #[br(count = code_length)]
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    #[br(count = exception_table_length)]
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, BinRead)]
#[br(big)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Clone, BinRead)]
#[br(big)]
pub struct StackMapTable {
    pub number_of_entries: u16,
    #[br(count = number_of_entries)]
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    // value in 0-63
    Same {
        id: u8,
    },
    // value in 64-127
    SameLocals1StackItemFrame {
        id: u8,
        stack: [VerificationTypeInfo; 1],
    },
    // value is 247
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1],
    },
    // value in 248-250
    ChopFrame {
        id: u8,
        offset_delta: u16,
    },
    // value is 251
    SameFrameExtended {
        offset_delta: u16,
    },
    // value in 252-254
    AppendFrame {
        id: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    // value is 255
    FullFrame {
        offset_delta: u16,
        number_of_locals: u16,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: u16,
        stack: Vec<VerificationTypeInfo>,
    },
}

impl BinRead for StackMapFrame {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        Ok(match u8::read_options(reader, endian, args)? {
            id @ 0..=63 => Self::Same { id },
            id @ 64..=127 => Self::SameLocals1StackItemFrame {
                id,
                stack: [<_>::read_options(reader, endian, args)?],
            },
            247 => Self::SameLocals1StackItemFrameExtended {
                offset_delta: <_>::read_options(reader, endian, args)?,
                stack: [<_>::read_options(reader, endian, args)?],
            },
            id @ 248..=250 => Self::ChopFrame {
                id,
                offset_delta: <_>::read_options(reader, endian, args)?,
            },
            251 => Self::SameFrameExtended {
                offset_delta: <_>::read_options(reader, endian, args)?,
            },
            id @ 252..=254 => Self::AppendFrame {
                id,
                offset_delta: <_>::read_options(reader, endian, args)?,
                locals: <_>::read_options(
                    reader,
                    endian,
                    VecArgs::builder().count((id - 251) as _).finalize(),
                )?,
            },
            255 => {
                let offset_delta = <_>::read_options(reader, endian, args)?;
                let number_of_locals = <_>::read_options(reader, endian, args)?;
                let locals = <_>::read_options(
                    reader,
                    endian,
                    VecArgs::builder().count(number_of_locals as _).finalize(),
                )?;
                let number_of_stack_items = <_>::read_options(reader, endian, args)?;
                let stack = <_>::read_options(
                    reader,
                    endian,
                    VecArgs::builder()
                        .count(number_of_stack_items as _)
                        .finalize(),
                )?;

                Self::FullFrame {
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                }
            }
            v => return Err(binrw::Error::NoVariantMatch { pos: v as _ }),
        })
    }
}

#[derive(Debug, Clone, BinRead)]
pub enum VerificationTypeInfo {
    #[br(magic = 0x00u8)]
    Top,
    #[br(magic = 0x01u8)]
    Integer,
    #[br(magic = 0x02u8)]
    Float,
    #[br(magic = 0x03u8)]
    Long,
    #[br(magic = 0x04u8)]
    Double,
    #[br(magic = 0x05u8)]
    Null,
    #[br(magic = 0x06u8)]
    UninitializedThis,
    #[br(magic = 0x07u8)]
    Object { cpool_index: u16 },
    #[br(magic = 0x08u8)]
    Uninitialized { offset: u16 },
}

#[derive(Debug, Clone, BinRead)]
pub struct BootstrapMethods {
    pub num_bootstrap_methods: u16,
    #[br(count = num_bootstrap_methods)]
    pub bootstrap_methods: Vec<BootstrapMethodsEntry>,
}

#[derive(Debug, Clone, BinRead)]
pub struct BootstrapMethodsEntry {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    #[br(count = num_bootstrap_arguments)]
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone, BinRead)]
pub struct NestHost {
    pub host_class_index: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct NestMembers {
    pub number_of_classes: u16,
    #[br(count = number_of_classes)]
    pub classes: Vec<u16>,
}

#[derive(Debug, Clone, BinRead)]
pub struct PermittedSubclasses {
    pub number_of_classes: u16,
    #[br(count = number_of_classes)]
    pub classes: Vec<u16>,
}
// JAVA CRITICAL

#[derive(Debug, Clone, BinRead)]
pub struct Exceptions {
    pub number_of_exceptions: u16,
    #[br(count = number_of_exceptions)]
    pub exception_index_table: Vec<u16>,
}

#[derive(Debug, Clone, BinRead)]
pub struct InnerClasses {
    pub number_of_classes: u16,
    #[br(count = number_of_classes)]
    pub classes: Vec<InnerClassesEntry>,
}

#[derive(Debug, Clone, BinRead)]
pub struct InnerClassesEntry {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    #[br(map = |x: u16| InnerClassAccessFlags::from_bits(x))]
    pub inner_class_access_flags: Vec<InnerClassAccessFlags>,
}

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u16)]
pub enum InnerClassAccessFlags {
    Public = 0x0001,     // 	Marked or implicitly public in source.
    Private = 0x0002,    // 	Marked private in source.
    Protected = 0x0004,  // 	Marked protected in source.
    Static = 0x0008,     // 	Marked or implicitly static in source.
    Final = 0x0010,      // 	Marked or implicitly final in source.
    Interface = 0x0200,  // 	Was an interface in source.
    Abstract = 0x0400,   // 	Marked or implicitly abstract in source.
    Synthetic = 0x1000,  // 	Declared synthetic; not present in the source code.
    Annotation = 0x2000, // 	Declared as an annotation interface.
    Enum = 0x4000,       // 	Declared as an enum class.
}

impl InnerClassAccessFlags {
    pub fn from_bits(bits: u16) -> Vec<Self> {
        Self::iter().filter(|v| (bits & (*v as u16)) != 0).collect()
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct EnclosingMethod {
    pub class_index: u16,
    pub method_index: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct Synthetic {}

#[derive(Debug, Clone, BinRead)]
pub struct Signature {
    pub signature_index: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct SourceFile {
    pub sourcefile_index: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct LineNumberTable {
    pub line_number_table_length: u16,
    #[br(count = line_number_table_length)]
    pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug, Clone, BinRead)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct LocalVariableTable {
    pub local_variable_table_length: u16,
    #[br(count = local_variable_table_length)]
    pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug, Clone, BinRead)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct LocalVariableTypeTable {
    pub local_variable_type_table_length: u16,
    #[br(count = local_variable_type_table_length)]
    pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

#[derive(Debug, Clone, BinRead)]
pub struct LocalVariableTypeTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct Record {
    pub components_count: u16,
    #[br(count = components_count)]
    pub components: Vec<RecordComponentInfo>,
}

#[derive(Debug, Clone, BinRead)]
pub struct RecordComponentInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

// USEFUL

#[derive(Debug, Clone, BinRead)]
#[br(import(length: usize))]
pub struct SourceDebugExtension {
    #[br(count = length)]
    pub debug_extension: Vec<u8>,
}

#[derive(Debug, Clone, BinRead)]
pub struct Deprecated {}

#[derive(Debug, Clone, BinRead)]
pub struct MethodParameters {
    pub parameters_count: u8,
    #[br(count = parameters_count)]
    pub parameters: Vec<MethodParametersEntry>,
}

#[derive(Debug, Clone, BinRead)]
pub struct MethodParametersEntry {
    pub name_index: u16,
    #[br(map = |bits: u16| MethodParametersEntryAccessFlag::from_bits(bits))]
    pub access_flags: Vec<MethodParametersEntryAccessFlag>,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
#[repr(u16)]
pub enum MethodParametersEntryAccessFlag {
    Final = 0x0010,
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

impl MethodParametersEntryAccessFlag {
    pub fn from_bits(bits: u16) -> Vec<Self> {
        Self::iter().filter(|v| (bits & (*v as u16)) != 0).collect()
    }
}
