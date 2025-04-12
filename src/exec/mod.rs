use std::{
    collections::{HashMap, HashSet},
    iter::once,
    sync::Arc,
};

use class::{Class, ClassField};
use either::Either;
use heap::JvmHeap;
use interface::Interface;
use method::Method;
use runtime_type::RuntimeType;
use thread::JvmThread;

use crate::{
    class::{JvmClass, JvmUnit, JvmUnitField, JvmUnitMethod, JvmUnitType},
    types::JvmTypeDescriptor,
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
    pub interfaces: HashMap<String, Interface>,
    pub heap: JvmHeap,
    pub threads: Vec<JvmThread>,
    pub start_class: Option<Class>,
    pub code: Vec<u8>,

    pub partial_classes: Vec<PartialClass>,
    pub required_units: Vec<String>,
}

impl JvmExecEnv {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            interfaces: HashMap::new(),
            heap: JvmHeap {},
            threads: Vec::new(),
            start_class: None,
            code: Vec::new(),
            partial_classes: Vec::new(),
            required_units: Vec::new(),
        }
    }

    pub fn missing_units(&self) -> HashSet<String> {
        let mut res = self
            .partial_classes
            .iter()
            .map(|c| c.missing_unit_names())
            .fold(HashSet::new(), |mut v, next| {
                v.extend(next);
                v
            });

        res.extend(self.required_units.iter().cloned());

        res
    }

    pub fn add_unit(&mut self, jvm_unit: JvmUnit) -> bool {
        let class_name = jvm_unit.this_class.name.convert_to_string();

        let parse_field = |f: &JvmUnitField| ClassField {
            name: Arc::new(f.name.convert_to_string()),
            value: f
                .is_static
                .then_some(())
                .and_then(|_| f.constant_value.clone())
                .map(|cv| RuntimeType::from(cv))
                .unwrap_or(RuntimeType::default_of(&f.ty)),
            is_final: f.is_final,
        };

        for field in jvm_unit.fields.iter() {
            if let JvmTypeDescriptor::Class(c) = &field.ty {
                if c != &class_name {
                    self.required_units.push(c.clone());
                }
            }
        }

        for JvmUnitMethod { descriptor, .. } in jvm_unit.methods.iter() {
            for ty in descriptor
                .parameter_types
                .iter()
                .chain(once(descriptor.return_type.as_ref()).filter_map(|v| v))
            {
                if let JvmTypeDescriptor::Class(c) = ty {
                    if c != &class_name {
                        self.required_units.push(c.clone());
                    }
                }
            }
        }

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
            .filter(|f| f.is_static)
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
                        self.code.extend_from_slice(&m.code.unwrap().code);
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
                self.partial_classes.push(PartialClass {
                    super_class: jvm_unit
                        .super_class
                        .map(|s| Either::Left(s.name.convert_to_string())),
                    name: class_name,
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
            JvmUnitType::Interface(_) => {
                self.interfaces.insert(
                    class_name.clone(),
                    Interface::new(class_name, static_fields),
                );
            }
            JvmUnitType::Record(_) => todo!(),
            JvmUnitType::Module(_) => (), // TODO: Modules
        }

        self.try_complete()
    }

    fn try_complete(&mut self) -> bool {
        loop {
            if !self.partial_classes.is_empty() {
                break;
            }

            let last_partial_count = self.partial_classes.len();
            let mut still_incomplete = vec![];

            for (idx, content) in self.partial_classes.drain(..).enumerate() {
                match content.try_complete(&self.classes, &self.interfaces) {
                    Either::Left(incomplete) => still_incomplete.push(incomplete),
                    Either::Right(complete) => {
                        self.classes.insert(complete.name.clone(), complete.clone());

                        if idx == 0 && self.start_class.is_none() {
                            self.start_class = Some(complete);
                        }
                    }
                }
            }

            self.partial_classes = still_incomplete;

            if self.partial_classes.len() == last_partial_count {
                break;
            }
        }

        let mut still_missing = vec![];

        for missing in self.required_units.drain(..) {
            if !self.classes.contains_key(&missing) && !self.interfaces.contains_key(&missing) {
                still_missing.push(missing);
            }
        }

        self.required_units = still_missing;
        self.partial_classes.is_empty() && self.required_units.is_empty()
    }
}

// pub struct JvmExecEnvBuilder {
//     pub classes: HashMap<String, Class>,
//     pub interfaces: HashMap<String, Interface>,
//     pub start_class: Option<String>,
//     pub code: Vec<u8>,

//     pub partial_classes: Vec<PartialClass>,
// }

// impl JvmExecEnvBuilder {
// pub fn next_missing_class(&mut self) -> anyhow::Result<JvmExecEnv>

// pub fn build(&mut self) -> anyhow::Result<JvmExecEnv> {
//     let mut to_treat: Vec<PartialClass> = self.partial_classes.drain(..).collect();

//     loop {
//         if to_treat.is_empty() {
//             break;
//         }

//         let last_partial_count = to_treat.len();
//         let mut still_incomplete = vec![];

//         for content in to_treat {
//             match content.try_complete(&self.classes, &self.interfaces) {
//                 Either::Left(incomplete) => still_incomplete.push(incomplete),
//                 Either::Right(complete) => {
//                     self.classes.insert(complete.name.clone(), complete);
//                 }
//             }
//         }

//         if still_incomplete.len() == last_partial_count {
//             let missing_names = still_incomplete
//                 .into_iter()
//                 .map(|c| c.name)
//                 .collect::<Vec<_>>();

//             bail!("Unable to resolve some classes: {missing_names:?}",);
//         }

//         to_treat = still_incomplete;
//     }

//     Ok(JvmExecEnv {
//         classes: self.classes.drain().collect(),
//         heap: JvmHeap::new(),
//         threads: vec![],
//         start_class: todo!(),
//     })
// }
// }

pub struct PartialClass {
    super_class: Option<Either<String, Class>>,
    name: String,
    static_fields: Box<[ClassField]>,
    fields: Box<[ClassField]>,
    methods: HashMap<String, Method>,
    interfaces: Vec<Either<String, Interface>>,
    is_abstract: bool,
}

impl PartialClass {
    pub fn missing_unit_names(&self) -> Vec<String> {
        let mut missings = vec![];

        if let Some(Either::Left(name)) = self.super_class.as_ref() {
            missings.push(name.to_string());
        }

        for partial_interface in self.interfaces.iter() {
            if let Either::Left(name) = partial_interface {
                missings.push(name.to_string());
            }
        }

        missings
    }

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
