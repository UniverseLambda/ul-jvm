use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

use class::{Class, ClassField};
use either::Either;
use heap::JvmHeap;
use interface::Interface;
use method::{AbstractMethod, Method};
use thread::JvmThread;

use crate::class::{
    JvmClass, JvmUnit, JvmUnitField, JvmUnitType, constant_pool::LoadableJvmConstant,
};

pub mod array;
pub mod class;
pub mod heap;
pub mod interface;
pub mod method;
pub mod runtime_type;
pub mod thread;

pub struct JvmExecEnv {
    pub classes: HashMap<String, Class>,
    pub heap: JvmHeap,
    pub threads: Vec<JvmThread>,
    pub start_class: Class,
    pub loadable_constant_pool: HashMap<u16, LoadableJvmConstant>,
}

impl JvmExecEnv {
    pub fn builder() -> JvmExecEnvBuilder {
        JvmExecEnvBuilder {
            classes: HashMap::new(),
            start_class: None,
            code: vec![],
            interfaces: vec![],
            partial_classes: VecDeque::new(),
        }
    }
}

pub struct JvmExecEnvBuilder {
    pub classes: HashMap<String, Class>,
    pub interfaces: Vec<Interface>,
    pub start_class: Option<String>,
    pub code: Vec<u8>,

    pub partial_classes: VecDeque<PartialClass>,
}

impl JvmExecEnvBuilder {
    pub fn set_start_class(&mut self, name: String) -> &mut Self {
        self.start_class = Some(name);

        self
    }

    pub fn add_unit(&mut self, jvm_unit: JvmUnit) -> &mut Self {
        let parse_field = |f: &JvmUnitField| ClassField {
            name: Arc::new(f.name.convert_to_string()),
            value: f.constant_value.clone().into(),
            is_final: f.is_final,
        };

        let fields = jvm_unit
            .fields
            .iter()
            .filter(|f| !f.is_static)
            .map(parse_field)
            .collect::<Vec<ClassField>>()
            .into_boxed_slice();
        let static_fields = jvm_unit
            .fields
            .iter()
            .map(parse_field)
            .collect::<Vec<ClassField>>()
            .into_boxed_slice();

        let methods = jvm_unit
            .methods
            .iter()
            .cloned()
            .map(|m| {
                let name = m.name.convert_to_string();

                (
                    name.clone(),
                    if m.is_abstract {
                        Method::new_abstract(
                            m.descriptor.return_type,
                            m.descriptor.parameter_types,
                            name,
                            m.is_static,
                        )
                    } else if m.is_native {
                        Method::new_native(
                            m.descriptor.return_type,
                            m.descriptor.parameter_types,
                            name,
                            m.is_static,
                        )
                    } else {
                        let cp_start = self.code.len();
                        self.code.extend_from_slice(&m.code.code);
                        let cp_end = self.code.len();

                        Method::new_normal(
                            m.descriptor.return_type,
                            m.descriptor.parameter_types,
                            name,
                            m.is_static,
                            cp_start,
                            cp_end,
                        )
                    },
                )
            })
            .collect();

        match jvm_unit.unit_type {
            JvmUnitType::Class(JvmClass { is_abstract, .. }) => {
                self.partial_classes.push_back(PartialClass {
                    super_class: jvm_unit
                        .super_class
                        .map(|s| Either::Left(s.name.convert_to_string())),
                    name: jvm_unit.this_class.name.convert_to_string(),
                    static_fields,
                    fields,
                    methods,
                    interfaces: jvm_unit
                        .interfaces
                        .into_iter()
                        .map(|i| Either::Left(i.name.convert_to_string()))
                        .collect(),
                    is_abstract,
                });
            }
            JvmUnitType::Interface(_) => todo!(),
            JvmUnitType::Record(_) => todo!(),
            JvmUnitType::Module(_) => (), // TODO: Modules
        }

        self
    }

    pub fn build(&mut self) -> anyhow::Result<JvmExecEnv> {
        todo!()
    }
}

struct PartialClass {
    super_class: Option<Either<String, Class>>,
    name: String,
    static_fields: Box<[ClassField]>,
    fields: Box<[ClassField]>,
    methods: HashMap<String, Method>,
    interfaces: Vec<Either<String, Interface>>,
    is_abstract: bool,
}

impl PartialClass {
    pub fn try_complete(
        mut self,
        classes: &HashMap<String, Class>,
        interfaces: &HashMap<String, Interface>,
    ) -> Either<PartialClass, Class> {
        if let Some(Either::Left(name)) = self.super_class.as_ref() {
            if let Some(super_class) = classes.get(name).cloned() {
                self.super_class = Some(Either::Right(super_class));
            }
        }

        let mut incomplete_interfaces = 0;

        for partial_interface in self.interfaces.iter_mut() {
            if let Either::Left(name) = partial_interface {
                if let Some(found) = interfaces.get(name).cloned() {
                    *partial_interface = Either::Right(found);
                } else {
                    incomplete_interfaces += 1;
                }
            }
        }

        if self.super_class.as_ref().is_none_or(|s| s.is_right()) && incomplete_interfaces == 0 {
            Either::Right(Class::new(
                self.super_class.map(|s| s.unwrap_right()),
                self.interfaces
                    .into_iter()
                    .map(|i| i.unwrap_right())
                    .collect(),
                self.name,
                self.static_fields,
                self.fields,
                self.methods,
                self.is_abstract,
            ))
        } else {
            Either::Left(self)
        }
    }
}
