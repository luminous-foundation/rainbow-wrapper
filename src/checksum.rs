#[derive(Clone)]
pub struct ChecksumChunk {
    pub checksum: u32,
    pub chunk_checksums: Vec<u32>,
}

impl ChecksumChunk {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut self.checksum.to_ne_bytes().to_vec());

        for checksum in &self.chunk_checksums {
            bytes.append(&mut checksum.to_ne_bytes().to_vec());
        }

        return bytes;
    }
}
