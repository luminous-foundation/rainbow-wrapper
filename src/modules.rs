use crate::code::{Arg, CodeChunk};

pub struct ModuleChunk {
    pub blocks: Vec<ModuleBlock>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub externs: Vec<Extern>,
}

pub enum ModuleBlock {
    Code(CodeChunk),
    Module(ModuleChunk),
}

pub enum Import {
    ModuleImport(String, String), // path, as name
    ItemImport(String, String, String), // path, item, as name
}

pub struct Export {
    pub item_name: String,
    pub as_name: String,
}

pub struct Extern {
    pub path: String,
    pub name: String,
    pub args: Vec<Arg>,
    pub as_name: String,
}
