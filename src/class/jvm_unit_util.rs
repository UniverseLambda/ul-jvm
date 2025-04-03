use std::collections::HashMap;

use anyhow::{anyhow, bail};

use super::{
    constant_pool::{ConstantClass, ConstantJvmUtf8, LoadableJvmConstant},
    parser::ConstantPoolInfo,
};

pub(super) fn get_string(
    jvm_strings: &HashMap<u16, ConstantJvmUtf8>,
    idx: &u16,
) -> anyhow::Result<ConstantJvmUtf8> {
    jvm_strings
        .get(idx)
        .cloned()
        .ok_or_else(|| anyhow!("no strings in constant pool at {idx}"))
}

pub(super) fn get_name_and_type(
    constant_pool: &Vec<ConstantPoolInfo>,
    idx: &u16,
) -> anyhow::Result<(u16, u16)> {
    let res = constant_pool
        .get(*idx as usize)
        .ok_or_else(|| anyhow!("no NameAndType in constant pool at {idx}"))?
        .clone();

    let ConstantPoolInfo::NameAndType {
        name_index,
        descriptor_index,
    } = res
    else {
        bail!(
            "tried to access NameAndType at {idx} in constant pool, but something else was found"
        );
    };

    Ok((name_index, descriptor_index))
}

pub(super) fn get_class(
    loadable_constant_pool: &HashMap<u16, LoadableJvmConstant>,
    idx: &u16,
) -> anyhow::Result<ConstantClass> {
    let res = loadable_constant_pool
        .get(idx)
        .ok_or_else(|| anyhow!("no Class in constant pool at {idx}"))?
        .clone();

    let LoadableJvmConstant::Class(c) = res else {
        bail!("tried to access Class at {idx} in constant pool, but something else was found");
    };

    Ok(c)
}

pub(super) fn get_loadable_constant(
    loadable_constant_pool: &HashMap<u16, LoadableJvmConstant>,
    idx: &u16,
) -> anyhow::Result<LoadableJvmConstant> {
    loadable_constant_pool
        .get(idx)
        .cloned()
        .ok_or_else(|| anyhow!("no loadable constant in pool at {idx}"))
}
