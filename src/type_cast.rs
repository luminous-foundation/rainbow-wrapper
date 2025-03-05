use std::collections::HashMap;

use crate::chunks::{FunctionRef, Type};

pub struct TypeCastChunk {
    pub type_casts: HashMap<(Type, Type), FunctionRef>
}
