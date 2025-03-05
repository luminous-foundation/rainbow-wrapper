use crate::{chunks::Type, instructions::Instruction};

pub struct CodeChunk {
    pub blocks: Vec<CodeBlock>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
}

pub enum CodeBlock {
    Code(Vec<Instruction>),
    Scope(CodeChunk),
}

pub struct Struct {
    
}

pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
}

pub struct Arg {
    pub typ: Type,
    pub name: String,
}

