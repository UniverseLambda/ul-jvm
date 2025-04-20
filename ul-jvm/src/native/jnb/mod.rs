/*!
    JNB: Java Native Binding

    A way to create real native classes without too much headaches.

    Heavily inspired from other modern FFI binding methods.
*/

pub mod classes;

use std::fmt::Debug;

use crate::{
    exec::{JvmExecEnv, class::ClassInstance, runtime_type::RuntimeType, thread::JvmThread},
    types::{JvmMethodDescriptor, JvmTypeDescriptor},
};

pub struct JnbObjectTypeDescriptor {
    /// The fully qualified name of the class (with the package)
    ///
    /// Example: the class String is "java/lang/String"
    pub full_name: &'static str,
    pub fields: &'static [(&'static str, JvmTypeDescriptor, bool /* is final */)],
    pub static_fields: &'static [(&'static str, JvmTypeDescriptor, bool)],
    pub methods: &'static [(&'static str, JvmMethodDescriptor)],
    pub static_methods: &'static [(&'static str, JvmMethodDescriptor)],
}

#[allow(unused)]
pub trait JnbObjectType: Debug + Send + Sync {
    fn clinit(&self) -> anyhow::Result<()>;
    fn instanciate_uninit(&self) -> Box<dyn JnbObject>;

    fn is_standalone(&self) -> bool;

    /// name and value are always checked before
    fn get_static_field(&self, name: &str) -> RuntimeType {
        unimplemented!()
    }

    /// name and value are always checked before
    fn set_static_field(&self, name: &str, value: RuntimeType) {
        unimplemented!()
    }

    /// name is always checked before
    fn call_static(
        &self,
        info: JnbCallInfo,
        name: &str,
        args: &[RuntimeType],
    ) -> anyhow::Result<Option<RuntimeType>> {
        unimplemented!()
    }

    fn descriptor(&self) -> JnbObjectTypeDescriptor;
}

#[allow(unused)]
pub trait JnbObject: Debug + Send + Sync {
    /// name is always checked before
    fn call(
        &self,
        info: JnbCallInfo,
        name: &str,
        args: &[RuntimeType],
    ) -> anyhow::Result<Option<RuntimeType>> {
        unimplemented!()
    }

    /// name is always checked before
    fn get_field(&self, name: &str) -> anyhow::Result<RuntimeType> {
        unimplemented!()
    }

    /// name is always checked before
    fn set_field(&self, name: &str, value: RuntimeType) -> anyhow::Result<()> {
        unimplemented!()
    }
}

macro_rules! jnb_call {
    ($self:ident.$func:ident($info:expr, $args:expr $(, $idx:ty)* $(,)?)) => {
        $self.$func($info, $(
            $args.get($idx)
                .ok_or_else(|| anyhow::anyhow!(
                    "not enought argument provided to jnb method {} ({} expected, got {})",
                    stringify!($func),
                    jnb_call!(@COUNT@ $($idx),*),
                    $args.len()
                ))?
                .try_into_native().ok_or_else(|| anyhow::anyhow!("wrong arg type for JNB method"))?
        ),*).map(|v| v.to_opt_runtime_type())
    };
    ($self:ident::$func:ident($info:expr, $args:expr $(, $idx:ty)* $(,)?)) => {
        $self::$func($info, $(
            $args.get($idx)
                .ok_or_else(|| anyhow::anyhow!(
                    "not enought argument provided to jnb method {} ({} expected, got {})",
                    stringify!($func),
                    jnb_call!(@COUNT@ $($idx),*),
                    $args.len()
                ))?
                .try_into_native().ok_or_else(|| anyhow::anyhow!("wrong arg type for JNB method"))?
        ),*).map(|v| v.to_opt_runtime_type())
    };
    (@COUNT@ $tk:tt, $($more:tt),+) => {
        1 + jnb_call!(@COUNT@ $($more),+)
    };
    (@COUNT@ $tk:tt) => {
        1
    };
}

pub(super) use jnb_call;

pub struct JnbCallInfo<'a> {
    pub env: &'static JvmExecEnv,
    pub thread: &'a mut JvmThread,
    pub class: &'a ClassInstance,
}

// Does not work :(
// pub trait JnbStaticMethod {
//     fn call(self, args: &[RuntimeType]) -> anyhow::Result<Option<RuntimeType>>;

//     fn as_descriptor(&self) -> JvmMethodDescriptor;
// }

// pub trait JnbMethod<This: JnbObject> {
//     fn call(self, this: &This, args: &[RuntimeType]) -> anyhow::Result<Option<RuntimeType>>;

//     fn as_descriptor(&self) -> JvmMethodDescriptor;
// }

// macro_rules! impl_fn {
//     ($(($($idx:expr),*)),* $(,)?) => {
//         $(
//             paste::paste! {
//                 impl<R: NativeOptJvmType, $([<T $idx>]: NativeJvmType),* > JnbStaticMethod for for<'a> fn($([< T $idx >]),*) -> anyhow::Result<R> {
//                     fn call(self, _args: &[RuntimeType]) -> anyhow::Result<Option<RuntimeType>> {
//                         self($(
//                             _args[$idx].try_into_native().ok_or_else(|| anyhow::anyhow!("wrong arg type for JNB method"))?
//                         ),*).map(|r| r.to_opt_runtime_type())
//                     }

//                     fn as_descriptor(&self) -> JvmMethodDescriptor {
//                         JvmMethodDescriptor {
//                             parameter_types: vec![ $( [< T $idx >]::to_jvm_type() ),* ],
//                             return_type: R::to_opt_jvm_type()
//                         }
//                     }
//                 }

//                 impl<This: JnbObject, R: NativeOptJvmType, $([<T $idx>]: NativeJvmType),* > JnbMethod<This> for for<'a> fn(&'a This, $([< T $idx >]),*) -> anyhow::Result<R> {
//                     fn call(self, this: &This, _args: &[RuntimeType]) -> anyhow::Result<Option<RuntimeType>> {
//                         self(
//                             this,
//                             $(_args[$idx]
//                                 .try_into_native()
//                                 .ok_or_else(|| anyhow::anyhow!("wrong arg type for JNB method"))?),*)
//                             .map(|r| r.to_opt_runtime_type())
//                     }

//                     fn as_descriptor(&self) -> JvmMethodDescriptor {
//                         JvmMethodDescriptor {
//                             parameter_types: vec![ $( [< T $idx >]::to_jvm_type() ),* ],
//                             return_type: R::to_opt_jvm_type()
//                         }
//                     }
//                 }
//             }
//         )*
//     };
// }

// impl_fn! {
//     (),
//     (0),
//     (0, 1),
//     (0, 1, 2),
//     (0, 1, 2, 3),
//     (0, 1, 2, 3, 4),
//     (0, 1, 2, 3, 4, 5),
//     (0, 1, 2, 3, 4, 5, 6),
//     (0, 1, 2, 3, 4, 5, 6, 7),
// }
