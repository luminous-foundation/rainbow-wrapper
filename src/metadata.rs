pub struct MetadataChunk {
    pub metadata: Vec<Metadata>
}

pub enum Metadata {
    General(String, String),
    Byte(usize, usize, String),
}
