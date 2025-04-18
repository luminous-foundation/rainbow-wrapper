use indexmap::IndexMap;

use crate::{chunks::{Data, FuncRef, Type}, Wrapper};

#[derive(Clone)]
pub struct TypeCastChunk {
    pub type_casts: IndexMap<(Type, Type), FuncRef>
}

impl TypeCastChunk {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for type_cast in &self.type_casts {
            bytes.append(&mut type_cast.0.0.to_bytes(wrapper));
            bytes.append(&mut type_cast.0.1.to_bytes(wrapper));
            bytes.append(&mut wrapper.add_data(Data::FuncRef(type_cast.1.clone())));
        }

        return bytes;
    }
}
