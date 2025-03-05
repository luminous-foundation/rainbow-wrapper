use crate::{checksum::ChecksumChunk, code::CodeChunk, conditional_parsing::ConditionalParsingChunk, data::DataChunk, metadata::MetadataChunk, modules::ModuleChunk, type_cast::TypeCastChunk};

/// The `Chunk` enum
/// Defines every type of data chunk present in a Rainbow file
pub enum Chunk {
    Code(CodeChunk),
    Module(ModuleChunk),
    Data(DataChunk),
    Metadata(MetadataChunk),
    Checksum(ChecksumChunk),
    TypeCast(TypeCastChunk),
    ConditionalParsing(ConditionalParsingChunk),
}

// stuff shared between chunks
pub enum Type {
    Void,
    U8,
    U16,
    U32,
    U64,
    UXX(u64), // NOTE: in future versions this will be truly unlimited, but we're not there yet
    I8,
    I16,
    I32,
    I64,
    IXX(u64),
    F8,
    F16,
    F32,
    F64,
    FXX(u64),
    Struct(StructRef),
    Name,
    Type,
}

pub enum Data {
    Number(Number),
    Name(String),
    Array(Vec<Data>),
    FunctionRef(FunctionRef),
    StructRef(StructRef),
    ComplexType(Type, TypeData),
}

pub enum TypeData {
    StructData(bool, Box<Data>),
    DynamicData(bool, Box<Data>),
}

pub enum Number {
    Signed(i64),
    Unsigned(u64),
    Decimal(f64),
}
pub struct FunctionRef {
    pub module: Vec<String>,
    pub function: Vec<String>,
    pub name: String,
}

pub struct StructRef {
    pub module: Vec<String>,
    pub function: Vec<String>,
    pub name: String,
}
