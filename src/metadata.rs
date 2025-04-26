use crate::WrapperCore;

#[derive(Debug, Clone)]
pub struct MetadataChunk {
    pub metadata: Vec<Metadata>
}

impl MetadataChunk {
    pub fn new() -> MetadataChunk {
        MetadataChunk { metadata: Vec::new() }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for data in &self.metadata {
            bytes.append(&mut data.to_bytes());
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum Metadata {
    General(String, String),
    Byte(usize, usize, String),
}

impl Metadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Metadata::General(key, value) => {
                bytes.append(&mut WrapperCore::index_to_bytes(key.len()));
                bytes.append(&mut key.as_bytes().to_vec());
                bytes.append(&mut WrapperCore::index_to_bytes(value.len()));
                bytes.append(&mut value.as_bytes().to_vec());
            }
            Metadata::Byte(chunk, offset, value) => {
                bytes.append(&mut WrapperCore::index_to_bytes(*chunk));
                bytes.append(&mut WrapperCore::index_to_bytes(*offset));
                bytes.append(&mut WrapperCore::index_to_bytes(value.len()));
                bytes.append(&mut value.as_bytes().to_vec());
            }
        }
        
        return bytes;
    }
}
