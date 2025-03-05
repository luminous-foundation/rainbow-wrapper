use chunks::Chunk;

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

    signed: bool,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper { signed: false, chunks: Vec::new(), compressed: false }
    }

    pub fn emit(&self) -> Vec<u8> {
        // RBB file header
        let mut out: Vec<u8> = b"RBB".to_vec();
        out.append(&mut MAJOR_VERSION.to_be_bytes().to_vec());
        out.append(&mut MINOR_VERSION.to_be_bytes().to_vec());
        out.append(&mut PATCH_VERSION.to_be_bytes().to_vec());

        out.append(&mut vec![0x12, 0x34, 0x56, 0x78]); // placeholder checksum

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

        out.push(1); // placeholder data index width
        out.push(0); // placeholder size

        return out;
    }
}
