use crate::{chunks::Data, runtime_constants::RuntimeConstant, WrapperCore};

#[derive(Debug, Clone)]
pub struct ConditionalParsingChunk {
    pub conditional_chunks: Vec<ConditionalChunk>,
}

impl ConditionalParsingChunk {
    pub fn new() -> ConditionalParsingChunk {
        ConditionalParsingChunk { conditional_chunks: Vec::new() }
    }

    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for chunk in self.conditional_chunks {
            bytes.append(&mut chunk.to_bytes(wrapper));
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub struct ConditionalChunk {
    pub chunk_id: usize,
    pub conditional: Conditional,
}

impl ConditionalChunk {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut WrapperCore::num_to_bytes(self.chunk_id));
        bytes.append(&mut self.conditional.to_bytes(wrapper));

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub left: ConditionalValue,
    pub condition: Condition,
    pub right: ConditionalValue,
}

impl Conditional {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut self.left.to_bytes(wrapper));
        bytes.append(&mut self.condition.to_bytes());
        bytes.append(&mut self.right.to_bytes(wrapper));

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum ConditionalValue {
    Value(Data),
    RuntimeConstant(RuntimeConstant),
}

impl ConditionalValue {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            ConditionalValue::Value(data) => bytes.append(&mut wrapper.add_data(data.clone())),
            ConditionalValue::RuntimeConstant(default) => {
                bytes.append(&mut wrapper.add_runtime_constant(default.clone()));
            }
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Condition {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Condition::Equal        => vec![0x00],
            Condition::NotEqual     => vec![0x01],
            Condition::Greater      => vec![0x02],
            Condition::GreaterEqual => vec![0x03],
            Condition::Less         => vec![0x04],
            Condition::LessEqual    => vec![0x05],
        }
    }
}
