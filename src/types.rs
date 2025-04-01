pub type JvmByte = i8;
pub type JvmShort = i16;
pub type JvmInt = i32;
pub type JvmLong = i64;
pub type JvmChar = u16;

pub type JvmFloat = f32;
pub type JvmDouble = f64;

// Not really used, but still defined
pub type JvmBoolean = bool;

pub type JvmAddress = usize;

pub struct JvmReturnAddress(JvmAddress);

pub struct JvmRefClass(Option<JvmAddress>);
pub struct JvmRefInterface(Option<JvmAddress>);
pub struct JvmRefArray(Option<JvmAddress>);
