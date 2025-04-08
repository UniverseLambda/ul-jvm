use std::str::FromStr;

use anyhow::{Context, anyhow, bail};
use log::{debug, trace};
use serde::Serialize;

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
                    v if v.is_empty() => bail!("empty class name in type descriptor"),
                    v if v.contains(&[';', '.', '[']) => bail!("forbidden character in class name"),
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
            v if v.is_empty() => bail!("empty method descriptor"),
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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::JvmMethodDescriptor;

    fn method_desc_test() {
        JvmMethodDescriptor::from_str("(IDLjava/lang/Thread;)Ljava/lang/Object;")
            .expect("not working");
    }
}
