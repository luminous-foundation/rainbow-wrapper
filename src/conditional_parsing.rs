use crate::chunks::Data;

pub struct ConditionalParsingChunk {
    pub conditional_chunks: Vec<ConditionalChunk>,
}

pub struct ConditionalChunk {
    pub chunk_id: usize,
    pub conditional: Conditional,
}

pub struct Conditional {
    pub left: ConditionalValue,
    pub condition: Condition,
    pub right: ConditionalValue,
}

pub enum ConditionalValue {
    Value(Data),
    RuntimeConstant(String),
}

pub enum Condition {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
