use crate::types::JvmMethodDescriptor;

use super::{
    JvmVisibility,
    attributes::{Code, MethodParameter, Signature},
    constant_pool::{ConstantClass, ConstantJvmUtf8},
};

#[derive(Debug, Clone)]
pub struct JvmUnitMethod {
    pub name: ConstantJvmUtf8,
    pub descriptor: JvmMethodDescriptor,
    pub code: Code,
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
    pub is_abstrace: bool,
    pub is_strict: bool,
    pub is_synthetic: bool,
}
