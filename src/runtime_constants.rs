use indexmap::IndexSet;

use crate::{chunks::{Data, Number, Type}, WrapperCore};

#[derive(Debug, Clone)]
pub struct RuntimeConstantChunk {
    pub constants: Vec<RuntimeConstant>,
}

impl RuntimeConstantChunk {
    pub fn new() -> RuntimeConstantChunk {
        RuntimeConstantChunk {
            constants: Vec::new(),
        }
    }

    pub fn from_set(constants: &IndexSet<RuntimeConstant>) -> RuntimeConstantChunk {
        let mut chunk: RuntimeConstantChunk = RuntimeConstantChunk { constants: Vec::new() };

        for constant in constants {
            chunk.constants.push(constant.clone());
        }

        return chunk;
    }

    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
    
        for constant in self.constants {
            bytes.append(&mut constant.to_bytes(wrapper));
        }

        return bytes;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RuntimeConstant {
    pub name: String,
    pub typ: Type,
    pub default: Constant,
}

impl RuntimeConstant {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut WrapperCore::num_to_bytes(self.name.len()));
        bytes.append(&mut self.name.as_bytes().to_vec());
        bytes.append(&mut self.typ.to_bytes(wrapper));
        bytes.append(&mut self.default.to_bytes(wrapper));

        return bytes;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Constant {
    Number(Number),
}

impl Constant {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Constant::Number(n) => bytes.append(&mut wrapper.add_data(Data::Number(n.clone()))),
        }
        
        return bytes;
    }
}
