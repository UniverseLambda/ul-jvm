use std::{ops::Deref, sync::Arc};

use crate::exec::{array::Array, class::ClassInstance};

#[derive(Debug, Clone)]
pub struct JvmRef<T> {
    inner: Option<Arc<T>>,
}

impl<T> JvmRef<T> {
    pub fn new_null() -> Self {
        Self { inner: None }
    }

    pub fn new(value: T) -> Self {
        JvmRef {
            inner: Some(Arc::new(value)),
        }
    }

    pub fn try_read(&self) -> Option<JvmRefReadHandle<T>> {
        self.inner.clone().map(|v| JvmRefReadHandle(v))
    }

    pub fn is_null(&self) -> bool {
        self.inner.is_none()
    }
}

pub struct JvmRefReadHandle<T>(Arc<T>);

impl<T> AsRef<T> for JvmRefReadHandle<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for JvmRefReadHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub type ClassRef = JvmRef<ClassInstance>;
pub type ArrayRef = JvmRef<Array>;
