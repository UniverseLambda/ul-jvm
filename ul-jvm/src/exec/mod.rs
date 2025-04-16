use std::{
    collections::{HashMap, HashSet},
    iter::once,
    sync::Arc,
};

use class::{Class, ClassField, ConstantPool};
use either::Either;
use heap::JvmHeap;
use interface::Interface;
use log::debug;
use method::Method;
use runtime_type::RuntimeType;

use crate::{
    class::{
        JvmClass, JvmUnit, JvmUnitField, JvmUnitMethod, JvmUnitType,
        constant_pool::{ConstantMethodHandle, LoadableJvmConstant},
    },
    native::jnb::JnbObjectType,
    types::JvmTypeDescriptor,
};

pub mod array;
pub mod class;
pub mod heap;
pub mod interface;
pub mod jpu;
pub mod method;
pub mod runtime_type;
pub mod thread;

#[derive(Default)]
pub struct JvmExecEnv {
    pub classes: HashMap<String, Class>,
    pub interfaces: HashMap<String, Interface>,
    pub heap: JvmHeap,
    // pub threads: Vec<JvmThread>,
    pub start_class: Option<Class>,
    pub code: Vec<u8>,

    pub partial_classes: Vec<PartialClass>,
    pub required_units: HashSet<String>,
    pub differed_units: HashSet<String>,
}

impl JvmExecEnv {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            interfaces: HashMap::new(),
            heap: JvmHeap::new(),
            // threads: Vec::new(),
            start_class: None,
            code: Vec::new(),
            partial_classes: Vec::new(),
            required_units: HashSet::new(),
            differed_units: HashSet::new(),
        }
    }

    pub fn missing_units(&self) -> HashSet<String> {
        let mut res = self
            .partial_classes
            .iter()
            .map(|c| {
                let missing = c.missing_unit_names();
                debug!("Missing unit names for {}: {:?}", c.name, missing);

                missing
            })
            .fold(HashSet::new(), |mut v, next| {
                v.extend(next);
                v
            });

        debug!("required_units: {:?}", self.required_units);

        res.extend(self.required_units.iter().cloned());

        res
    }

    pub fn add_unit(&mut self, jvm_unit: JvmUnit, eager_loading: bool) -> bool {
        let class_name = jvm_unit.this_class.name;

        let parse_field = |f: &JvmUnitField| ClassField {
            name: f.name.clone(),
            value: f
                .is_static
                .then_some(())
                .and_then(|_| f.constant_value.clone())
                .map(RuntimeType::from)
                .unwrap_or(RuntimeType::default_of(&f.ty)),
            is_final: f.is_final,
        };

        for field in jvm_unit.fields.iter() {
            if let JvmTypeDescriptor::Class(c) = &field.ty {
                if c != class_name.as_ref() {
                    self.required_units.insert(c.clone());
                }
            }
        }

        for JvmUnitMethod { descriptor, .. } in jvm_unit.methods.iter() {
            for ty in descriptor
                .parameter_types
                .iter()
                .chain(once(descriptor.return_type.as_ref()).flatten())
            {
                if let JvmTypeDescriptor::Class(c) = ty {
                    if c != class_name.as_ref() {
                        if eager_loading {
                            self.required_units.insert(c.clone());
                        } else {
                            self.differed_units.insert(c.clone());
                        }
                    }
                }
            }
        }

        for constant in &jvm_unit.loadable_constant_pool {
            let v = match constant.1 {
                LoadableJvmConstant::Class(c) => Some(c.name.clone()),
                LoadableJvmConstant::MethodHandle(
                    ConstantMethodHandle::GetField(f)
                    | ConstantMethodHandle::GetStatic(f)
                    | ConstantMethodHandle::PutField(f)
                    | ConstantMethodHandle::PutStatic(f),
                ) => Some(f.class.name.clone()),
                LoadableJvmConstant::MethodHandle(
                    ConstantMethodHandle::NewInvokeSpecial(i)
                    | ConstantMethodHandle::InvokeVirtual(i)
                    | ConstantMethodHandle::InvokeSpecial(Either::Left(i))
                    | ConstantMethodHandle::InvokeStatic(Either::Left(i)),
                ) => Some(i.class.name.clone()),
                LoadableJvmConstant::MethodHandle(
                    ConstantMethodHandle::InvokeSpecial(Either::Right(i))
                    | ConstantMethodHandle::InvokeStatic(Either::Right(i)),
                ) => Some(i.class.name.clone()),
                _ => None,
            };

            if let Some(c) = v {
                if c != class_name {
                    if eager_loading {
                        self.required_units.insert(c.as_ref().clone());
                    } else {
                        self.differed_units.insert(c.as_ref().clone());
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
            .map(|f| (f.name.as_ref().clone(), f))
            .collect::<HashMap<String, ClassField>>();

        let mut methods: HashMap<String, Vec<Method>> = HashMap::new();

        for m in jvm_unit.methods.iter().cloned() {
            let name = m.name;
            let entry = methods.entry(name.as_ref().clone()).or_default();

            entry.push(if m.is_abstract {
                Method::new_abstract(m.descriptor.return_type, m.descriptor.parameter_types, name)
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
                    m.local_count,
                )
            });
        }

        match jvm_unit.unit_type {
            JvmUnitType::Class(JvmClass { is_abstract, .. }) => {
                self.partial_classes.push(PartialClass {
                    super_class: jvm_unit
                        .super_class
                        .map(|s| Either::Left(s.name.as_ref().clone())),
                    name: class_name,
                    constant_pool: ConstantPool::new(
                        jvm_unit.loadable_constant_pool,
                        jvm_unit.field_refs,
                        jvm_unit.method_refs,
                        jvm_unit.interface_method_refs,
                    ),
                    static_fields,
                    fields,
                    methods: methods
                        .drain()
                        .map(|(k, v)| (k, v.into_boxed_slice()))
                        .collect(),
                    interfaces: jvm_unit
                        .interfaces
                        .into_iter()
                        .map(|i| Either::Left(i.name))
                        .collect(),
                    is_abstract,
                    jnb: None,
                });
            }
            JvmUnitType::Interface(_) => {
                self.interfaces.insert(
                    class_name.as_ref().clone(),
                    Interface::new(class_name, static_fields),
                );
            }
            JvmUnitType::Record(mut rec) => {
                self.partial_classes.push(PartialClass {
                    super_class: jvm_unit
                        .super_class
                        .map(|s| Either::Left(s.name.as_ref().clone())),
                    name: class_name,
                    constant_pool: ConstantPool::new(
                        jvm_unit.loadable_constant_pool,
                        jvm_unit.field_refs,
                        jvm_unit.method_refs,
                        jvm_unit.interface_method_refs,
                    ),
                    static_fields,
                    fields: rec
                        .components
                        .drain(..)
                        // TODO: check if this is the proper way to handle records components (or if should do the same as with classes)
                        .map(|c| ClassField {
                            value: RuntimeType::default_of(&c.descriptor),
                            name: c.name,
                            is_final: true,
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                    methods: methods
                        .drain()
                        .map(|(k, v)| (k, v.into_boxed_slice()))
                        .collect(),
                    interfaces: jvm_unit
                        .interfaces
                        .into_iter()
                        .map(|i| Either::Left(i.name))
                        .collect(),
                    is_abstract: false,
                    jnb: None,
                });
            }
            JvmUnitType::Module(_) => (), // TODO: Modules
        }

        self.try_complete()
    }

    fn try_complete(&mut self) -> bool {
        loop {
            if self.partial_classes.is_empty() {
                break;
            }

            let last_partial_count = self.partial_classes.len();
            let mut still_incomplete = vec![];

            for (idx, content) in self.partial_classes.drain(..).enumerate() {
                match content.try_complete(&self.classes, &self.interfaces) {
                    Either::Left(incomplete) => still_incomplete.push(incomplete),
                    Either::Right(complete) => {
                        self.classes
                            .insert(complete.name.as_ref().clone(), complete.clone());

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

        let mut still_missing = HashSet::new();

        for missing in self.required_units.drain() {
            if !self.classes.contains_key(&missing)
                && !self.interfaces.contains_key(&missing)
                && !self
                    .partial_classes
                    .iter()
                    .any(|c| c.name.as_ref() == &missing)
            {
                still_missing.insert(missing);
            }
        }

        self.required_units = still_missing;

        self.partial_classes.is_empty() && self.required_units.is_empty()
    }
}

#[derive(Debug)]
pub struct PartialClass {
    super_class: Option<Either<String, Class>>,
    name: Arc<String>,
    constant_pool: ConstantPool,
    static_fields: HashMap<String, ClassField>,
    fields: Box<[ClassField]>,
    methods: HashMap<String, Box<[Method]>>,
    interfaces: Vec<Either<Arc<String>, Interface>>,
    is_abstract: bool,
    jnb: Option<Box<dyn JnbObjectType>>,
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
                if let Some(found) = interfaces.get(name.as_ref()).cloned() {
                    *partial_interface = Either::Right(found);
                } else {
                    incomplete_interfaces += 1;
                }
            }
        }

        if self.super_class.as_ref().is_none_or(Either::is_right) && incomplete_interfaces == 0 {
            Either::Right(Class::new(
                self.super_class.map(|s| s.unwrap_right()),
                self.interfaces
                    .into_iter()
                    .map(|i| i.unwrap_right())
                    .collect(),
                self.name,
                self.constant_pool,
                self.static_fields,
                self.fields,
                self.methods,
                self.is_abstract,
                self.jnb,
            ))
        } else {
            Either::Left(self)
        }
    }
}
