use std::{collections::HashMap, ops::Deref, sync::Arc};

use super::{method::Method, runtime_type::RuntimeType};

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
        name: String,
        static_fields: Box<[ClassField]>,
        fields: Box<[ClassField]>,
        methods: HashMap<String, Method>,
        is_abstract: bool,
    ) -> Self {
        Self(Arc::new(InnerClass {
            super_class,
            name,
            static_fields,
            fields,
            methods,
            is_abstract,
        }))
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
pub struct InnerClass {
    pub super_class: Option<Class>,
    pub name: String,
    pub static_fields: Box<[ClassField]>,
    pub fields: Box<[ClassField]>,
    pub methods: HashMap<String, Method>,
    pub is_abstract: bool,
}

#[derive(Debug, Clone)]
pub struct ClassField {
    pub name: Arc<String>,
    pub value: RuntimeType,
    pub is_final: bool,
}
