use std::ops::Deref;

use binrw::BinRead;

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
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    #[br(count = if constant_pool_count == 0 { 0 } else { constant_pool_count - 1 })]
    pub constant_pool: Vec<u16>,
    #[br(map = |bits: u16| ClassAccessFlags::from_bits(bits))]
    pub access_flags: Vec<ClassAccessFlags>,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    #[br(count = if interfaces_count == 0 { 0 } else { interfaces_count - 1 })]
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    #[br(count = if fields_count == 0 { 0 } else { fields_count - 1 })]
    pub fields: Vec<u16>,
    pub methods_count: u16,
    #[br(count = if methods_count == 0 { 0 } else { methods_count - 1 })]
    pub methods: Vec<u16>,
    pub attributes_count: u16,
    #[br(count = if attributes_count == 0 { 0 } else { attributes_count - 1 })]
    pub attributes: Vec<u16>,
}

#[derive(Debug, Clone, BinRead)]
pub enum ConstantPoolInfo {
    #[br(magic = 7u8)]
    ClassInfo { name_index: u16 },
    #[br(magic = 9u8)]
    FieldrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 10u8)]
    MethodrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 11u8)]
    InterfaceMethodrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 8u8)]
    StringInfo { string_index: u16 },
    #[br(magic = 3u8)]
    IntegerInfo { bytes: u32 },
    #[br(magic = 4u8)]
    FloatInfo { bytes: u32 },
    #[br(magic = 5u8)]
    LongInfo { high_bytes: u32, low_bytes: u32 },
    #[br(magic = 6u8)]
    DoubleInfo { high_bytes: u32, low_bytes: u32 },
    #[br(magic = 12u8)]
    NameAndTypeInfo { name: u16, descriptor_index: u16 },
    #[br(magic = 1u8)]
    Utf8Info {
        length: u16,
        #[br(args(length))]
        bytes: ModifiedUtf8String,
    },
    #[br(magic = 15u8)]
    MethodHandleInfo {
        reference_kind: u8,
        reference_index: u16,
    },
    #[br(magic = 16u8)]
    MethodTypeInfo { descriptor_index: u16 },
    #[br(magic = 17u8)]
    DynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 18u8)]
    DynamicInvokeInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    #[br(magic = 19u8)]
    ModuleInfo { name_index: u16 },
    #[br(magic = 19u8)]
    PackageInfo { name_index: u16 },
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

#[derive(Debug, Clone, Copy)]
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
        let mut res = vec![];

        if (bits & (Self::AccPublic as u16)) != 0 {
            res.push(Self::AccPublic);
        }

        if (bits & (Self::AccFinal as u16)) != 0 {
            res.push(Self::AccFinal);
        }

        if (bits & Self::AccSuper as u16) != 0 {
            res.push(Self::AccSuper);
        }

        if (bits & Self::AccInterface as u16) != 0 {
            res.push(Self::AccInterface);
        }
        if (bits & Self::AccAbstract as u16) != 0 {
            res.push(Self::AccAbstract);
        }
        if (bits & Self::AccSynthetic as u16) != 0 {
            res.push(Self::AccSynthetic);
        }
        if (bits & Self::AccAnnotation as u16) != 0 {
            res.push(Self::AccAnnotation);
        }
        if (bits & Self::AccEnum as u16) != 0 {
            res.push(Self::AccEnum);
        }
        if (bits & Self::AccModule as u16) != 0 {
            res.push(Self::AccModule);
        }

        res
    }
}
