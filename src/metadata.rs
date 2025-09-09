use crate::{Data, WrapperCore};

#[derive(Debug, Clone)]
pub struct MetadataChunk {
    pub metadata: Vec<Metadata>
}

impl MetadataChunk {
    pub fn new() -> MetadataChunk {
        MetadataChunk { metadata: Vec::new() }
    }

    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for data in self.metadata {
            bytes.append(&mut data.to_bytes(wrapper));
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum Metadata {
    General(String, String), // key => value
    Byte(usize, usize, String), // (chunk, byte_offset) => value
    Element(usize, usize, String), // (chunk, element) => value
}

impl Metadata {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Metadata::General(key, value) => {
                bytes.push(0x00);
                bytes.append(&mut wrapper.add_data(Data::Text(key.into())));
                bytes.append(&mut wrapper.add_data(Data::Text(value.into())));
            }
            Metadata::Byte(chunk, byte_offset, value) => {
                bytes.push(0x01);
                bytes.append(&mut WrapperCore::num_to_bytes(*chunk));
                bytes.append(&mut WrapperCore::num_to_bytes(*byte_offset));
                bytes.append(&mut wrapper.add_data(Data::Text(value.into())));
            }
            Metadata::Element(chunk, element, value) => {
                bytes.push(0x02);
                bytes.append(&mut WrapperCore::num_to_bytes(*chunk));
                bytes.append(&mut WrapperCore::num_to_bytes(*element));
                bytes.append(&mut wrapper.add_data(Data::Text(value.into())));
            }
        }
        
        return bytes;
    }
}
