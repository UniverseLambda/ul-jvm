use binrw::{BinRead, BinResult};
use strum::{EnumIter, IntoEnumIterator};

/* From Java SE 23 spec:
ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/
#[derive(Debug, Clone, BinRead)]
#[br(magic = 0xCAFEBABEu32, big)]
pub struct ClassFile {
    #[br(dbg)]
    pub minor_version: u16,
    #[br(dbg)]
    pub major_version: u16,
    #[br(dbg)]
    pub constant_pool_count: u16,
    #[br(dbg, args(constant_pool_count), parse_with = parse_constant_pool)]
    pub constant_pool: Vec<ConstantPoolInfo>,
    #[br(dbg, map = |bits: u16| ClassAccessFlags::from_bits(bits))]
    pub access_flags: Vec<ClassAccessFlags>,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    #[br(count = interfaces_count)]
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    #[br(count = fields_count)]
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    #[br(count = methods_count)]
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

#[binrw::parser(reader, endian)]
fn parse_constant_pool(constant_pool_count: u16) -> BinResult<Vec<ConstantPoolInfo>> {
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

#[derive(Debug, Clone, BinRead)]
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
    NameAndType { name: u16, descriptor_index: u16 },
    #[br(magic = 1u8)]
    Utf8 {
        length: u16,
        #[br(args(length))]
        bytes: ModifiedUtf8String,
    },
    #[br(magic = 15u8)]
    MethodHandle {
        reference_kind: u8,
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

// TODO: implement decoder
#[derive(Debug, Clone, BinRead)]
#[br(import(length: u16))]
pub struct ModifiedUtf8String(
    #[br(count = length, assert(!self_0.contains(&0), "string contains \\0"), assert(!self_0.iter().any(|v| 0xf0 <= *v), "invalid byte in string"))]
     Vec<u8>,
);

impl AsRef<[u8]> for ModifiedUtf8String {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u16)]
pub enum ClassAccessFlags {
    AccPublic = 0x0001, // 	Declared public; may be accessed from outside its package.
    AccFinal = 0x0010,  // 	Declared final; no subclasses allowed.
    AccSuper = 0x0020, // 	Treat superclass methods specially when invoked by the invokespecial instruction.
    AccInterface = 0x0200, // 	Is an interface, not a class.
    AccAbstract = 0x0400, // 	Declared abstract; must not be instantiated.
    AccSynthetic = 0x1000, // 	Declared synthetic; not present in the source code.
    AccAnnotation = 0x2000, // 	Declared as an annotation interface.
    AccEnum = 0x4000,  // 	Declared as an enum class.
    AccModule = 0x8000, // 	Is a module, not a class or interface.
}

impl ClassAccessFlags {
    pub fn from_bits(bits: u16) -> Vec<Self> {
        Self::iter().filter(|v| (bits & (*v as u16)) != 0).collect()
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct FieldInfo {
    #[br(map = |bits: u16| FieldAccessFlags::from_bits(bits))]
    pub access_flags: Vec<FieldAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u16)]
pub enum FieldAccessFlags {
    AccPublic = 0x0001,  // 	Declared public; may be accessed from outside its package.
    AccPrivate = 0x0002, // 	Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
    AccProtected = 0x0004, // 	Declared protected; may be accessed within subclasses.
    AccStatic = 0x0008,  // 	Declared static.
    AccFinal = 0x0010, // 	Declared final; never directly assigned to after object construction (JLS §17.5).
    AccVolatile = 0x0040, // 	Declared volatile; cannot be cached.
    AccTransient = 0x0080, // 	Declared transient; not written or read by a persistent object manager.
    AccSynthetic = 0x1000, // 	Declared synthetic; not present in the source code.
    AccEnum = 0x4000,      // 	Declared as an element of an enum class.
}

impl FieldAccessFlags {
    pub fn from_bits(bits: u16) -> Vec<Self> {
        Self::iter().filter(|v| (bits & (*v as u16)) != 0).collect()
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct MethodInfo {
    #[br(map = |bits: u16| MethodAccessFlags::from_bits(bits))]
    pub access_flags: Vec<MethodAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u16)]
pub enum MethodAccessFlags {
    AccPublic = 0x0001,  // 	Declared public; may be accessed from outside its package.
    AccPrivate = 0x0002, // 	Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
    AccProtected = 0x0004, // 	Declared protected; may be accessed within subclasses.
    AccStatic = 0x0008,  // 	Declared static.
    AccFinal = 0x0010,   // 	Declared final; must not be overridden (§5.4.5).
    AccSynchronized = 0x0020, // 	Declared synchronized; invocation is wrapped by a monitor use.
    AccBridge = 0x0040,  // 	A bridge method, generated by the compiler.
    AccVarargs = 0x0080, // 	Declared with variable number of arguments.
    AccNative = 0x0100, // 	Declared native; implemented in a language other than the Java programming language.
    AccAbstract = 0x0400, // 	Declared abstract; no implementation is provided.
    AccStrict = 0x0800, // 	In a class file whose major version number is at least 46 and at most 60: Declared strictfp.
    AccSynthetic = 0x1000, // 	Declared synthetic; not present in the source code.
}

impl MethodAccessFlags {
    pub fn from_bits(bits: u16) -> Vec<Self> {
        Self::iter().filter(|v| (bits & (*v as u16)) != 0).collect()
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    #[br(count = attribute_length)]
    pub info: Vec<u8>,
}

pub mod attributes {
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
    - MethodParameters
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
        pub code: Vec<u64>,
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
        pub attribute_name_index: u16,
        pub attribute_length: u32,
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
        AccPublic = 0x0001,     // 	Marked or implicitly public in source.
        AccPrivate = 0x0002,    // 	Marked private in source.
        AccProtected = 0x0004,  // 	Marked protected in source.
        AccStatic = 0x0008,     // 	Marked or implicitly static in source.
        AccFinal = 0x0010,      // 	Marked or implicitly final in source.
        AccInterface = 0x0200,  // 	Was an interface in source.
        AccAbstract = 0x0400,   // 	Marked or implicitly abstract in source.
        AccSynthetic = 0x1000,  // 	Declared synthetic; not present in the source code.
        AccAnnotation = 0x2000, // 	Declared as an annotation interface.
        AccEnum = 0x4000,       // 	Declared as an enum class.
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
        pub attribute_name_index: u16,
        pub attribute_length: u32,
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
}
