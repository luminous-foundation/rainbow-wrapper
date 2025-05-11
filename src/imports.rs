use crate::{Data, WrapperCore};

#[derive(Debug, Clone)]
pub struct ImportChunk {
    pub imports: Vec<(String, String)> // path, name
}

impl ImportChunk {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for import in self.imports {
            bytes.append(&mut wrapper.add_data(Data::Name(import.0)));
            bytes.append(&mut wrapper.add_data(Data::Name(import.1)));
        }
        return bytes;
    }
}
