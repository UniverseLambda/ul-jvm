mod object;

use std::collections::HashMap;

pub use object::*;

use super::JnbObjectType;

macro_rules! insert_jnb {
    ($map:ident, $jnb:expr) => {{
        let jnb = $jnb;

        $map.insert(
            jnb.descriptor().full_name,
            Box::new(jnb) as Box<dyn JnbObjectType>,
        );
    }};
}

pub fn jvm_intrisics() -> HashMap<&'static str, Box<dyn JnbObjectType>> {
    let mut map = HashMap::new();

    insert_jnb!(map, ObjectType);

    map
}
