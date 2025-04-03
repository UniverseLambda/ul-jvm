use crate::types::JvmTypeDescriptor;

use super::{
    JvmVisibility,
    attributes::Signature,
    constant_pool::{ConstantJvmUtf8, LoadableJvmConstant},
};

#[derive(Debug, Clone)]
pub struct JvmUnitField {
    pub name: ConstantJvmUtf8,
    pub vis: JvmVisibility,
    pub ty: JvmTypeDescriptor,
    pub constant_value: LoadableJvmConstant,
    pub signature: Option<Signature>,
    pub is_deprecated: bool,
    pub is_static: bool,
    pub is_final: bool,
    pub is_volatile: bool,
    pub is_transient: bool,
    pub is_synthetic: bool,
    pub is_enum: bool,
}
