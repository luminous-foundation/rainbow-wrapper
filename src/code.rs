use std::process::exit;

use crate::{chunks::{Chunk, Data, Type}, instructions::Instruction, WrapperCore};

#[derive(Debug, Clone)]
pub struct CodeChunk {
    pub has_parent: bool,

    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub blocks: Vec<CodeBlock>,
}

impl CodeChunk {
    pub fn new(has_parent: bool) -> CodeChunk {
        CodeChunk {
            has_parent, 
            structs: Vec::new(), 
            functions: Vec::new(), 
            blocks: vec![CodeBlock::Code(Vec::new())],
        }
    }

    pub fn add_scope(&mut self, chunk: CodeChunk) {
        self.blocks.push(CodeBlock::Scope(chunk));
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn add_struct(&mut self, strct: Struct) {
        self.structs.push(strct);
    }

    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        if self.has_parent {
            bytes.push(0x01);
        } else {
            bytes.push(0x00);
        }

        for structt in self.structs {
            bytes.append(&mut structt.to_bytes(wrapper));
        }

        for function in self.functions {
            bytes.append(&mut function.to_bytes(wrapper));
        }

        for block in self.blocks {
            bytes.append(&mut block.to_bytes(wrapper));
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum CodeBlock {
    Code(Vec<Instruction>),
    Scope(CodeChunk),
}

impl CodeBlock {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        match self {
            CodeBlock::Code(instructions) => {
                for instruction in instructions {
                    bytes.append(&mut instruction.to_bytes(wrapper));
                }
            }
            CodeBlock::Scope(chunk) => {
                bytes.push(0xFF);
                bytes.append(&mut wrapper.add_chunk(Chunk::Code(chunk.clone())));
            }
        }
        return bytes;
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub vars: Vec<(Type, String, Option<Data>)>,
}

impl Struct {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // sanity check, as technically bad data could make it here
        for var in &self.vars {
            match var.0 {
                Type::Struct(_) => {},
                _ => {
                    if let None = var.2 {
                        fox::scritical!("field `{}` of struct `{}` was missing default value", var.0, self.name);
                        exit(1);
                    }
                }
            }
        }

        bytes.push(0xFD);

        bytes.append(&mut wrapper.add_data(Data::Text(self.name)));

        for var in self.vars {
            bytes.append(&mut var.0.to_bytes(wrapper));
            bytes.append(&mut wrapper.add_data(Data::Text(var.1)));

            match var.0 {
                Type::Struct(_) => {},
                _ => {
                    if let Some(default) = var.2 {
                        bytes.append(&mut wrapper.add_data(default));
                    }
                }
            }
        }

        bytes.push(0xFC);

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub ret_type: Type,
    pub args: Vec<(Type, String)>,
    pub body: CodeChunk,
}

impl Function {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0xFB];

        bytes.append(&mut wrapper.add_data(Data::Text(self.name.clone())));

        bytes.append(&mut self.ret_type.to_bytes(wrapper));

        for arg in self.args {
            bytes.append(&mut arg.0.to_bytes(wrapper));
            bytes.append(&mut wrapper.add_data(Data::Text(arg.1.clone())));
        }

        bytes.push(0xFA);

        bytes.push(0xFF);
        bytes.append(&mut wrapper.add_chunk(Chunk::Code(self.body)));
        bytes.push(0xFE);

        return bytes;
    }
}
