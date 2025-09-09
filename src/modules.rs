use std::process::exit;

use crate::{chunks::{Chunk, Data, FuncRef, StructRef, Type}, code::CodeChunk, WrapperCore};

#[derive(Debug, Clone)]
pub struct ModuleChunk {
    pub name: String,
    pub has_parent: bool,

    pub code_chunk: Option<CodeChunk>,

    pub submodules: Vec<ModuleChunk>,
    pub imports: Vec<Import>,
    pub exports: Vec<Item>,
    pub externs: Vec<Extern>,
}

impl ModuleChunk {
    pub fn new(name: String, has_parent: bool) -> ModuleChunk {
        ModuleChunk {
            name,
            has_parent,

            code_chunk: None,

            submodules:  Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            externs: Vec::new(),
        }
    }

    // i think these are only functions for consistency with other code
    // honestly i dont remember but i started doing functions so im gonna keep doing functions
    pub fn set_code(&mut self, mut chunk: CodeChunk) {
        chunk.has_parent = true;
        self.code_chunk = Some(chunk);
    }

    pub fn add_module(&mut self, mut chunk: ModuleChunk) {
        chunk.has_parent = true;
        self.submodules.push(chunk);
    }

    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    pub fn add_export(&mut self, export: Item) {
        self.exports.push(export);
    }

    pub fn add_extern(&mut self, xtern: Extern) {
        self.externs.push(xtern);
    }

    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        
        bytes.append(&mut wrapper.add_data(Data::Text(self.name)));
        
        if self.has_parent {
            bytes.push(0x01);
        } else {
            bytes.push(0x00);
        }

        if let Some(chunk) = self.code_chunk {
            bytes.append(&mut wrapper.add_chunk(Chunk::Code(chunk)));
        } else {
            fox::scritical!("encountered a module chunk with no associated code chunk");
            fox::scritical!("this is likely a bug");
            exit(1);
        }

        if self.submodules.len() > 0 {
            let mut blocks: Vec<u8> = Vec::new();
            for module in self.submodules {
                blocks.append(&mut wrapper.add_chunk(Chunk::Module(module))); 
            }

            bytes.push(0x00);
            bytes.append(&mut WrapperCore::num_to_bytes(blocks.len()));
            bytes.append(&mut blocks);
        }

        if self.imports.len() > 0 {
            let mut imports: Vec<u8> = Vec::new();
            for import in self.imports {
                imports.append(&mut import.to_bytes(wrapper));
            }

            bytes.push(0x01);
            bytes.append(&mut WrapperCore::num_to_bytes(imports.len()));
            bytes.append(&mut imports);
        }

        if self.exports.len() > 0 {
            let mut exports: Vec<u8> = Vec::new();
            for export in self.exports {
                exports.append(&mut export.to_bytes(wrapper));
            }

            bytes.push(0x02);
            bytes.append(&mut WrapperCore::num_to_bytes(exports.len()));
            bytes.append(&mut exports);
        }

        if self.externs.len() > 0 {
            let mut externs: Vec<u8> = Vec::new();
            for item in self.externs {
                externs.append(&mut item.to_bytes(wrapper));
            }

            bytes.push(0x03);
            bytes.append(&mut WrapperCore::num_to_bytes(externs.len()));
            bytes.append(&mut externs);
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum Import {
    ModuleImport {
        path: String,
        name: String,
        as_name: String
    },
    ItemImport {
        path: String,
        name: String,
        item: Item,
        as_name: String
    },
}

impl Import {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Import::ModuleImport { path, name, as_name } => {
                bytes.append(&mut wrapper.add_data(Data::Text(path)));
                bytes.append(&mut wrapper.add_data(Data::Text(name)));
                bytes.append(&mut wrapper.add_data(Data::Text(as_name)));
            }
            Import::ItemImport { path, name, item, as_name } => {
                bytes.append(&mut wrapper.add_data(Data::Text(path)));
                bytes.append(&mut wrapper.add_data(Data::Text(name)));
                bytes.append(&mut item.to_bytes(wrapper));
                bytes.append(&mut wrapper.add_data(Data::Text(as_name)));
            }
        }

        return bytes;
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(FuncRef),
    Struct(StructRef),
    Variable(String),
}

impl Item {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        match self {
            Item::Function(func_ref) => wrapper.add_data(Data::FuncRef(func_ref)),
            Item::Struct(struct_ref) => wrapper.add_data(Data::StructRef(struct_ref)),
            Item::Variable(name)     => wrapper.add_data(Data::Text(name))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Extern {
    pub path: String,
    pub name: String,
    pub ret_type: Type,
    pub args: Vec<Type>,
    pub as_name: String,
}

impl Extern {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut wrapper.add_data(Data::Text(self.path)));
        bytes.append(&mut wrapper.add_data(Data::Text(self.name)));

        bytes.append(&mut self.ret_type.to_bytes(wrapper));

        bytes.append(&mut WrapperCore::num_to_bytes(self.args.len()));
        for arg in &self.args {
            bytes.append(&mut arg.to_bytes(wrapper));
        }

        bytes.append(&mut wrapper.add_data(Data::Text(self.as_name)));

        return bytes;
    }
}
