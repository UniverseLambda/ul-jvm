use crate::{
    exec::{JvmExecEnv, heap::ObjectRef, runtime_type::RuntimeType, thread::JvmThread},
    native::jnb::{JnbCallInfo, JnbObject, JnbObjectType, JnbObjectTypeDescriptor, jnb_call},
    types::{JvmInt, JvmMethodDescriptor, JvmTypeDescriptor, NativeOptJvmType},
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
        Box::new(Object)
    }

    fn descriptor(&self) -> JnbObjectTypeDescriptor {
        static METHODS: [(&'static str, JvmMethodDescriptor); 2] = [
            (
                "<init>",
                JvmMethodDescriptor {
                    parameter_types: vec![],
                    return_type: None,
                },
            ),
            (
                "hash_code",
                JvmMethodDescriptor {
                    parameter_types: vec![],
                    return_type: Some(JvmTypeDescriptor::Int),
                },
            ),
        ];

        JnbObjectTypeDescriptor {
            full_name: "java/lang/Object",
            fields: &[],
            static_fields: &[],
            methods: &METHODS,
            static_methods: &[],
        }
    }
}

#[derive(Debug)]
pub struct Object;

impl JnbObject for Object {
    #[allow(unused_variables)]
    fn call(
        &self,
        info: JnbCallInfo,
        name: &str,
        args: &[RuntimeType],
    ) -> anyhow::Result<Option<RuntimeType>> {
        match name {
            "<init>" => {
                jnb_call!(self.ctor(info, args))
            }
            "get_class" => {
                jnb_call!(self.get_class(info, args))
            }
            "hash_code" => {
                jnb_call!(self.hash_code(info, args))
            }
            _ => unreachable!(),
        }
    }
}

impl Object {
    pub fn ctor(&self, info: JnbCallInfo) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn hash_code(&self, info: JnbCallInfo) -> anyhow::Result<JvmInt> {
        Ok((self as *const Self) as i32)
    }

    pub fn get_class(&self, info: JnbCallInfo) -> anyhow::Result<ObjectRef> {
        todo!()
    }
}
