use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, atomic::AtomicBool},
};

use anyhow::{anyhow, bail};
use parking_lot::{Mutex, ReentrantMutex};

use crate::{
    class::constant_pool::{
        ConstantDouble, ConstantFieldref, ConstantInterfaceMethodref, ConstantLong,
        ConstantMethodref, LoadableJvmConstant,
    },
    types::JvmMethodDescriptor,
};

use super::{interface::Interface, method::Method, runtime_type::RuntimeType};

#[derive(Debug, Clone)]
pub struct ClassInstance {
    pub class_type: Class,
    pub is_abstract: bool,
    pub parent: Option<Box<ClassInstance>>,
    pub fields: Box<[RuntimeType]>,
}

#[derive(Debug, Clone)]
pub struct UninitClassInstance(ClassInstance);

impl UninitClassInstance {
    pub fn assume_init(self) -> ClassInstance {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Class(Arc<InnerClass>);

impl Class {
    pub fn instanciate_uninit(&self) -> UninitClassInstance {
        let class_instance = ClassInstance {
            class_type: self.clone(),
            is_abstract: false,
            parent: self
                .super_class
                .as_ref()
                .map(|c| Box::new(c.instanciate_uninit().assume_init())),
            fields: self
                .fields
                .iter()
                .map(|f| f.value.clone())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        };

        UninitClassInstance(class_instance)
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
    ) -> Self {
        Self(Arc::new(InnerClass {
            super_class,
            interfaces,
            name,
            constant_pool,
            static_fields: ReentrantMutex::new(
                static_fields
                    .into_iter()
                    .map(|(k, v)| (k, Mutex::new(v)))
                    .collect(),
            ),
            statics_initialized: AtomicBool::new(false),
            fields,
            methods,
            is_abstract,
        }))
    }

    pub fn get_method(&self, name: &String, ty: JvmMethodDescriptor) -> Option<&Method> {
        self.methods.get(name).and_then(|methods| {
            methods
                .iter()
                .find(|v| v.parameters() == ty.parameter_types && v.ret_type() == &ty.return_type)
        })
    }

    pub fn read_static(&self, name: &String) -> anyhow::Result<RuntimeType> {
        // FIXME: throw an error when the statics are not yet initialized

        self.0
            .static_fields
            .lock()
            .get(name)
            .map(|s| s.lock().value.clone())
            .ok_or(anyhow!("no static field at {}@{name}", self.name))
    }

    pub fn write_static(&self, name: &String, value: RuntimeType) -> anyhow::Result<()> {
        let lock = self.0.static_fields.lock();
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
    static_fields: ReentrantMutex<HashMap<String, Mutex<ClassField>>>,
    statics_initialized: AtomicBool,
    pub fields: Box<[ClassField]>,
    pub methods: HashMap<String, Box<[Method]>>,
    pub is_abstract: bool,
}

#[derive(Debug, Clone)]
pub struct ClassField {
    pub name: Arc<String>,
    pub value: RuntimeType,
    pub is_final: bool,
}
