use std::sync::{Arc, Weak, atomic::AtomicUsize};

use crate::exec::{array::Array, class::ClassInstance};

/**
 * The idea here is that JvmStrongRef are only stored in the Heap struct (and probably in the stack and static fields).
 * So after the garbage allocator visits each allocated structures and marked them as visited or not, it only needs to remove from the Heap struct those
 * with a strong_count of 1 AND that were not marked as visited (and the Arc should do the rest).
*/
#[derive(Debug)]
pub struct JvmStrongRef<T> {
    inner: Option<Arc<T>>,
    last_visited: AtomicUsize,
}

impl<T> JvmStrongRef<T> {
    pub(super) fn new(value: T) -> Self {
        Self {
            inner: Some(Arc::new(value)),
            last_visited: AtomicUsize::new(0),
        }
    }

    pub(super) fn new_ref(&self) -> JvmRef<T> {
        JvmRef {
            inner: self.inner.as_ref().map(|a| Arc::downgrade(a)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JvmRef<T> {
    inner: Option<Weak<T>>,
}

impl<T> Default for JvmRef<T> {
    fn default() -> Self {
        Self::new_null()
    }
}

impl<T> JvmRef<T> {
    pub fn new_null() -> Self {
        Self { inner: None }
    }

    // pub fn try_read(&self) -> Option<JvmRefReadHandle<T>> {
    //     self.inner.clone().map(|v| JvmRefReadHandle(v))
    // }

    pub fn is_null(&self) -> bool {
        self.inner.is_none()
    }
}

// pub struct JvmRefReadHandle<T>(Arc<T>);

// impl<T> AsRef<T> for JvmRefReadHandle<T> {
//     fn as_ref(&self) -> &T {
//         &self.0
//     }
// }

// impl<T> Deref for JvmRefReadHandle<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         self.0.deref()
//     }
// }

pub type ClassRef = JvmRef<ClassInstance>;
pub type ArrayRef = JvmRef<Array>;

pub type StrongClassRef = JvmStrongRef<ClassInstance>;
pub type StrongArrayRef = JvmStrongRef<Array>;
