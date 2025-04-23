use crate::{chunks::{Chunk, Data, FuncRef, StructRef, Type}, code::CodeChunk, WrapperCore};

#[derive(Clone)]
pub struct ModuleChunk {
    pub name: String,
    pub has_parent: bool,

    pub blocks: Vec<ModuleBlock>,
    pub imports: Vec<Import>,
    pub exports: Vec<Item>,
    pub externs: Vec<Extern>,
}

impl ModuleChunk {
    pub fn new(name: String, has_parent: bool) -> ModuleChunk {
        ModuleChunk {
            name,
            has_parent,

            blocks:  Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            externs: Vec::new(),
        }
    }

    pub fn to_bytes(&mut self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        
        bytes.append(&mut wrapper.add_data(Data::Name(self.name.clone())));
        
        if self.has_parent {
            bytes.push(0x01);
        } else {
            bytes.push(0x00);
        }

        let mut blocks: Vec<u8> = Vec::new();
        for block in &mut self.blocks {
            blocks.append(&mut block.to_bytes(wrapper));
        }

        bytes.push(0x00);
        bytes.append(&mut WrapperCore::index_to_bytes(blocks.len()));
        bytes.append(&mut blocks);

        let mut imports: Vec<u8> = Vec::new();
        for import in &self.imports {
            imports.append(&mut import.to_bytes(wrapper));
        }

        bytes.push(0x01);
        bytes.append(&mut WrapperCore::index_to_bytes(imports.len()));
        bytes.append(&mut imports);

        let mut exports: Vec<u8> = Vec::new();
        for export in &self.exports {
            exports.append(&mut export.to_bytes(wrapper));
        }

        bytes.push(0x02);
        bytes.append(&mut WrapperCore::index_to_bytes(exports.len()));
        bytes.append(&mut exports);

        let mut externs: Vec<u8> = Vec::new();
        for item in &self.externs {
            externs.append(&mut item.to_bytes(wrapper));
        }

        bytes.push(0x03);
        bytes.append(&mut WrapperCore::index_to_bytes(externs.len()));
        bytes.append(&mut externs);

        return bytes;
    }
}

#[derive(Clone)]
pub enum ModuleBlock {
    Code(CodeChunk),
    Module(ModuleChunk),
}

impl ModuleBlock {
    pub fn to_bytes(&mut self, wrapper: &mut WrapperCore) -> Vec<u8> {
        match self {
            ModuleBlock::Code(chunk) => { chunk.has_parent = true; wrapper.add_chunk(Chunk::Code(chunk.clone())) }
            ModuleBlock::Module(chunk) => { chunk.has_parent = true; wrapper.add_chunk(Chunk::Module(chunk.clone())) }
        }
    }
}

#[derive(Clone)]
pub enum Import {
    ModuleImport(String, String, String), // path, name, as name
    ItemImport(String, String, Item, String), // path, name, item, as name
}

impl Import {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Import::ModuleImport(path, name, as_name) => {
                bytes.append(&mut wrapper.add_data(Data::Name(path.clone())));
                bytes.append(&mut wrapper.add_data(Data::Name(name.clone())));
                bytes.append(&mut wrapper.add_data(Data::Name(as_name.clone())));
            }
            Import::ItemImport(path, name, item, as_name) => {
                bytes.append(&mut wrapper.add_data(Data::Name(path.clone())));
                bytes.append(&mut wrapper.add_data(Data::Name(name.clone())));
                bytes.append(&mut item.to_bytes(wrapper));
                bytes.append(&mut wrapper.add_data(Data::Name(as_name.clone())));
            }
        }

        return bytes;
    } 
}

#[derive(Clone)]
pub enum Item {
    Function(FuncRef),
    Struct(StructRef),
    Variable(String),
}

impl Item {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        match self {
            Item::Function(func_ref) => wrapper.add_data(Data::FuncRef(func_ref.clone())),
            Item::Struct(struct_ref) => wrapper.add_data(Data::StructRef(struct_ref.clone())),
            Item::Variable(name)     => wrapper.add_data(Data::Name(name.clone()))
        }
    }
}

#[derive(Clone)]
pub struct Extern {
    pub path: String,
    pub name: String,
    pub args: Vec<Type>,
    pub as_name: String,
}

impl Extern {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut wrapper.add_data(Data::Name(self.path.clone())));
        bytes.append(&mut wrapper.add_data(Data::Name(self.name.clone())));

        for arg in &self.args {
            bytes.append(&mut arg.to_bytes(wrapper));
        }
        bytes.push(0xFA);

        bytes.append(&mut wrapper.add_data(Data::Name(self.as_name.clone())));

        return bytes;
    }
}
