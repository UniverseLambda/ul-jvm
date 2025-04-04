use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use class::{Class, ClassField};
use heap::JvmHeap;
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
    pub classes: Vec<Class>,
    pub heap: JvmHeap,
    pub threads: Vec<JvmThread>,
    pub start_class: Class,
    pub loadable_constant_pool: HashMap<u16, LoadableJvmConstant>,
}

impl JvmExecEnv {
    pub fn builder() -> JvmExecEnvBuilder {
        JvmExecEnvBuilder {
            classes: vec![],
            start_class: None,
            code: vec![],
            classes_to_resolve: HashSet::new(),
        }
    }
}

pub struct JvmExecEnvBuilder {
    pub classes: Vec<Class>,
    pub start_class: Option<Class>,

    pub code: Vec<u8>,

    pub classes_to_resolve: HashSet<String>,
    // pub interfaces_to_resolve: HashSet<String>,
}

impl JvmExecEnvBuilder {
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

        let methods = jvm_unit.methods.iter().cloned().map(|m| {
            if m.is_abstract {
                Method::new_abstract(
                    m.descriptor.return_type,
                    m.descriptor.parameter_types,
                    m.name.convert_to_string(),
                    m.is_static,
                )
            } else if m.is_native {
                Method::new_native(
                    m.descriptor.return_type,
                    m.descriptor.parameter_types,
                    m.name.convert_to_string(),
                    m.is_static,
                )
            } else {
                let cp_start = self.code.len();
                self.code.extend_from_slice(&m.code.code);
                let cp_end = self.code.len();

                Method::new_normal(
                    m.descriptor.return_type,
                    m.descriptor.parameter_types,
                    m.name.convert_to_string(),
                    m.is_static,
                    cp_start,
                    cp_end,
                )
            }
        });

        match jvm_unit.unit_type {
            JvmUnitType::Class(JvmClass { is_abstract, .. }) => {
                self.classes.push(Class::new(
                    None,
                    jvm_unit.this_class.name.convert_to_string(),
                    static_fields,
                    fields,
                    HashMap::new(),
                    is_abstract,
                ));
            }
            JvmUnitType::Interface(_) => todo!(),
            JvmUnitType::Record(_) => todo!(),
            JvmUnitType::Module(_) => (), // TODO: Modules
        }

        self
    }
}
