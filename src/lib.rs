use std::{hint::unreachable_unchecked, process::exit};

use conditional_parsing::ConditionalParsingChunk;
use fox::critical;
use chunks::{Chunk, Data};
use code::{CodeBlock, CodeChunk};
use data::DataChunk;
use indexmap::IndexSet;
use instructions::Instruction;
use metadata::MetadataChunk;
use modules::ModuleChunk;
use runtime_constants::{RuntimeConstant, RuntimeConstantChunk};
use type_cast::TypeCastChunk;

// is having this many files the best idea?
// i could combine them into less files and it would still work
// but maybe it's a good idea to do this for organization
pub mod chunks;
pub mod instructions;
pub mod code;
pub mod modules;
pub mod data;
pub mod metadata;
pub mod checksum;
pub mod type_cast;
pub mod conditional_parsing;
pub mod runtime_constants;

const MAJOR_VERSION: u16 = 1;
const MINOR_VERSION: u16 = 0;
const PATCH_VERSION: u16 = 0;

pub struct Wrapper {
    pub wrapper_core: WrapperCore,
    
    chunk_list: Vec<Chunk>,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper {
            wrapper_core: WrapperCore::new(), 

            chunk_list: Vec::new(),
        }
    }

    // code chunk
    pub fn code_begin(&mut self) {
        if let Some(chunk) = self.prev_chunk() {
            match chunk {
                Chunk::Code(_) | Chunk::Module(_) => {},
                _ => { critical!("cannot nest code chunks with non-code/non-module chunks"); exit(1); }
            }
        }

        self.add_chunk(Chunk::Code(CodeChunk::new(self.chunk_list.len() > 0)));
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        let chunk = self.chunk_list.last_mut();
        if let Some(chunk) = chunk {
            match chunk {
                Chunk::Code(c) => {
                    let block = c.blocks.last_mut();
                    if let Some(block) = block {
                        let block = match block {
                            CodeBlock::Code(inst) => inst,
                            CodeBlock::Scope(_) => { // TODO: this is dumb
                                c.blocks.push(CodeBlock::Code(Vec::new()));
                                if let Some(b) = c.blocks.last_mut() {
                                    if let CodeBlock::Code(c) = b {
                                        c
                                    } else {
                                        unsafe { unreachable_unchecked() }
                                    }
                                } else {
                                    unsafe { unreachable_unchecked() }
                                }
                            },
                        };
                        block.push(inst);
                    } else {
                        unreachable!("found a code chunk with no blocks. this is probably a bug");
                    }
                }
                _ => {
                    fox::scritical!("attempted to add an instruction while not in a code chunk");
                    exit(1);
                }
            }
        } else {
            fox::scritical!("attempted to add an instruction while not in a chunk");
            exit(1);
        }
    }

    pub fn code_end(&mut self) {
        let chunk = self.chunk_list.pop();
        if let Some(chunk) = chunk {
            let chunk = match chunk {
                Chunk::Code(c) => c,
                _ => {
                    fox::scritical!("ran `code_end` when not in code chunk");
                    exit(1);
                }
            };
            let prev = self.chunk_list.last_mut();
            if let Some(prev) = prev {
                match prev {
                    Chunk::Code(c)   => c.add_scope(chunk), 
                    Chunk::Module(c) => c.add_code(chunk),
                    _ => unreachable!("somehow nested a code block in a non-code block. this is probably a bug")
                }
            } else {
                self.chunk_list.push(Chunk::Code(chunk));
            }
        } else {
            fox::serror!("attempted to end chunk but was not in one");
        }
    }

    // module chunk
    pub fn module_begin(&mut self, name: String) {
        if let Some(chunk) = self.prev_chunk() {
            match chunk {
                Chunk::Module(_) => {},
                _ => { critical!("cannot nest module chunks with non-module chunks"); exit(1); }
            }
        }
        self.add_chunk(Chunk::Module(ModuleChunk::new(name, self.chunk_list.len() > 0)));
    }

    // creating data section chunks manually is not supported here, as there is no way to use them
    
    // metadata chunk
    pub fn metadata_begin(&mut self) {
        if self.chunk_list.len() > 0 {
            critical!("cannot nest non code/module chunks"); 
            exit(1); 
        }

        self.add_chunk(Chunk::Metadata(MetadataChunk::new()));
    }

    // checksum chunk
    pub fn checksum_begin(&mut self) {
        if self.chunk_list.len() > 0 {
            critical!("cannot nest non code/module chunks"); 
            exit(1); 
        }

        todo!(); // :trol: i dont wanna implement it right now
    }

    // type cast chunk
    pub fn type_cast_begin(&mut self) {
        if self.chunk_list.len() > 0 {
            critical!("cannot nest non code/module chunks"); 
            exit(1); 
        }

        self.add_chunk(Chunk::TypeCast(TypeCastChunk::new()));
    }
    
    // conditional parsing chunk
    pub fn conditional_parsing_begin(&mut self) {
        if self.chunk_list.len() > 0 {
            critical!("cannot nest non code/module chunks"); 
            exit(1); 
        }

        self.add_chunk(Chunk::ConditionalParsing(ConditionalParsingChunk::new()));
    }

    // creating runtime constant chunks manually is not supported here, as there is no way to use them
    
    // all of these `[chunk]_end` functions are just for consistency unless they have more functionality
    // i feel like a C# dev right now
    pub fn module_end(&mut self) {
        todo!();
    }

    pub fn metadata_end(&mut self) {
        self.chunk_end();
    }

    pub fn checksum_end(&mut self) {
        self.chunk_end();
    }

    pub fn type_cast_end(&mut self) {
        self.chunk_end();
    }

    pub fn conditional_parsing_end(&mut self) {
        self.chunk_end();
    }

    fn chunk_end(&mut self) {
        if let Some(chunk) = self.chunk_list.pop() {
            self.wrapper_core.add_chunk(chunk);
        } else {
            fox::serror!("attempted to end chunk but was not in one");
        }
    }

    fn add_chunk(&mut self, chunk: Chunk) {
        self.chunk_list.push(chunk);
    }

    fn prev_chunk(&self) -> Option<Chunk> {
        return self.chunk_list.last().cloned();
    }

    pub fn emit(&mut self) -> Vec<u8> {
        self.wrapper_core.chunks.append(&mut self.chunk_list);
        self.wrapper_core.emit()
    }
}

pub struct WrapperCore {
    pub compressed: bool,
    pub chunks: Vec<Chunk>,
    pub endianness: bool,

    data: IndexSet<Data>,
    runtime_constants: IndexSet<RuntimeConstant>,

    signed: bool,
}

impl WrapperCore {
    pub fn new() -> WrapperCore {
        WrapperCore { 
            chunks: Vec::new(), 
            data: IndexSet::new(), 
            runtime_constants: IndexSet::new(), 
            compressed: false, 
            signed: false, 
            endianness: u16::from_ne_bytes([1, 0]) == 0, // stupid but im not making a function for this
        }
    }

    /// Convert a number into a data section compatible number,
    /// consisting of {size} {bytes}
    pub fn index_to_bytes(index: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // get smallest possible fit and use that
        if index < u8::MAX as usize {
            bytes.push((u8::BITS/8) as u8);
            bytes.append(&mut (index as u8).to_ne_bytes().to_vec());
        } else if index < u16::MAX as usize {
            bytes.push((u16::BITS/8) as u8);
            bytes.append(&mut (index as u16).to_ne_bytes().to_vec());
        } else if index < u32::MAX as usize {
            bytes.push((u32::BITS/8) as u8);
            bytes.append(&mut (index as u32).to_ne_bytes().to_vec());
        } else {
            bytes.push((u64::BITS/8) as u8);
            bytes.append(&mut (index as u64).to_ne_bytes().to_vec());
        }

        return bytes;
    }

    pub fn add_data(&mut self, data: Data) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x01, 0x00]; // default data section index
        bytes.append(&mut WrapperCore::index_to_bytes(self.data.len()));

        self.data.insert(data);
        return bytes;
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> Vec<u8> {
        self.chunks.push(chunk);

        return WrapperCore::index_to_bytes(self.chunks.len() - 1);
    }

    pub fn add_runtime_constant(&mut self, constant: RuntimeConstant) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut WrapperCore::index_to_bytes(constant.name.len()));
        bytes.append(&mut constant.name.as_bytes().to_vec());

        self.runtime_constants.insert(constant);

        return bytes;
    }

    // adler-32 chosen for ease of implementation
    fn checksum(data: &[u8]) -> u32 {
        let mut s1: u32 = 1;
        let mut s2: u32 = 0;
        
        const MOD_ADLER: u32 = 65521;
        const NMAX: usize = 5552;
        
        for chunk in data.chunks(NMAX) {
            for byte in chunk {
                s1 = s1 + (*byte as u32);
                s2 = s2 + s1;
            }
            s1 = s1 % MOD_ADLER;
            s2 = s2 % MOD_ADLER;
        }
        
        return (s2 << 16) | s1;
    }

    pub fn emit(&mut self) -> Vec<u8> {
        let mut body: Vec<u8> = Vec::new();

        // TODO: figure out a way to populate the data section *without* needing to do this whole thing twice
        let mut i = 0;
        while i < self.chunks.len() {
            let mut chunk = self.chunks[i].clone(); // .clone() :why:
            chunk.to_bytes(self);
            i += 1;
        }

        self.chunks.insert(0, Chunk::Data(DataChunk::from_set(&self.data)));
        self.chunks.insert(1, Chunk::RuntimeConstant(RuntimeConstantChunk::from_set(&self.runtime_constants)));

        let mut i = 0;
        while i < self.chunks.len() {
            let mut chunk = self.chunks[i].clone(); // .clone() :why:
            body.append(&mut chunk.to_bytes(self));
            i += 1;
        }

        // RBB file header
        let mut out: Vec<u8> = b"RBB".to_vec();
        out.append(&mut MAJOR_VERSION.to_ne_bytes().to_vec());
        out.append(&mut MINOR_VERSION.to_ne_bytes().to_vec());
        out.append(&mut PATCH_VERSION.to_ne_bytes().to_vec());

        out.append(&mut Self::checksum(&body).to_ne_bytes().to_vec());

        let endianness = if self.endianness {
            1
        } else {
            0
        };
        out.push(endianness);

        let compressed = if self.compressed {
            1
        } else {
            0
        };
        out.push(compressed);

        if self.signed {
            todo!("Signing of Rainbow programs is not yet implemented");
        } else {
            out.push(0);
        }

        out.append(&mut WrapperCore::index_to_bytes(body.len()));
        
        out.append(&mut body);

        return out;
    }
}
