use indexmap::IndexSet;

use crate::{chunks::Data, Wrapper};

#[derive(Clone)]
pub struct DataChunk {
    pub data: Vec<Data>
}

impl DataChunk {
    pub fn from_set(data: &IndexSet<Data>) -> DataChunk {
        let mut chunk: DataChunk = DataChunk { data: Vec::new() };

        for value in data {
            chunk.data.push(value.clone());
        }

        return chunk;
    }

    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for value in &self.data {
            bytes.append(&mut value.to_bytes(wrapper));
        }

        return bytes;
    }
}
