use binrw::BinRead;
use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

pub mod attributes;
mod constant_pool;
mod modified_utf8;

pub use constant_pool::*;
pub use modified_utf8::*;

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
#[derive(Debug, Clone, BinRead, Serialize)]
#[br(magic = 0xCAFEBABEu32, big)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    #[br(args(constant_pool_count), parse_with = parse_constant_pool)]
    pub constant_pool: Vec<ConstantPoolInfo>,
    #[br(map = |bits: u16| ClassAccessFlags::from_bits(bits))]
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

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
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

#[derive(Debug, Clone, BinRead, Serialize)]
pub struct FieldInfo {
    #[br(map = |bits: u16| FieldAccessFlags::from_bits(bits))]
    pub access_flags: Vec<FieldAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
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

#[derive(Debug, Clone, BinRead, Serialize)]
pub struct MethodInfo {
    #[br(map = |bits: u16| MethodAccessFlags::from_bits(bits))]
    pub access_flags: Vec<MethodAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    #[br(count = attributes_count)]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
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
