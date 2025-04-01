use crate::{chunks::{Data, Type}, instructions::Instruction, Wrapper};

#[derive(Clone)]
pub struct CodeChunk {
    pub has_parent: bool,

    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub blocks: Vec<CodeBlock>,
}

impl CodeChunk {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        if self.has_parent {
            bytes.push(0x01);
        } else {
            bytes.push(0x00);
        }

        for structt in &self.structs {
            bytes.append(&mut structt.to_bytes(wrapper));
        }

        for function in & self.functions {
            bytes.append(&mut function.to_bytes(wrapper));
        }

        for block in &self.blocks {
            bytes.append(&mut block.to_bytes(wrapper));
        }

        return bytes;
    }
}

#[derive(Clone)]
pub enum CodeBlock {
    Code(Vec<Instruction>),
    Scope(CodeChunk),
}

impl CodeBlock {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        match self {
            CodeBlock::Code(instructions) => {
                for instruction in instructions {
                    bytes.append(&mut instruction.to_bytes());
                }
            }
            CodeBlock::Scope(chunk) => {
                bytes.push(0xFF);
                bytes.append(&mut chunk.to_bytes(wrapper));
                bytes.push(0xFE);
            }
        }
        return bytes;
    }
}

#[derive(Clone)]
pub struct Struct {
    pub vars: Vec<(Type, String)>,
}

impl Struct {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(0xFD);

        for var in &self.vars {
            bytes.append(&mut var.0.to_bytes(wrapper));
            bytes.append(&mut wrapper.add_data(Data::Name(var.1.clone())));
        }

        bytes.push(0xFC);

        return bytes;
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<(Type, String)>,
    pub body: CodeChunk,
}

impl Function {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0xFB];

        bytes.append(&mut wrapper.add_data(Data::Name(self.name.clone())));

        for arg in &self.args {
            bytes.append(&mut arg.0.to_bytes(wrapper));
            bytes.append(&mut wrapper.add_data(Data::Name(arg.1.clone())));
        }

        bytes.push(0xFA);

        bytes.push(0xFF);
        bytes.append(&mut self.body.to_bytes(wrapper));
        bytes.push(0xFE);

        return bytes;
    }
}
