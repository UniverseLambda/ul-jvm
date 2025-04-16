use crate::{
    exec::runtime_type::RuntimeType,
    native::jnb::{JnbMethod, JnbObject, JnbObjectType, JnbObjectTypeDescriptor},
    types::JvmInt,
};

#[derive(Debug)]
pub struct ObjectType;

impl JnbObjectType for ObjectType {
    fn clinit(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn is_standalone(&self) -> bool {
        false
    }

    fn instanciate_uninit(&self) -> Box<dyn crate::native::jnb::JnbObject> {
        todo!()
    }

    fn call_static(&self, _: &str, _: &[RuntimeType]) -> anyhow::Result<Option<RuntimeType>> {
        unreachable!()
    }

    fn descriptor(&self) -> JnbObjectTypeDescriptor {
        JnbObjectTypeDescriptor {
            full_name: "java/lang/Object",
            fields: &[],
            static_fields: &[],
            methods: &[(
                "hash_code",
                JnbMethod::<Object>::as_descriptor(&Object::hash_code),
            )],
            static_methods: &[],
        }
    }
}

#[derive(Debug)]
pub struct Object;

impl JnbObject for Object {
    fn call(&self, name: &str, args: &[RuntimeType]) -> anyhow::Result<Option<RuntimeType>> {
        todo!()
    }
}

impl Object {
    pub fn hash_code<'a>(&'a self) -> anyhow::Result<JvmInt> {
        Ok((self as *const Self) as i32)
    }
}
