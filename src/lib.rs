use chunks::{Chunk, Data};
use data::DataChunk;
use indexmap::IndexSet;
use runtime_constants::{RuntimeConstant, RuntimeConstantChunk};

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
    pub compressed: bool,
    pub chunks: Vec<Chunk>,

    data: IndexSet<Data>,
    runtime_constants: IndexSet<RuntimeConstant>,

    signed: bool,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper { chunks: Vec::new(), data: IndexSet::new(), runtime_constants: IndexSet::new(), compressed: false, signed: false }
    }

    // TODO: optimize byte size to reduce file size
    /// Convert a number into a data section compatible number,
    /// consisting of {size} {bytes}
    pub fn index_to_bytes(index: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push((usize::BITS/8) as u8);
        bytes.append(&mut index.to_be_bytes().to_vec());

        return bytes;
    }

    pub fn add_data(&mut self, data: Data) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x01, 0x00]; // default data section index
        bytes.append(&mut Wrapper::index_to_bytes(self.data.len()));

        self.data.insert(data);
        return bytes;
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> Vec<u8> {
        self.chunks.push(chunk);

        return Wrapper::index_to_bytes(self.chunks.len() - 1);
    }

    pub fn add_runtime_constant(&mut self, constant: RuntimeConstant) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut Wrapper::index_to_bytes(constant.name.len()));
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
            let mut chunk = self.chunks[i].clone();
            body.append(&mut chunk.to_bytes(self));
            i += 1;
        }

        // RBB file header
        let mut out: Vec<u8> = b"RBB".to_vec();
        out.append(&mut MAJOR_VERSION.to_be_bytes().to_vec());
        out.append(&mut MINOR_VERSION.to_be_bytes().to_vec());
        out.append(&mut PATCH_VERSION.to_be_bytes().to_vec());

        out.append(&mut Self::checksum(&body).to_be_bytes().to_vec());

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

        out.append(&mut Wrapper::index_to_bytes(body.len()));
        
        out.append(&mut body);

        return out;
    }
}
