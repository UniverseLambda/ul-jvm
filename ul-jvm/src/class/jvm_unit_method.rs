use std::{collections::HashMap, io::Cursor, str::FromStr};

use anyhow::{Result, bail};
use binrw::BinRead;
use log::warn;
use serde::Serialize;

use crate::types::{JvmMethodDescriptor, JvmTypeDescriptor};

use super::{
    JvmVisibility,
    attributes::{
        Code, ExceptionTableEntry, LineNumberTable, LineNumberTableEntry, LocalVariableTable,
        LocalVariableTableEntry, LocalVariableTypeTable, LocalVariableTypeTableEntry,
        MethodParameter, Signature, StackMapFrame, StackMapTable, VerificationTypeInfo,
    },
    constant_pool::{ConstantClass, ConstantJvmUtf8, LoadableJvmConstant},
    get_class, get_string, get_string_opt,
    parser::{self, MethodAccessFlags, MethodInfo, attributes::MethodParametersEntryAccessFlag},
};

#[derive(Debug, Clone, Serialize)]
pub struct JvmUnitMethod {
    pub name: ConstantJvmUtf8,
    pub descriptor: JvmMethodDescriptor,
    pub code: Option<Code>,
    pub local_count: usize,
    pub exceptions: Vec<ConstantClass>,
    pub parameters: Option<Vec<MethodParameter>>,
    pub signature: Option<Signature>,
    pub vis: JvmVisibility,
    pub is_deprecated: bool,
    pub is_static: bool,
    pub is_final: bool,
    pub is_synchronized: bool,
    pub is_bridge: bool,
    pub is_variadic: bool,
    pub is_native: bool,
    pub is_abstract: bool,
    pub is_strict: bool,
    pub is_synthetic: bool,
}

impl JvmUnitMethod {
    pub fn from_class_file(
        info: &MethodInfo,
        jvm_strings: &HashMap<u16, ConstantJvmUtf8>,
        loadable_constant_pool: &HashMap<u16, LoadableJvmConstant>,
    ) -> Result<Self> {
        let name = get_string(jvm_strings, &info.name_index)?;
        let is_public = info.access_flags.contains(&MethodAccessFlags::Public);
        let is_private = info.access_flags.contains(&MethodAccessFlags::Private);
        let is_protected = info.access_flags.contains(&MethodAccessFlags::Protected);

        if (is_protected || is_public) && is_private || (is_protected && is_public) {
            bail!("class has multiple visibility access flags");
        }

        let vis = if is_private {
            JvmVisibility::Private
        } else if is_protected {
            JvmVisibility::Protected
        } else {
            JvmVisibility::Public
        };

        let is_static = info.access_flags.contains(&MethodAccessFlags::Static);
        let is_final = info.access_flags.contains(&MethodAccessFlags::Final);
        let is_synchronized = info.access_flags.contains(&MethodAccessFlags::Synchronized);
        let is_bridge = info.access_flags.contains(&MethodAccessFlags::Bridge);
        let is_variadic = info.access_flags.contains(&MethodAccessFlags::Varargs);
        let is_native = info.access_flags.contains(&MethodAccessFlags::Native);
        let is_abstract = info.access_flags.contains(&MethodAccessFlags::Abstract);
        let is_strict = info.access_flags.contains(&MethodAccessFlags::Strict);
        let mut is_synthetic = info.access_flags.contains(&MethodAccessFlags::Synthetic);

        let ty = JvmMethodDescriptor::from_str(&get_string(jvm_strings, &info.descriptor_index)?)?;

        let mut signature = None;
        let mut is_deprecated = false;
        let mut code = None;
        let mut exceptions_opt = None;
        let mut parameters_opt = None;

        for attribute in info.attributes.iter() {
            let attribute_name = get_string(jvm_strings, &attribute.attribute_name_index)?;

            match attribute_name.as_str() {
                "Signature" => {
                    let sig =
                        parser::attributes::Signature::read_be(&mut Cursor::new(&attribute.info))?;

                    let signature_str = get_string(jvm_strings, &sig.signature_index)?;

                    signature = Some(Signature {
                        signature: signature_str,
                    })
                }
                "Code" => {
                    let code_attr =
                        parser::attributes::Code::read_be(&mut Cursor::new(&attribute.info))?;

                    // FIXME: error if multiple found

                    let mut line_number_table = vec![];
                    let mut local_variable_table = vec![];
                    let mut local_variable_type_table = vec![];
                    let mut stack_map_table = vec![];

                    for attribute in code_attr.attributes {
                        let attribute_name =
                            get_string(jvm_strings, &attribute.attribute_name_index)?;

                        match attribute_name.as_str() {
                            "LineNumberTable" => {
                                let lnt = parser::attributes::LineNumberTable::read_be(
                                    &mut Cursor::new(&attribute.info),
                                )?;

                                line_number_table.push(LineNumberTable {
                                    line_number_table: lnt
                                        .line_number_table
                                        .into_iter()
                                        .map(|v| LineNumberTableEntry {
                                            start_pc: v.start_pc,
                                            line_number: v.line_number,
                                        })
                                        .collect(),
                                });
                            }
                            "LocalVariableTable" => {
                                let lvt = parser::attributes::LocalVariableTable::read_be(
                                    &mut Cursor::new(&attribute.info),
                                )?;

                                local_variable_table.push(LocalVariableTable {
                                    local_variable_table: lvt
                                        .local_variable_table
                                        .into_iter()
                                        .map(|v| {
                                            Ok(LocalVariableTableEntry {
                                                start_pc: v.start_pc,
                                                length: v.length,
                                                name: get_string(jvm_strings, &v.name_index)?,
                                                descriptor: JvmTypeDescriptor::from_str(
                                                    &get_string(jvm_strings, &v.descriptor_index)?,
                                                )?,
                                                index: v.index,
                                            })
                                        })
                                        .collect::<Result<Vec<_>>>()?,
                                });
                            }
                            "LocalVariableTypeTable" => {
                                let lvtt = parser::attributes::LocalVariableTypeTable::read_be(
                                    &mut Cursor::new(&attribute.info),
                                )?;

                                local_variable_type_table.push(LocalVariableTypeTable {
                                    local_variable_type_table: lvtt
                                        .local_variable_type_table
                                        .into_iter()
                                        .map(|v| {
                                            Ok(LocalVariableTypeTableEntry {
                                                start_pc: v.start_pc,
                                                length: v.length,
                                                name: get_string(jvm_strings, &v.name_index)?,
                                                signature: get_string(
                                                    jvm_strings,
                                                    &v.signature_index,
                                                )?,
                                                index: v.index,
                                            })
                                        })
                                        .collect::<Result<Vec<_>>>()?,
                                });
                            }
                            "StackMapTable" => {
                                let smt = parser::attributes::StackMapTable::read_be(
                                    &mut Cursor::new(&attribute.info),
                                )?;

                                stack_map_table.push(StackMapTable {
                                    entries: smt.entries.into_iter().map(|v| Ok(match v {
                                        parser::attributes::StackMapFrame::Same { id } => StackMapFrame::Same { id },
                                        parser::attributes::StackMapFrame::SameLocals1StackItemFrame { id, stack } => {
                                            let zarma = VerificationTypeInfo::try_from_parser(loadable_constant_pool, stack[0].clone())?;

                                            StackMapFrame::SameLocals1StackItemFrame { id, stack: [zarma] }
                                        },
                                        parser::attributes::StackMapFrame::SameLocals1StackItemFrameExtended { offset_delta, stack } => {
                                            let zarma = VerificationTypeInfo::try_from_parser(loadable_constant_pool, stack[0].clone())?;

                                            StackMapFrame::SameLocals1StackItemFrameExtended { offset_delta, stack: [zarma] }
                                        },
                                        parser::attributes::StackMapFrame::ChopFrame { id, offset_delta } => StackMapFrame::ChopFrame { id, offset_delta },
                                        parser::attributes::StackMapFrame::SameFrameExtended { offset_delta } => StackMapFrame::SameFrameExtended { offset_delta },
                                        parser::attributes::StackMapFrame::AppendFrame { id, offset_delta, locals } => {
                                            let locals = locals.into_iter().map(|v| VerificationTypeInfo::try_from_parser(loadable_constant_pool, v)).collect::<Result<Vec<_>>>()?;

                                            StackMapFrame::AppendFrame { id, offset_delta, locals }
                                        },
                                        parser::attributes::StackMapFrame::FullFrame { offset_delta, number_of_locals, locals, number_of_stack_items, stack } => {
                                            let locals = locals.into_iter().map(|v| VerificationTypeInfo::try_from_parser(loadable_constant_pool, v)).collect::<Result<Vec<_>>>()?;
                                            let stack = stack.into_iter().map(|v| VerificationTypeInfo::try_from_parser(loadable_constant_pool, v)).collect::<Result<Vec<_>>>()?;

                                            StackMapFrame::FullFrame { offset_delta, number_of_locals, locals, number_of_stack_items, stack }
                                        },
                                    })).collect::<Result<Vec<_>>>()?,
                                });
                            }
                            v => {
                                warn!(
                                    "unknown/unsupported attributes in code attribute of a method: {v}"
                                );
                            }
                        }
                    }

                    code = Some(Code {
                        max_stack: code_attr.max_stack,
                        max_locals: code_attr.max_locals,
                        code: code_attr.code,
                        exception_table: code_attr
                            .exception_table
                            .into_iter()
                            .map(|v| {
                                Ok(ExceptionTableEntry {
                                    start_pc: v.start_pc,
                                    end_pc: v.end_pc,
                                    handler_pc: v.handler_pc,
                                    catch_type: if v.catch_type == 0 {
                                        None
                                    } else {
                                        Some(get_class(loadable_constant_pool, &v.catch_type)?)
                                    },
                                })
                            })
                            .collect::<Result<Vec<_>>>()?,
                        stack_map_table,
                        line_number_table,
                        local_variable_table,
                        local_variable_type_table,
                    });
                }
                "Exceptions" => {
                    let excs =
                        parser::attributes::Exceptions::read_be(&mut Cursor::new(&attribute.info))?;

                    exceptions_opt = Some(
                        excs.exception_index_table
                            .into_iter()
                            .map(|v| get_class(loadable_constant_pool, &v))
                            .collect::<Result<Vec<_>>>()?,
                    );
                }
                "MethodParameters" => {
                    let mp = parser::attributes::MethodParameters::read_be(&mut Cursor::new(
                        &attribute.info,
                    ))?;

                    parameters_opt = Some(
                        mp.parameters
                            .into_iter()
                            .map(|v| {
                                Ok(MethodParameter {
                                    name: get_string_opt(jvm_strings, &v.name_index),
                                    is_final: v
                                        .access_flags
                                        .contains(&MethodParametersEntryAccessFlag::Final),
                                    is_synthetic: v
                                        .access_flags
                                        .contains(&MethodParametersEntryAccessFlag::Synthetic),
                                    is_mandated: v
                                        .access_flags
                                        .contains(&MethodParametersEntryAccessFlag::Mandated),
                                })
                            })
                            .collect::<Result<Vec<_>>>()?,
                    );
                }
                "Deprecated" => {
                    is_deprecated = true;
                }
                "Synthetic" => {
                    is_synthetic = true;
                }
                v => {
                    warn!("unknown/unsupported attributes in method: {v}");
                }
            }
        }

        if code.is_none() && !is_native && !is_abstract {
            bail!(
                "no code attribute present in method {}, but it is not native nor abstract",
                name
            );
        }

        Ok(Self {
            local_count: code
                .as_ref()
                .map(|c| c.max_locals as usize)
                .unwrap_or_default(),
            code,
            name,
            descriptor: ty,
            exceptions: exceptions_opt.unwrap_or_default(),
            parameters: parameters_opt,
            signature,
            vis,
            is_deprecated,
            is_static,
            is_final,
            is_synchronized,
            is_bridge,
            is_variadic,
            is_native,
            is_abstract,
            is_strict,
            is_synthetic,
        })
    }
}
