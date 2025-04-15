use crate::ptr::JniPtr;

// Find ways to enforce hierarchy

pub type JniObject = JniPtr<()>;
pub type JniClass = JniObject;
pub type JniString = JniObject;
pub type JniArray = JniObject;
pub type JniObjectArray = JniArray;
pub type JniBooleanArray = JniArray;
pub type JniByteArray = JniArray;
pub type JniCharArray = JniArray;
pub type JniShortArray = JniArray;
pub type JniIntArray = JniArray;
pub type JniLongArray = JniArray;
pub type JniFloatArray = JniArray;
pub type JniDoubleArray = JniArray;
pub type JniThrowable = JniObject;
