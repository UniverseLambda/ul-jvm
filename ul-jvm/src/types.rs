use std::str::FromStr;

use anyhow::{Context, anyhow, bail};
use serde::Serialize;

use crate::exec::runtime_type::RuntimeType;

pub type JvmInt = i32;
pub type JvmLong = i64;

pub type JvmFloat = f32;
pub type JvmDouble = f64;

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub enum JvmTypeDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(String),
    Short,
    Boolean,
    Array(Box<JvmTypeDescriptor>),
}

impl FromStr for JvmTypeDescriptor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("empty type descriptor");
        }

        Ok(match s.chars().next().unwrap() {
            'B' => Self::Byte,
            'C' => Self::Char,
            'D' => Self::Double,
            'F' => Self::Float,
            'I' => Self::Int,
            'J' => Self::Long,
            'S' => Self::Short,
            'Z' => Self::Boolean,
            '[' => Self::Array(Box::new(
                Self::from_str(&s[1..]).context("while parsing component type of an array")?,
            )),
            'L' => {
                // FIXME: implement more checks (https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.2)
                if !s.ends_with(';') {
                    bail!("no semi-colon at the end of class type descriptor");
                }

                let class_name = &s[1..s.len() - 1];

                match class_name {
                    "" => bail!("empty class name in type descriptor"),
                    v if v.contains([';', '.', '[']) => bail!("forbidden character in class name"),
                    &_ => (),
                }

                Self::Class(class_name.to_string())
            }
            v => bail!("unknown type descriptor: {v}"),
        })
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct JvmMethodDescriptor {
    pub parameter_types: Vec<JvmTypeDescriptor>,
    pub return_type: Option<JvmTypeDescriptor>,
}

impl FromStr for JvmMethodDescriptor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => bail!("empty method descriptor"),
            v if !v.starts_with('(') => bail!("method descriptor does not starts with '('"),
            v if !v.contains(')') => bail!("method descriptor does not contain a closing ')'"),
            &_ => (),
        }

        let (chunky_boi, ret_desc) = &s.split_at(s.find(')').unwrap());

        let mut chunky_boi = &chunky_boi[1..];

        let mut parameter_types = vec![];

        while !chunky_boi.is_empty() {
            let (to_parse, next) = if chunky_boi.starts_with("[") {
                let array_desc_end = chunky_boi
                    .find(|c| c != '[')
                    .ok_or_else(|| anyhow!("un-ended array descriptor"))?;

                if chunky_boi.chars().nth(array_desc_end).unwrap() == 'L' {
                    let class_desc = chunky_boi
                        .find(';')
                        .ok_or_else(|| anyhow!("un-ended class descriptor"))?;

                    chunky_boi.split_at(class_desc + 1)
                } else {
                    chunky_boi.split_at(array_desc_end + 1)
                }
            } else if chunky_boi.starts_with("L") {
                let class_desc = chunky_boi
                    .find(';')
                    .ok_or_else(|| anyhow!("un-ended class descriptor"))?;

                chunky_boi.split_at(class_desc + 1)
            } else {
                chunky_boi.split_at(1)
            };

            chunky_boi = next;
            parameter_types.push(
                to_parse
                    .parse()
                    .context("while parsing a parameter type of a method")?,
            );
        }

        let ret_desc = &ret_desc[1..];
        let return_type = if ret_desc == "V" {
            None
        } else {
            Some(
                JvmTypeDescriptor::from_str(ret_desc)
                    .context("while parsing the return type of a method")?,
            )
        };

        Ok(Self {
            parameter_types,
            return_type,
        })
    }
}

pub trait NativeJvmType {
    fn to_jvm_type() -> JvmTypeDescriptor;
    fn to_runtime_type(&self) -> RuntimeType;

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized;
}

pub trait NativeOptJvmType {
    fn to_opt_jvm_type() -> Option<JvmTypeDescriptor>;
    fn to_opt_runtime_type(&self) -> Option<RuntimeType>;
}

impl NativeOptJvmType for () {
    fn to_opt_jvm_type() -> Option<JvmTypeDescriptor> {
        None
    }

    fn to_opt_runtime_type(&self) -> Option<RuntimeType> {
        None
    }
}

impl<T> NativeOptJvmType for T
where
    T: NativeJvmType,
{
    fn to_opt_jvm_type() -> Option<JvmTypeDescriptor> {
        Some(T::to_jvm_type())
    }

    fn to_opt_runtime_type(&self) -> Option<RuntimeType> {
        Some(self.to_runtime_type())
    }
}

impl NativeJvmType for i8 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Byte
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Int(*self as JvmInt)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Int(v) => Some(*v as Self),
            _ => None,
        }
    }
}

impl NativeJvmType for i16 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Short
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Int(*self as JvmInt)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Int(v) => Some(*v as Self),
            _ => None,
        }
    }
}

impl NativeJvmType for i32 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Int
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Int(*self as JvmInt)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Int(v) => Some(*v as Self),
            _ => None,
        }
    }
}

impl NativeJvmType for i64 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Long
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Long(*self)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Long(v) => Some(*v as Self),
            _ => None,
        }
    }
}

impl NativeJvmType for f32 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Float
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Float(*self)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Float(v) => Some(*v as Self),
            _ => None,
        }
    }
}

impl NativeJvmType for f64 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Double
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Double(*self)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Double(v) => Some(*v as Self),
            _ => None,
        }
    }
}

impl NativeJvmType for u16 {
    fn to_jvm_type() -> JvmTypeDescriptor {
        JvmTypeDescriptor::Char
    }

    fn to_runtime_type(&self) -> RuntimeType {
        RuntimeType::Int(*self as JvmInt)
    }

    fn try_from_rt(rt: &RuntimeType) -> Option<Self>
    where
        Self: Sized,
    {
        match rt {
            RuntimeType::Int(v) => Some(*v as Self),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::JvmMethodDescriptor;

    fn method_desc_test() {
        JvmMethodDescriptor::from_str("(IDLjava/lang/Thread;)Ljava/lang/Object;")
            .expect("not working");
    }
}
