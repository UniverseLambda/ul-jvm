use super::runtime_type::RuntimeType;

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub static_fields: Box<[RuntimeType]>,
}
