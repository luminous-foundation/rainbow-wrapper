use crate::chunks::{Data, Type};

pub struct RuntimeConstantChunk {
    pub constants: Vec<RuntimeConstant>,
}

pub struct RuntimeConstant {
    pub name: String,
    pub typ: Type,
    pub defuault: Data,
}
