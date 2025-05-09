use std::collections::HashMap;
use std::io::Cursor;
use std::mem::swap;
use std::str::FromStr;

use anyhow::{Result, anyhow, bail};
use binrw::BinRead;
use either::Either;
use log::{debug, trace, warn};
use serde::Serialize;

use super::constant_pool::{
    ConstantClass, ConstantFieldref, ConstantInterfaceMethodref, ConstantJvmUtf8,
    ConstantMethodHandle, ConstantMethodref, DynamicInvoke, LoadableJvmConstant,
};
use super::parser::{self, ClassAccessFlags, ClassFile, ConstantPoolInfo, MethodKind};
use super::{JvmUnitField, JvmUnitMethod, get_class, get_name_and_type, get_string};

use crate::types::{JvmMethodDescriptor, JvmTypeDescriptor};

#[derive(Debug, Clone, Serialize)]
pub struct JvmUnit {
    pub minor_version: u16,
    pub major_version: u16,
    pub this_class: ConstantClass,
    pub super_class: Option<ConstantClass>,
    pub loadable_constant_pool: HashMap<u16, LoadableJvmConstant>,
    pub is_public: bool,
    pub is_synthetic: bool,
    pub is_deprecated: bool,
    pub unit_type: JvmUnitType,
    pub interfaces: Vec<ConstantClass>,
    pub fields: Vec<JvmUnitField>,
    pub methods: Vec<JvmUnitMethod>,
    pub field_refs: HashMap<u16, ConstantFieldref>,
    pub method_refs: HashMap<u16, ConstantMethodref>,
    pub interface_method_refs: HashMap<u16, ConstantInterfaceMethodref>,
}

#[derive(Debug, Clone, Serialize)]
pub enum JvmUnitType {
    Class(JvmClass),
    Module(JvmModule),
    Interface(JvmInterface),
    Record(JvmRecord),
}

#[derive(Debug, Clone, Serialize)]
pub struct JvmClass {
    pub is_abstract: bool,
    pub is_final: bool,
    pub is_super: bool,
    pub is_enum: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct JvmModule {}

#[derive(Debug, Clone, Serialize)]
pub struct JvmInterface {
    pub is_annotation: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct JvmRecord {
    pub is_abstract: bool,
    // pub is_final: bool, /* ALWAYS TRUE */
    pub is_super: bool,
    pub components: Vec<JvmRecordComponent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JvmRecordComponent {
    pub name: ConstantJvmUtf8,
    pub descriptor: JvmTypeDescriptor,
}

#[derive(Debug, Clone, Serialize)]
pub enum JvmVisibility {
    Public,
    Private,
    Protected,
}

impl JvmUnit {
    pub fn from_class_file(class_file: ClassFile) -> Result<Self> {
        let minor_version = class_file.minor_version;
        let major_version = class_file.major_version;

        let mut jvm_strings: HashMap<u16, ConstantJvmUtf8> = HashMap::new(); // OK
        let mut field_refs: HashMap<u16, ConstantFieldref> = HashMap::new(); // OK
        let mut method_refs: HashMap<u16, ConstantMethodref> = HashMap::new(); // OK
        let mut dynamic_invokes: HashMap<u16, DynamicInvoke> = HashMap::new(); // OK
        let mut interface_method_refs: HashMap<u16, ConstantInterfaceMethodref> = HashMap::new(); // OK
        let mut loadable_constant_pool: HashMap<u16, LoadableJvmConstant> = HashMap::new(); // OK
        let mut name_and_types: HashMap<u16, (u16, u16)> = HashMap::new();

        debug!(
            "CONSTANT POOL - PASS 0 ({} to treat)",
            class_file.constant_pool.len()
        );

        let mut processing = vec![];
        let mut indexed_constant_pool = class_file
            .constant_pool
            .into_iter()
            .enumerate()
            .map(|(idx, v)| (idx as u16, v))
            .collect();

        swap(&mut processing, &mut indexed_constant_pool);

        for (idx, constant) in processing.drain(..) {
            trace!("- [{idx}] {}", <&'static str>::from(&constant));
            match constant {
                ConstantPoolInfo::Utf8 { bytes, .. } => {
                    jvm_strings.insert(idx, ConstantJvmUtf8::new(bytes.convert_to_string()));
                }
                ConstantPoolInfo::Integer { bytes } => {
                    loadable_constant_pool.insert(idx, LoadableJvmConstant::Integer(bytes));
                }
                ConstantPoolInfo::Long {
                    high_bytes,
                    low_bytes,
                } => {
                    loadable_constant_pool.insert(
                        idx,
                        LoadableJvmConstant::Long(
                            (((high_bytes as u64) << 32) | (low_bytes as u64)) as i64,
                        ),
                    );
                }
                ConstantPoolInfo::Float { bytes } => {
                    loadable_constant_pool.insert(idx, LoadableJvmConstant::Float(bytes));
                }
                ConstantPoolInfo::Double {
                    high_bytes,
                    low_bytes,
                } => {
                    loadable_constant_pool.insert(
                        idx,
                        LoadableJvmConstant::Double(f64::from_bits(
                            ((high_bytes as u64) << 32) | (low_bytes as u64),
                        )),
                    );
                }
                ConstantPoolInfo::NameAndType {
                    name_index,
                    descriptor_index,
                } => {
                    name_and_types.insert(idx, (name_index, descriptor_index));
                }
                v => indexed_constant_pool.push((idx, v)),
            }
        }

        trace!("- jvm_strings: {jvm_strings:#?}");
        trace!("- loadable_constant_pool: {loadable_constant_pool:#?}");
        debug!("CONSTANT POOL - PASS 1",);

        swap(&mut processing, &mut indexed_constant_pool);

        for (idx, constant) in processing.drain(..) {
            trace!("- [{idx}] {}", <&'static str>::from(&constant));

            match constant {
                ConstantPoolInfo::String { string_index } => {
                    loadable_constant_pool.insert(
                        idx,
                        LoadableJvmConstant::String(get_string(&jvm_strings, &string_index)?),
                    );
                }
                ConstantPoolInfo::Class { name_index } => {
                    loadable_constant_pool.insert(
                        idx,
                        LoadableJvmConstant::Class(ConstantClass {
                            name: get_string(&jvm_strings, &name_index)?,
                        }),
                    );
                }
                ConstantPoolInfo::MethodType { descriptor_index } => {
                    let descriptor = JvmMethodDescriptor::from_str(&get_string(
                        &jvm_strings,
                        &descriptor_index,
                    )?)?;

                    loadable_constant_pool
                        .insert(idx, LoadableJvmConstant::MethodType { descriptor });
                }
                ConstantPoolInfo::Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => {
                    let (name_idx, descriptor_idx) =
                        get_name_and_type(&name_and_types, &name_and_type_index)?;

                    let name = get_string(&jvm_strings, &name_idx)?;
                    let descriptor =
                        JvmTypeDescriptor::from_str(&get_string(&jvm_strings, &descriptor_idx)?)?;

                    loadable_constant_pool.insert(
                        idx,
                        LoadableJvmConstant::Dynamic {
                            bootstrap_method_attr_index,
                            name,
                            ty: descriptor,
                        },
                    );
                }
                ConstantPoolInfo::DynamicInvoke {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => {
                    let (name_idx, descriptor_idx) =
                        get_name_and_type(&name_and_types, &name_and_type_index)?;

                    let name = get_string(&jvm_strings, &name_idx)?;
                    let descriptor =
                        JvmMethodDescriptor::from_str(&get_string(&jvm_strings, &descriptor_idx)?)?;

                    dynamic_invokes.insert(
                        idx,
                        DynamicInvoke {
                            bootstrap_method_attr_index,
                            name,
                            ty: descriptor,
                        },
                    );
                }
                v => indexed_constant_pool.push((idx, v)),
            }
        }

        trace!("- loadable_constant_pool: {loadable_constant_pool:#?}");
        trace!("- dynamic_invokes: {dynamic_invokes:#?}");
        debug!("CONSTANT POOL - PASS 2");

        swap(&mut processing, &mut indexed_constant_pool);

        for (idx, constant) in processing.drain(..) {
            trace!("- [{idx}] {}", <&'static str>::from(&constant));

            match constant {
                ConstantPoolInfo::Fieldref {
                    class_index,
                    name_and_type_index,
                } => {
                    let class = get_class(&loadable_constant_pool, &class_index)?;
                    let (name_idx, descriptor_idx) =
                        get_name_and_type(&name_and_types, &name_and_type_index)?;

                    let name = get_string(&jvm_strings, &name_idx)?;
                    let descriptor =
                        JvmTypeDescriptor::from_str(&get_string(&jvm_strings, &descriptor_idx)?)?;

                    field_refs.insert(
                        idx,
                        ConstantFieldref {
                            class,
                            name,
                            ty: descriptor,
                        },
                    );
                }
                v @ ConstantPoolInfo::Methodref {
                    class_index,
                    name_and_type_index,
                }
                | v @ ConstantPoolInfo::InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                } => {
                    let class = get_class(&loadable_constant_pool, &class_index)?;
                    let (name_idx, descriptor_idx) =
                        get_name_and_type(&name_and_types, &name_and_type_index)?;

                    let name = get_string(&jvm_strings, &name_idx)?;
                    let descriptor =
                        JvmMethodDescriptor::from_str(&get_string(&jvm_strings, &descriptor_idx)?)?;

                    match v {
                        ConstantPoolInfo::Methodref { .. } => {
                            method_refs.insert(
                                idx,
                                ConstantMethodref {
                                    class,
                                    name,
                                    ty: descriptor,
                                },
                            );
                        }
                        ConstantPoolInfo::InterfaceMethodref { .. } => {
                            interface_method_refs.insert(
                                idx,
                                ConstantInterfaceMethodref {
                                    class,
                                    name,
                                    ty: descriptor,
                                },
                            );
                        }
                        _ => unreachable!(),
                    }
                }
                v => indexed_constant_pool.push((idx, v)),
            }
        }

        trace!("- field_refs: {field_refs:#?}");
        trace!("- method_refs: {method_refs:#?}");
        trace!("- interface_method_refs: {interface_method_refs:#?}");

        debug!("CONSTANT POOL - PASS 3");

        for (idx, constant) in indexed_constant_pool.drain(..) {
            trace!("- [{idx}] {}", <&'static str>::from(&constant));

            if let ConstantPoolInfo::MethodHandle {
                reference_kind,
                reference_index,
            } = constant
            {
                let res = match reference_kind {
                    v @ MethodKind::GetField
                    | v @ MethodKind::GetStatic
                    | v @ MethodKind::PutField
                    | v @ MethodKind::PutStatic => {
                        let field_ref = field_refs
                            .get(&reference_index)
                            .ok_or_else(|| {
                                anyhow!("no FieldRef found at {reference_index} in constant pool (for {v:#?})")
                            })?
                            .clone();

                        match v {
                            MethodKind::GetField => ConstantMethodHandle::GetField(field_ref),
                            MethodKind::GetStatic => ConstantMethodHandle::GetStatic(field_ref),
                            MethodKind::PutField => ConstantMethodHandle::PutField(field_ref),
                            MethodKind::PutStatic => ConstantMethodHandle::PutStatic(field_ref),
                            _ => unreachable!(),
                        }
                    }
                    v @ MethodKind::InvokeVirtual | v @ MethodKind::NewInvokeSpecial => {
                        let method_ref = method_refs
                            .get(&reference_index)
                            .ok_or_else(|| {
                                anyhow!("no MethodRef found at {reference_index} in constant pool")
                            })?
                            .clone();

                        match v {
                            MethodKind::InvokeVirtual => {
                                ConstantMethodHandle::InvokeVirtual(method_ref)
                            }
                            MethodKind::NewInvokeSpecial => {
                                ConstantMethodHandle::NewInvokeSpecial(method_ref)
                            }
                            _ => unreachable!(),
                        }
                    }
                    v @ MethodKind::InvokeStatic | v @ MethodKind::InvokeSpecial => {
                        let method_ref = method_refs.get(&reference_index).cloned();
                        let interface_method_ref =
                            interface_method_refs.get(&reference_index).cloned();

                        let res = if let Some(zarma) = method_ref {
                            Either::Left(zarma)
                        } else if let Some(zarma) = interface_method_ref {
                            Either::Right(zarma)
                        } else {
                            bail!(
                                "no MethodRef nor InterfaceMethodRef found at {reference_index} in constant pool"
                            );
                        };

                        match v {
                            MethodKind::InvokeStatic => ConstantMethodHandle::InvokeStatic(res),
                            MethodKind::InvokeSpecial => ConstantMethodHandle::InvokeSpecial(res),
                            _ => unreachable!(),
                        }
                    }
                    MethodKind::InvokeInterface => {
                        let interface_method_ref = interface_method_refs
                            .get(&reference_index)
                            .cloned()
                            .ok_or_else(|| {
                                anyhow!("no InterfaceMethodRef found at {reference_index} in constant pool")
                            })?;

                        ConstantMethodHandle::InvokeInterface(interface_method_ref)
                    }
                };

                loadable_constant_pool.insert(idx, LoadableJvmConstant::MethodHandle(res));
            }
        }

        trace!("- loadable_constant_pool: {loadable_constant_pool:#?}");
        debug!("== END OF CONSTANT POOL TABLE ==");

        let this_class = get_class(&loadable_constant_pool, &class_file.this_class)?;
        let super_class = if class_file.super_class == 0 {
            None
        } else {
            Some(get_class(&loadable_constant_pool, &class_file.super_class)?)
        };

        let is_public = class_file.access_flags.contains(&ClassAccessFlags::Public);
        let is_final = class_file.access_flags.contains(&ClassAccessFlags::Final);
        let is_super = class_file.access_flags.contains(&ClassAccessFlags::Super);
        let is_interface = class_file
            .access_flags
            .contains(&ClassAccessFlags::Interface);
        let is_abstract = class_file
            .access_flags
            .contains(&ClassAccessFlags::Abstract);
        let mut is_synthetic = class_file
            .access_flags
            .contains(&ClassAccessFlags::Synthetic);
        let is_annotation = class_file
            .access_flags
            .contains(&ClassAccessFlags::Annotation);
        let is_enum = class_file.access_flags.contains(&ClassAccessFlags::Enum);
        let is_module = class_file.access_flags.contains(&ClassAccessFlags::Module);

        let mut unit_type = if is_interface {
            JvmUnitType::Interface(JvmInterface { is_annotation })
        } else if is_module {
            JvmUnitType::Module(JvmModule {})
        } else {
            JvmUnitType::Class(JvmClass {
                is_abstract,
                is_final,
                is_super,
                is_enum,
            })
        };

        let mut interfaces = vec![];
        for interface_index in class_file.interfaces.iter() {
            interfaces.push(get_class(&loadable_constant_pool, interface_index)?);
        }

        let mut fields = vec![];
        for info in class_file.fields.iter() {
            fields.push(JvmUnitField::from_class_file(
                info,
                &jvm_strings,
                &loadable_constant_pool,
            )?)
        }

        let mut methods = vec![];
        for info in class_file.methods.iter() {
            methods.push(JvmUnitMethod::from_class_file(
                info,
                &jvm_strings,
                &loadable_constant_pool,
            )?)
        }

        let mut is_deprecated = false;

        for attribute in class_file.attributes.iter() {
            let attribute_name = get_string(&jvm_strings, &attribute.attribute_name_index)?;

            match attribute_name.as_str() {
                "Record" => {
                    let record =
                        parser::attributes::Record::read_be(&mut Cursor::new(&attribute.info))?;

                    unit_type = JvmUnitType::Record(JvmRecord {
                        is_abstract,
                        is_super,
                        components: record
                            .components
                            .into_iter()
                            .map(|v| {
                                Ok(JvmRecordComponent {
                                    name: get_string(&jvm_strings, &v.name_index)?,
                                    descriptor: JvmTypeDescriptor::from_str(&get_string(
                                        &jvm_strings,
                                        &v.descriptor_index,
                                    )?)?,
                                })
                            })
                            .collect::<Result<Vec<_>>>()?,
                    })
                }
                "Deprecated" => {
                    is_deprecated = true;
                }
                "Synthetic" => {
                    is_synthetic = true;
                }
                v => {
                    warn!("unknown/unsupported attributes in code attribute of a method: {v}");
                }
            }
        }

        Ok(Self {
            minor_version,
            major_version,
            this_class,
            super_class,
            loadable_constant_pool,
            is_public,
            is_synthetic,
            is_deprecated,
            unit_type,
            interfaces,
            fields,
            methods,
            field_refs,
            method_refs,
            interface_method_refs,
        })
    }
}
