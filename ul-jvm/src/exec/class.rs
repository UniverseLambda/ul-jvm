use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::{anyhow, bail};
use parking_lot::{Mutex, ReentrantMutex, ReentrantMutexGuard};

use crate::{
    class::constant_pool::{
        ConstantDouble, ConstantFieldref, ConstantInterfaceMethodref, ConstantLong,
        ConstantMethodref, LoadableJvmConstant,
    },
    native::jnb::{JnbObject, JnbObjectType},
    types::JvmMethodDescriptor,
};

use super::{
    JvmExecEnv, heap::ObjectRef, interface::Interface, method::Method, runtime_type::RuntimeType,
};

#[derive(Debug)]
pub struct ClassInstance {
    pub class_type: Class,
    pub is_abstract: bool,
    pub parent: Option<Box<ClassInstance>>,
    pub fields: Box<[RuntimeType]>,
    pub jnb: Option<Box<dyn JnbObject>>,
}

#[derive(Debug, Clone)]
pub struct Class(Arc<InnerClass>);

impl Class {
    pub fn instanciate_uninit(&self) -> ClassInstance {
        let class_instance = ClassInstance {
            class_type: self.clone(),
            is_abstract: false,
            parent: self
                .super_class
                .as_ref()
                .map(|c| Box::new(c.instanciate_uninit())),
            fields: self
                .fields
                .iter()
                .map(|f| f.value.clone())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            jnb: self.jnb.as_ref().map(|jnb| jnb.instanciate_uninit()),
        };

        class_instance
    }

    pub fn new_standalone_jnb(
        super_class: Option<Class>,
        interfaces: Vec<Interface>,
        name: Arc<String>,
        constant_pool: ConstantPool,
        jnb_type: Box<dyn JnbObjectType>,
    ) -> Self {
        Self(Arc::new(InnerClass {
            super_class,
            interfaces,
            name,
            constant_pool,
            statics_initialized: AtomicBool::new(false),
            class_impl: ClassImpl::JnbStandalone {
                jnb: jnb_type,
                statics_lock: ReentrantMutex::new(()),
            },
        }))
    }

    pub fn new(
        super_class: Option<Class>,
        interfaces: Vec<Interface>,
        name: Arc<String>,
        constant_pool: ConstantPool,
        static_fields: HashMap<String, ClassField>,
        fields: Box<[ClassField]>,
        methods: HashMap<String, Box<[Method]>>,
        is_abstract: bool,
        jnb_type: Option<Box<dyn JnbObjectType>>,
    ) -> Self {
        Self(Arc::new(InnerClass {
            super_class,
            interfaces,
            name,
            constant_pool,
            statics_initialized: AtomicBool::new(false),
            class_impl: ClassImpl::Normal {
                static_fields: ReentrantMutex::new(
                    static_fields
                        .into_iter()
                        .map(|(k, v)| (k, Mutex::new(v)))
                        .collect(),
                ),
                fields,
                methods,
                is_abstract,
                jnb: jnb_type,
            },
        }))
    }

    pub fn get_static_method(&self, name: &str, ty: JvmMethodDescriptor) -> Option<Method> {
        match &self.class_impl {
            ClassImpl::Normal { methods, .. } => methods.get(name).and_then(|methods| {
                methods
                    .iter()
                    .find(|v| {
                        v.is_static()
                            && v.parameters() == ty.parameter_types
                            && v.ret_type() == &ty.return_type
                    })
                    .cloned()
            }),
            ClassImpl::JnbStandalone { jnb, .. } => {
                jnb.descriptor().static_methods.iter().find_map(|m| {
                    if m.0 == name && m.1 == ty {
                        Some(Method::new_native(
                            m.1.return_type.clone(),
                            m.1.parameter_types.clone(),
                            Arc::new(m.0.to_string()),
                            true,
                        ))
                    } else {
                        None
                    }
                })
            }
        }
    }

    pub fn get_instance_method(&self, name: &str, ty: JvmMethodDescriptor) -> Option<Method> {
        match &self.class_impl {
            ClassImpl::Normal { methods, .. } => methods.get(name).and_then(|methods| {
                methods
                    .iter()
                    .find(|v| {
                        !v.is_static()
                            && v.parameters() == ty.parameter_types
                            && v.ret_type() == &ty.return_type
                    })
                    .cloned()
            }),
            ClassImpl::JnbStandalone { jnb, .. } => {
                jnb.descriptor().static_methods.iter().find_map(|m| {
                    if m.0 == name && m.1 == ty {
                        Some(Method::new_native(
                            m.1.return_type.clone(),
                            m.1.parameter_types.clone(),
                            Arc::new(m.0.to_string()),
                            false,
                        ))
                    } else {
                        None
                    }
                })
            }
        }
    }

    pub fn read_static(&self, name: &String) -> anyhow::Result<RuntimeType> {
        // FIXME: throw an error when the statics are not yet initialized

        self.lock_statics()
            .get(name)
            .ok_or(anyhow!("no static field at {}@{name}", self.name))
    }

    pub fn write_static(&self, name: &String, value: RuntimeType) -> anyhow::Result<()> {
        let lock = self.lock_statics();
        let var_lock = lock
            .get(name)
            .ok_or(anyhow!("no static field at {}@{name}", self.name))?;

        let mut var = var_lock.lock();

        if var.is_final {
            bail!(
                "tried to assign static field {}@{name}, but it is declared as final",
                self.name,
            );
        }

        var.value = value;

        Ok(())
    }

    pub fn lock_statics(&self) -> StaticLock {
        match &self.class_impl {
            ClassImpl::Normal { static_fields, .. } => StaticLock::Normal(static_fields.lock()),
            ClassImpl::JnbStandalone { jnb, statics_lock } => {
                StaticLock::JnbStandalone(jnb.as_ref(), statics_lock.lock())
            }
        }
    }

    pub fn set_initialized_if_needed(&self) -> bool {
        self.0
            .statics_initialized
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }
}

impl AsRef<InnerClass> for Class {
    fn as_ref(&self) -> &InnerClass {
        &self.0
    }
}

impl Deref for Class {
    type Target = InnerClass;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

// impl ObjectBacking for Class {
//     fn as_object(&self, env: &JvmExecEnv) -> ObjectRef {
//         self.backed_instance
//             .get_or_init(|| {
//                 let class = env
//                     .classes
//                     .get(&String::from("java/lang/Class"))
//                     .expect("mandatory class java/lang/Class not loaded")
//                     .clone();

//                 let instance = class.instanciante_special(InternalBacking::Class(self.clone()));

//                 env.heap.store_object(instance)
//             })
//             .new_ref()
//     }
// }

#[derive(Debug, Clone)]
pub struct ConstantPool {
    loadables: HashMap<u16, LoadableJvmConstant>,
    fieldrefs: HashMap<u16, ConstantFieldref>,
    methodrefs: HashMap<u16, ConstantMethodref>,
    interface_methodrefs: HashMap<u16, ConstantInterfaceMethodref>,
}

impl ConstantPool {
    pub fn new(
        loadables: HashMap<u16, LoadableJvmConstant>,
        fieldrefs: HashMap<u16, ConstantFieldref>,
        methodrefs: HashMap<u16, ConstantMethodref>,
        interface_methodrefs: HashMap<u16, ConstantInterfaceMethodref>,
    ) -> Self {
        Self {
            loadables,
            fieldrefs,
            methodrefs,
            interface_methodrefs,
        }
    }

    pub fn get_field_ref(&self, cp_index: u16) -> Option<ConstantFieldref> {
        self.fieldrefs.get(&cp_index).cloned()
    }

    pub fn get_method_ref(&self, cp_index: u16) -> Option<ConstantMethodref> {
        self.methodrefs.get(&cp_index).cloned()
    }

    pub fn get_interface_method_ref(&self, cp_index: u16) -> Option<ConstantInterfaceMethodref> {
        self.interface_methodrefs.get(&cp_index).cloned()
    }

    pub fn get_long(&self, cp_index: u16) -> Option<ConstantLong> {
        self.loadables.get(&cp_index).cloned().and_then(|v| {
            if let LoadableJvmConstant::Long(v) = v {
                Some(v)
            } else {
                None
            }
        })
    }

    pub fn get_double(&self, cp_index: u16) -> Option<ConstantDouble> {
        self.loadables.get(&cp_index).cloned().and_then(|v| {
            if let LoadableJvmConstant::Double(v) = v {
                Some(v)
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub struct InnerClass {
    pub super_class: Option<Class>,
    pub interfaces: Vec<Interface>,
    pub name: Arc<String>,
    pub constant_pool: ConstantPool,
    statics_initialized: AtomicBool,
    class_impl: ClassImpl,
}

#[derive(Debug)]
pub enum ClassImpl {
    Normal {
        static_fields: ReentrantMutex<HashMap<String, Mutex<ClassField>>>,
        fields: Box<[ClassField]>,
        methods: HashMap<String, Box<[Method]>>,
        is_abstract: bool,
        jnb: Option<Box<dyn JnbObjectType>>,
    },
    JnbStandalone {
        jnb: Box<dyn JnbObjectType>,
        statics_lock: ReentrantMutex<()>,
    },
}

#[derive(Debug)]
pub enum StaticLock<'a> {
    Normal(ReentrantMutexGuard<'a, HashMap<String, Mutex<ClassField>>>),
    JnbStandalone(&'a dyn JnbObjectType, ReentrantMutexGuard<'a, ()>),
}

impl<'a> StaticLock<'a> {
    pub fn get(&self, name: &str) -> Option<RuntimeType> {
        match self {
            StaticLock::Normal(guard) => guard.get(name).map(|v| v.lock().value.clone()),
            StaticLock::JnbStandalone(jnb_object_type, _) => {
                // TODO: check if the variable exists
                Some(jnb_object_type.get_static_field(name))
            }
        }
    }

    pub fn set(&self, name: &str, value: RuntimeType) -> anyhow::Result<()> {
        // TODO: check if variable exists and check value type

        match self {
            StaticLock::Normal(guard) => {
                guard.get(name).unwrap().lock().value = value;
            }
            StaticLock::JnbStandalone(jnb_object_type, _) => {
                jnb_object_type.set_static_field(name, value)
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClassField {
    pub name: Arc<String>,
    pub value: RuntimeType,
    pub is_final: bool,
}

pub trait ObjectBacking {
    fn as_object(&self, env: &JvmExecEnv) -> ObjectRef;
}
