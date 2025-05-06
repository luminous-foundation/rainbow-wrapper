use std::process::exit;

use chunks::{Chunk, FuncRef, StructRef, Type};
use code::{CodeBlock, CodeChunk, Function};
use conditional_parsing::ConditionalParsingChunk;
use data::DataChunk;
use indexmap::IndexSet;
use instructions::Instruction;
use metadata::MetadataChunk;
use modules::{Extern, Import, Item, ModuleChunk};
use runtime_constants::{RuntimeConstant, RuntimeConstantChunk};
use type_cast::TypeCastChunk;

// is having this many files the best idea?
// i could combine them into less files and it would still work
// but maybe it's a good idea to do this for organization
pub mod chunks;
pub mod code;
pub mod conditional_parsing;
pub mod data;
pub mod instructions;
pub mod metadata;
pub mod modules;
pub mod runtime_constants;
pub mod type_cast;

pub use chunks::{Data, Number};

const MAJOR_VERSION: u16 = 1;
const MINOR_VERSION: u16 = 0;
const PATCH_VERSION: u16 = 0;

pub struct Wrapper {
    pub wrapper_core: WrapperCore,

    pub chunk_index: usize,
    pub element_index: usize,
    pub instruction_index: usize, // technically the same as `element_index`, but i think it's good to have names that match what the things are

    chunk_stack: Vec<Chunk>,
    chunk_indices: Vec<usize>,
    chunk_elements: Vec<usize>,
    raw_chunk_index: usize,

    argument_stack: Vec<Vec<(Type, String)>>,
    module_stack: Vec<String>,
    function_stack: Vec<String>,
}

macro_rules! verify_last_chunk {
    ($self:expr, $type:ident, $action:expr) => {{
        let chunk = $self.chunk_stack.last_mut();
        if let Some(chunk) = chunk {
            match chunk {
                Chunk::$type(c) => c,
                _ => {
                    fox::scritical!("attempted to {} while not in a {} chunk", $action, stringify!($type).to_lowercase());
                    fox::sinfo!("current chunk is a `{}` chunk", chunk.get_name());
                    exit(1);
                }
            }
        } else {
            fox::scritical!("attempted to {} while not in a chunk", $action);
            exit(1);
        }
    }};
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper {
            wrapper_core: WrapperCore::new(),

            chunk_index: 2,
            element_index: 0,
            instruction_index: 0,

            chunk_stack: Vec::new(),
            chunk_indices: Vec::new(),
            chunk_elements: Vec::new(),
            raw_chunk_index: 2,

            argument_stack: Vec::new(),
            module_stack: Vec::new(),
            function_stack: Vec::new(),
        }
    }

    // code chunk
    pub fn code_begin(&mut self) {
        if let Some(chunk) = self.chunk_stack.last() {
            match chunk {
                Chunk::Code(_) | Chunk::Module(_) => {}
                _ => { self.cannot_nest("attempted to nest code chunks with non-code/non-module chunks"); exit(1); }
            }
        }

        self.chunk_begin(Chunk::Code(CodeChunk::new(self.chunk_stack.len() > 0)));
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        let chunk = self.chunk_stack.last_mut();
        if let Some(chunk) = chunk {
            match chunk {
                Chunk::Code(c) => {
                    let last_block = c.blocks.last_mut();

                    let code_block = match last_block {
                        Some(CodeBlock::Code(code)) => code,
                        Some(CodeBlock::Scope(_)) | None => {
                            c.blocks.push(CodeBlock::Code(Vec::new()));
                            match c.blocks.last_mut().unwrap() {
                                CodeBlock::Code(code) => code,
                                _ => unreachable!("just pushed a code block, this shouldnt be possible")
                            }
                        }
                    };

                    self.element_index += 1;
                    code_block.push(inst);
                }
                _ => {
                    fox::scritical!("attempted to add an instruction while not in a code chunk");
                    fox::sinfo!("current chunk is a `{}` chunk", chunk.get_name());
                    exit(1);
                }
            }
        } else {
            fox::scritical!("attempted to add an instruction while not in a chunk");
            exit(1);
        }
        self.instruction_index = self.element_index;
    }
    
    pub fn function_start(&mut self, name: String, args: Vec<(Type, String)>) {
        self.function_stack.push(name);
        self.argument_stack.push(args);
        self.code_begin();
    }
    
    pub fn function_end(&mut self) -> FuncRef {
        let name = if let Some(s) = self.function_stack.pop() {
            s
        } else {
            fox::scritical!("no function name was found while trying to end a function");
            fox::sinfo!("(did you forget to start the function?)");
            exit(1);
        };
        let args = if let Some(s) = self.argument_stack.pop() {
            s
        } else {
            unreachable!("no function arguments were found while trying to end a function, this is probably a bug");
        };
        let body = if let Some(chunk) = self.chunk_stack.pop() {
            match chunk {
                Chunk::Code(c) => c,
                _ => {
                    fox::scritical!("attempted to create the function `{name}` while not in a code chunk");
                    fox::sinfo!("current chunk is a `{}` chunk", chunk.get_name());
                    fox::sinfo!("(did you forget to end that chunk?)");
                    exit(1);
                }
            }
        } else {
            fox::scritical!("attempted to create the function `{name}` while not in a chunk");
            fox::sinfo!("(did you end an extra chunk?)");
            exit(1);
        };

        let function = Function {
            name: name.clone(),
            args,
            body
        };

        let parent_chunk = verify_last_chunk!(self, Code, format!("create the function `{name}`"));
        parent_chunk.add_function(function);
        
        FuncRef { 
            module: self.module_stack.clone(), 
            function: self.function_stack.clone(), 
            name 
        }
    }

    pub fn code_end(&mut self) {
        // get the code chunk off of the chunk stack
        let chunk = self.chunk_stack.pop();

        // check if it exists
        if let Some(chunk) = chunk {
            // update the chunk index and element index
            self.chunk_index = self.chunk_indices.pop().expect("somehow didn't have a chunk index in the list, this is probably a bug");
            self.element_index = self.chunk_elements.pop().expect("somehow didn't have a chunk element count in the list, this is probably a bug");
            
            // get the `CodeChunk` out of the code chunk
            let chunk = match chunk {
                Chunk::Code(c) => c,
                _ => {
                    fox::scritical!("ran `code_end` when not in code chunk");
                    fox::sinfo!("current chunk is a `{}` chunk", chunk.get_name());
                    fox::sinfo!("(did you forget to end that chunk?)");
                    exit(1);
                }
            };
            
            // check for chunk nesting
            // get the previous code chunk off of the stack
            let prev = self.chunk_stack.last_mut();
            if let Some(prev) = prev { // check to see if it exists
                // check to make sure it's the correct type (code/module)
                match prev {
                    Chunk::Code(c) => c.add_scope(chunk),
                    Chunk::Module(c) => {
                        if let Some(_) = c.code_chunk {
                            fox::scritical!("attempted to add a code chunk to a module that already has one");
                            fox::sinfo!("module name is '{}'", c.name);
                            exit(1);
                        } else {
                            c.set_code(chunk);
                        }
                    }
                    _ => unreachable!(
                        "somehow nested a code block in a non-code block. this is probably a bug"
                    ),
                }
                // blocks count as elements
                self.element_index += 1;
            } else { // if it doesnt, add the current block normally
                self.chunk_index = self.raw_chunk_index;
                self.wrapper_core.add_chunk(Chunk::Code(chunk));
            }
        } else {
            fox::serror!("attempted to end chunk but was not in one");
        }
        // update instruction index
        self.instruction_index = self.element_index;
    }

    // module chunk
    pub fn module_begin(&mut self, name: String) {
        if let Some(chunk) = self.chunk_stack.last() {
            match chunk {
                Chunk::Module(_) => {}
                _ => { self.cannot_nest("attempted to nest module chunks with non-module chunks"); exit(1); }
            }
        }
        
        self.module_stack.push(name.clone());

        self.chunk_begin(Chunk::Module(ModuleChunk::new(
            name,
            self.chunk_stack.len() > 0,
        )));
        self.code_begin();
    }

    pub fn add_item_import(&mut self, path: String, name: String, item: Item, as_name: String) {
        let chunk = verify_last_chunk!(self, Module, "add item import");
        chunk.add_import(Import::ItemImport { path, name, item, as_name });
    }

    pub fn add_function_import(&mut self, path: String, name: String, func: FuncRef, as_name: String) {
        let chunk = verify_last_chunk!(self, Module, "add function import");
        chunk.add_import(Import::ItemImport { path, name, item: Item::Function(func), as_name });
    }

    pub fn add_struct_import(&mut self, path: String, name: String, strct: StructRef, as_name: String) {
        let chunk = verify_last_chunk!(self, Module, "add struct import");
        chunk.add_import(Import::ItemImport { path, name, item: Item::Struct(strct), as_name });
    }

    pub fn add_var_import(&mut self, path: String, name: String, var: String, as_name: String) {
        let chunk = verify_last_chunk!(self, Module, "add variable import");
        chunk.add_import(Import::ItemImport { path, name, item: Item::Variable(var), as_name });
    }

    pub fn add_export(&mut self, item: Item) {
        let chunk = verify_last_chunk!(self, Module, "add export");
        chunk.add_export(item);
    }

    pub fn add_function_export(&mut self, func: FuncRef) {
        let chunk = verify_last_chunk!(self, Module, "add function export");
        chunk.add_export(Item::Function(func));
    }

    pub fn add_struct_export(&mut self, strct: StructRef) {
        let chunk = verify_last_chunk!(self, Module, "add struct export");
        chunk.add_export(Item::Struct(strct));
    }

    pub fn add_var_export(&mut self, var: String) {
        let chunk = verify_last_chunk!(self, Module, "add variable export");
        chunk.add_export(Item::Variable(var));
    }

    pub fn add_extern(&mut self, extrn: Extern) {
        let chunk = verify_last_chunk!(self, Module, "add extern");
        chunk.add_extern(extrn);
    }

    pub fn module_end(&mut self) {
        self.code_end();

        let chunk = self.chunk_stack.pop();
        if let Some(chunk) = chunk {
            self.chunk_index = self.chunk_indices.pop().expect("somehow didn't have a chunk index in the list, this is probably a bug");
            self.element_index = self.chunk_elements.pop().expect("somehow didn't have a chunk element count in the list, this is probably a bug");
            let chunk = match chunk {
                Chunk::Module(c) => c,
                _ => {
                    fox::scritical!("ran `module_end` when not in module chunk");
                    fox::sinfo!("current chunk is a `{}` chunk", chunk.get_name());
                    fox::sinfo!("(did you forget to end that chunk?)");
                    exit(1);
                }
            };
            let prev = self.chunk_stack.last_mut();
            if let Some(prev) = prev {
                match prev {
                    Chunk::Module(c) => c.add_module(chunk),
                    _ => unreachable!(
                        "somehow nested a module block in a non-module block. this is probably a bug"
                    ),
                }
            } else {
                self.chunk_index = self.raw_chunk_index;
                self.wrapper_core.add_chunk(Chunk::Module(chunk));
            }
        } else {
            fox::serror!("attempted to end chunk but was not in one");
        }
        self.instruction_index = self.element_index;
    }
    // creating data section chunks manually is not supported here, as there is no way to use them

    // metadata chunk
    pub fn metadata_begin(&mut self) {
        if self.chunk_stack.len() > 0 {
            self.cannot_nest("attempted to nest non code/module chunks");
            fox::sinfo!("error occured while attempting to add metadata chunk");
            exit(1);
        }

        self.chunk_begin(Chunk::Metadata(MetadataChunk::new()));
    }

    // type cast chunk
    pub fn type_cast_begin(&mut self) {
        if self.chunk_stack.len() > 0 {
            self.cannot_nest("attempted to nest non code/module chunks");
            fox::sinfo!("error occured while attempting to add type cast chunk");
            exit(1);
        }

        self.chunk_begin(Chunk::TypeCast(TypeCastChunk::new()));
    }

    // conditional parsing chunk
    pub fn conditional_parsing_begin(&mut self) {
        if self.chunk_stack.len() > 0 {
            self.cannot_nest("attempted to nest non code/module chunks");
            fox::sinfo!("error occured while attempting to add conditional parsing chunk");
            exit(1);
        }

        self.chunk_begin(Chunk::ConditionalParsing(ConditionalParsingChunk::new()));
    }

    // creating runtime constant chunks manually is not supported here, as there is no way to use them

    // all of these `[chunk]_end` functions are just for consistency unless they have more functionality
    // i feel like a C# dev right now

    pub fn metadata_end(&mut self) {
        self.chunk_end();
    }

    pub fn type_cast_end(&mut self) {
        self.chunk_end();
    }

    pub fn conditional_parsing_end(&mut self) {
        self.chunk_end();
    }

    fn chunk_begin(&mut self, chunk: Chunk) {
        self.chunk_elements.push(self.element_index);
        self.element_index = 0;
        self.instruction_index = self.element_index;

        self.chunk_indices.push(self.raw_chunk_index);
        self.raw_chunk_index += 1;
        self.chunk_index = self.raw_chunk_index;

        self.chunk_stack.push(chunk);
    }

    fn chunk_end(&mut self) {
        if let Some(chunk) = self.chunk_stack.pop() {
            self.wrapper_core.add_chunk(chunk);
            self.chunk_indices.pop().expect("somehow didn't have a chunk index in the list, this is probably a bug");
            self.element_index = self.chunk_elements.pop().expect("somehow didn't have a chunk element count in the list, this is probably a bug");
            self.instruction_index = self.element_index;
        } else {
            fox::serror!("attempted to end chunk but was not in one");
        }
    }

    fn cannot_nest(&self, message: &str) {
        fox::scritical!("{message}");
        let mut chunks = String::new();
        let mut i = 0;
        for chunk in &self.chunk_stack {
            chunks += chunk.get_name();
            if i < self.chunk_stack.len() - 1 {
                chunks += ", ";
            }
            i += 1;
        }

        fox::sinfo!("chunk list contains [{chunks}]");
    }

    pub fn emit(&mut self) -> Vec<u8> {
        if self.chunk_stack.len() > 0 {
            // TODO: better wording needed (what is a "wrapper chunk list"?)
            fox::swarn!("wrapper chunk list contained unfinished chunks, these chunks will be discarded");
            
            let mut chunks = String::new();
            let mut i = 0;
            for chunk in &self.chunk_stack {
                chunks += chunk.get_name();
                if i < self.chunk_stack.len() - 1 {
                    chunks += ", ";
                }
                i += 1;
            }

            fox::sinfo!("contained [{chunks}]");
        }

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
            bytes.push((u8::BITS / 8) as u8);
            bytes.append(&mut (index as u8).to_ne_bytes().to_vec());
        } else if index < u16::MAX as usize {
            bytes.push((u16::BITS / 8) as u8);
            bytes.append(&mut (index as u16).to_ne_bytes().to_vec());
        } else if index < u32::MAX as usize {
            bytes.push((u32::BITS / 8) as u8);
            bytes.append(&mut (index as u32).to_ne_bytes().to_vec());
        } else {
            bytes.push((u64::BITS / 8) as u8);
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
        let mut post_body: Vec<u8> = Vec::new();

        // the chunks are filled with dummy sets as they just need to exist to get populated later
        self.chunks.insert(0, Chunk::Data(DataChunk::new()));
        self.chunks.insert(1, Chunk::RuntimeConstant(RuntimeConstantChunk::new()));

        let mut i = 2;
        while i < self.chunks.len() {
            let chunk = self.chunks[i].clone(); // .clone() :why:
            post_body.append(&mut chunk.to_bytes(self));
            i += 1;
        }

        // we need to do this as `self.data` only gets populated from previous chunks being converted to bytes
        self.chunks[0] = Chunk::Data(DataChunk::from_set(&self.data));
        self.chunks[1] = Chunk::RuntimeConstant(RuntimeConstantChunk::from_set(&self.runtime_constants));
        let mut pre_body: Vec<u8> = Vec::new();
        pre_body.append(&mut self.chunks[0].clone().to_bytes(self));
        pre_body.append(&mut self.chunks[1].clone().to_bytes(self));

        let mut body = Vec::new();
        body.append(&mut pre_body);
        body.append(&mut post_body);
        
        // RBB file header
        let mut out: Vec<u8> = b"RBB".to_vec();
        out.append(&mut MAJOR_VERSION.to_ne_bytes().to_vec());
        out.append(&mut MINOR_VERSION.to_ne_bytes().to_vec());
        out.append(&mut PATCH_VERSION.to_ne_bytes().to_vec());

        out.append(&mut Self::checksum(&body).to_ne_bytes().to_vec());

        let endianness = if self.endianness { 1 } else { 0 };
        out.push(endianness);

        let compressed = if self.compressed { 1 } else { 0 };
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
