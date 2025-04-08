use std::{ops::Deref, sync::Arc};

use super::class::ClassField;

#[derive(Debug, Clone)]
pub struct Interface(Arc<InterfaceInner>);

impl Interface {
    pub fn new(name: String, static_fields: Box<[ClassField]>) -> Self {
        Self(Arc::new(InterfaceInner {
            name,
            static_fields,
        }))
    }
}

impl AsRef<InterfaceInner> for Interface {
    fn as_ref(&self) -> &InterfaceInner {
        &self.0
    }
}

impl Deref for Interface {
    type Target = InterfaceInner;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct InterfaceInner {
    pub name: String,
    pub static_fields: Box<[ClassField]>,
}
