use std::{rc::Rc, usize};

use conditional_parsing::ConditionalParsingChunk;
use chunk_node::ChunkNode;
use chunks::{Chunk, Data};
use code::CodeChunk;
use data::DataChunk;
use fox::*;
use indexmap::IndexSet;
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

pub mod chunk_node;

const MAJOR_VERSION: u16 = 1;
const MINOR_VERSION: u16 = 0;
const PATCH_VERSION: u16 = 0;

pub struct Wrapper {
    pub wrapper_core: WrapperCore,
    
    pub cur_chunk: Option<Rc<ChunkNode>>,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper {
            wrapper_core: WrapperCore::new(), 

            cur_chunk: None,
        }
    }

    pub fn code_begin(&mut self) {
        self.set_chunk(Chunk::Code(CodeChunk::new(self.cur_chunk.is_some())));
    }

    pub fn module_begin(&mut self, name: String) {
        self.set_chunk(Chunk::Module(ModuleChunk::new(name, self.cur_chunk.is_some())));
    }

    // creating data section chunks manually is not supported here, as there is no way to use them
    
    pub fn metadata_begin(&mut self) {
        self.set_chunk(Chunk::Metadata(MetadataChunk::new()));
    }

    pub fn checksum_begin(&mut self) {
        todo!(); // :trol: i dont wanna implement it right now
    }

    pub fn type_cast_begin(&mut self) {
        self.set_chunk(Chunk::TypeCast(TypeCastChunk::new()));
    }

    pub fn conditional_parsing_begin(&mut self) {
        self.set_chunk(Chunk::ConditionalParsing(ConditionalParsingChunk::new()));
    }

    // creating runtime constant chunks manually is not supported here, as there is no way to use them
    
    // all of these `[chunk]_end` functions are just for consistency
    // i feel like a C# dev right now
    pub fn code_end(&mut self) {
        self.chunk_end();
    }

    pub fn module_end(&mut self) {
        self.chunk_end();
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
        if let Some(chunk) = &self.cur_chunk {
            if let Some(prev) = &chunk.prev {
                self.cur_chunk = Some(prev.clone());
            } else {
                self.cur_chunk = None;
            }
        } else {
            serror!("attempted to end chunk but was not in one");
        }
    }

    fn set_chunk(&mut self, chunk: Chunk) {
        self.cur_chunk = Some(Rc::new(ChunkNode::new(chunk, self.cur_chunk.clone())));
    }

    pub fn emit(&mut self) -> Vec<u8> {
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
            bytes.append(&mut (index as u8).to_be_bytes().to_vec());
        } else if index < u16::MAX as usize {
            bytes.push((u16::BITS/8) as u8);
            bytes.append(&mut (index as u16).to_be_bytes().to_vec());
        } else if index < u32::MAX as usize {
            bytes.push((u32::BITS/8) as u8);
            bytes.append(&mut (index as u32).to_be_bytes().to_vec());
        } else {
            bytes.push((u64::BITS/8) as u8);
            bytes.append(&mut (index as u64).to_be_bytes().to_vec());
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
        out.append(&mut MAJOR_VERSION.to_be_bytes().to_vec());
        out.append(&mut MINOR_VERSION.to_be_bytes().to_vec());
        out.append(&mut PATCH_VERSION.to_be_bytes().to_vec());

        out.append(&mut Self::checksum(&body).to_be_bytes().to_vec());

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
