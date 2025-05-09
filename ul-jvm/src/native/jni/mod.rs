use std::sync::LazyLock;

use ul_jni::{
    api::{JNI_VERSION_24, JniInterfaceFunctions},
    ptr::JniPtr,
    types::JniInt,
};

use crate::exec::{JvmExecEnv, heap::ObjectRef};

static JNI_INTERFACE: LazyLock<JniInterfaceFunctions<JvmExecEnv, ObjectRef>> =
    LazyLock::new(|| JniInterfaceFunctions {
        get_version: Some(get_version),
        // define_class: todo!(),
        // find_class: todo!(),
        ..Default::default()
    });

fn get_version(_env: JniPtr<JvmExecEnv>) -> JniInt {
    JNI_VERSION_24
}

// fn find_class(env: JniPtr<JvmExecEnv>, name: FfiStr) -> JniPtr<ClassRef> {
//     // TODO: handle classloaders
//     todo!()
// }
