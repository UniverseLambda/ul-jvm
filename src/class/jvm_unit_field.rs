use std::{collections::HashMap, io::Cursor, str::FromStr};

use anyhow::{Result, anyhow, bail};
use binrw::BinRead;
use log::warn;
use serde::Serialize;

use crate::types::JvmTypeDescriptor;

use super::{
    JvmVisibility,
    attributes::Signature,
    constant_pool::{ConstantJvmUtf8, LoadableJvmConstant},
    get_loadable_constant, get_string,
    parser::{self, FieldAccessFlags, FieldInfo},
};

#[derive(Debug, Clone, Serialize)]
pub struct JvmUnitField {
    pub name: ConstantJvmUtf8,
    pub vis: JvmVisibility,
    pub ty: JvmTypeDescriptor,
    pub constant_value: Option<LoadableJvmConstant>,
    pub signature: Option<Signature>,
    pub is_deprecated: bool,
    pub is_static: bool,
    pub is_final: bool,
    pub is_volatile: bool,
    pub is_transient: bool,
    pub is_synthetic: bool,
    pub is_enum: bool,
}

impl JvmUnitField {
    pub fn from_class_file(
        info: &FieldInfo,
        jvm_strings: &HashMap<u16, ConstantJvmUtf8>,
        loadable_constant_pool: &HashMap<u16, LoadableJvmConstant>,
    ) -> Result<Self> {
        let name = get_string(jvm_strings, &info.name_index)?;
        let is_public = info.access_flags.contains(&FieldAccessFlags::AccPublic);
        let is_private = info.access_flags.contains(&FieldAccessFlags::AccPrivate);
        let is_protected = info.access_flags.contains(&FieldAccessFlags::AccProtected);

        if (is_public && is_private) || (is_private && is_protected) || (is_protected && is_public)
        {
            bail!("class has multiple visibility access flags");
        }

        let vis = if is_private {
            JvmVisibility::Private
        } else if is_protected {
            JvmVisibility::Protected
        } else {
            JvmVisibility::Public
        };

        let is_static = info.access_flags.contains(&FieldAccessFlags::AccStatic);
        let is_final = info.access_flags.contains(&FieldAccessFlags::AccFinal);
        let is_volatile = info.access_flags.contains(&FieldAccessFlags::AccVolatile);
        let is_transient = info.access_flags.contains(&FieldAccessFlags::AccTransient);
        let mut is_synthetic = info.access_flags.contains(&FieldAccessFlags::AccSynthetic);
        let is_enum = info.access_flags.contains(&FieldAccessFlags::AccEnum);

        let ty = JvmTypeDescriptor::from_str(
            &get_string(jvm_strings, &info.descriptor_index)?.convert_to_string(),
        )?;

        let mut constant_value = None;
        let mut signature = None;
        let mut is_deprecated = false;

        for attribute in info.attributes.iter() {
            let attribute_name =
                get_string(jvm_strings, &attribute.attribute_name_index)?.convert_to_string();

            match attribute_name.as_str() {
                "Signature" => {
                    let sig =
                        parser::attributes::Signature::read_be(&mut Cursor::new(&attribute.info))?;

                    let signature_str = get_string(jvm_strings, &sig.signature_index)?;

                    signature = Some(Signature {
                        signature: signature_str,
                    })
                }
                "ConstantValue" => {
                    let cv = parser::attributes::ConstantValue::read_be(&mut Cursor::new(
                        &attribute.info,
                    ))?;

                    let res =
                        get_loadable_constant(&loadable_constant_pool, &cv.constantvalue_index)?;

                    constant_value = Some(res);
                }
                "Deprecated" => {
                    is_deprecated = true;
                }
                "Synthetic" => {
                    is_synthetic = true;
                }
                v => {
                    warn!("unknown/unsupported attributes in field: {v}");
                }
            }
        }

        Ok(Self {
            name,
            vis,
            ty,
            constant_value,
            signature,
            is_deprecated,
            is_static,
            is_final,
            is_volatile,
            is_transient,
            is_synthetic,
            is_enum,
        })
    }
}
