use std::hash::{Hash, Hasher};

use half::f16;

use crate::{checksum::ChecksumChunk, code::CodeChunk, conditional_parsing::ConditionalParsingChunk, data::DataChunk, metadata::MetadataChunk, modules::ModuleChunk, runtime_constants::RuntimeConstantChunk, type_cast::TypeCastChunk, Wrapper};

/// The `Chunk` enum
/// Defines every type of data chunk present in a Rainbow file
#[derive(Clone)]
pub enum Chunk {
    Code(CodeChunk),
    Module(ModuleChunk),
    Data(DataChunk),
    Metadata(MetadataChunk),
    Checksum(ChecksumChunk),
    TypeCast(TypeCastChunk),
    ConditionalParsing(ConditionalParsingChunk),
    RuntimeConstant(RuntimeConstantChunk),
}

impl Chunk {
    pub fn to_bytes(&mut self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut chunk_bytes = match self {
            Chunk::Code(c)               => c.to_bytes(wrapper),
            Chunk::Module(c)             => c.to_bytes(wrapper),
            Chunk::Data(c)               => c.to_bytes(wrapper),
            Chunk::Metadata(c)           => c.to_bytes(),
            Chunk::Checksum(c)           => c.to_bytes(),
            Chunk::TypeCast(c)           => c.to_bytes(wrapper),
            Chunk::ConditionalParsing(c) => c.to_bytes(wrapper),
            Chunk::RuntimeConstant(c)    => c.to_bytes(wrapper),
        };

        let mut bytes: Vec<u8> = Vec::new();
        match self {
            Chunk::Code(_)               => bytes.push(0x00),
            Chunk::Module(_)             => bytes.push(0x01),
            Chunk::Data(_)               => bytes.push(0x02),
            Chunk::Metadata(_)           => bytes.push(0x03),
            Chunk::Checksum(_)           => bytes.push(0x04),
            Chunk::TypeCast(_)           => bytes.push(0x05),
            Chunk::ConditionalParsing(_) => bytes.push(0x06),
            Chunk::RuntimeConstant(_)    => bytes.push(0x07),
        };

        bytes.append(&mut Wrapper::index_to_bytes(chunk_bytes.len()));
        bytes.append(&mut chunk_bytes);

        return bytes;
    }
}

// stuff shared between chunks
#[derive(Clone, Hash, PartialEq, Eq)]
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

macro_rules! vex {
    ($($element:expr),* $(,)? ; $($vec:expr),*) => {{
        let mut result = Vec::<u8>::new();
        $(
            result.push($element);
        )*
        $(
            result.append(&mut $vec);
        )*
        result
    }};
}

impl Type {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        return match self {
            Type::UXX(_) | Type::IXX(_) | Type::FXX(_) => wrapper.add_data(Data::ComplexType(self.clone())),
            Type::Struct(r) => wrapper.add_data(Data::StructRef(r.clone())),

            _ => self.to_bytes_raw(wrapper)
        }
    }

    pub fn to_bytes_raw(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        return match self {
            Type::Void      => vec![0x00],

            Type::U8        => vec![0x01],
            Type::U16       => vec![0x02],
            Type::U32       => vec![0x03],
            Type::U64       => vec![0x04],
            Type::UXX(s)    => vex![0x05, 0x08 ; s.to_be_bytes().to_vec()],

            Type::I8        => vec![0x06],
            Type::I16       => vec![0x07],
            Type::I32       => vec![0x08],
            Type::I64       => vec![0x09],
            Type::IXX(s)    => vex![0x0A, 0x08 ; s.to_be_bytes().to_vec()],

            Type::F8        => vec![0x0B],
            Type::F16       => vec![0x0C],
            Type::F32       => vec![0x0D],
            Type::F64       => vec![0x0E],
            Type::FXX(s)    => vex![0x0F, 0x08 ; s.to_be_bytes().to_vec()],

            Type::Struct(r) => vex![0x10 ; wrapper.add_data(Data::StructRef(r.clone()))],
            Type::Name      => vec![0x11],
            Type::Type      => vec![0x12],
        };
    }
}

#[derive(Clone)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    UXX(Vec<u8>),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    IXX(Vec<u8>),

    F8(u8),
    F16(f16),
    F32(f32),
    F64(f64),
    FXX(Vec<u8>),
}

impl Number {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Number::U8(n)  => vec![*n],
            Number::U16(n) => n.to_be_bytes().to_vec(),
            Number::U32(n) => n.to_be_bytes().to_vec(),
            Number::U64(n) => n.to_be_bytes().to_vec(),
            Number::UXX(n) => n.clone(),

            Number::I8(n)  => n.to_be_bytes().to_vec(),
            Number::I16(n) => n.to_be_bytes().to_vec(),
            Number::I32(n) => n.to_be_bytes().to_vec(),
            Number::I64(n) => n.to_be_bytes().to_vec(),
            Number::IXX(n) => n.clone(),

            Number::F8(n)  => vec![*n],
            Number::F16(n) => n.to_be_bytes().to_vec(),
            Number::F32(n) => n.to_be_bytes().to_vec(),
            Number::F64(n) => n.to_be_bytes().to_vec(),
            Number::FXX(n) => n.clone(),
        }
    }
}

impl Hash for Number {
    fn hash<H>(&self, h: &mut H) where H: Hasher {
        match self {
            Number::U8(n)  => n.hash(h),
            Number::U16(n) => n.hash(h),
            Number::U32(n) => n.hash(h),
            Number::U64(n) => n.hash(h),
            Number::UXX(n) => n.hash(h),

            Number::I8(n)  => n.hash(h),
            Number::I16(n) => n.hash(h),
            Number::I32(n) => n.hash(h),
            Number::I64(n) => n.hash(h),
            Number::IXX(n) => n.hash(h),

            Number::F8(n)  => n.hash(h),
            Number::F16(n) => n.to_bits().hash(h),
            Number::F32(n) => n.to_bits().hash(h),
            Number::F64(n) => n.to_bits().hash(h),
            Number::FXX(n) => n.hash(h),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        match (self, other) {
            (Number::U8(n),  Number::U8(n2))  => return n == n2,
            (Number::U8(_),  _) => return false,
            (Number::U16(n), Number::U16(n2)) => return n == n2,
            (Number::U16(_), _) => return false,
            (Number::U32(n), Number::U32(n2)) => return n == n2,
            (Number::U32(_), _) => return false,
            (Number::U64(n), Number::U64(n2)) => return n == n2,
            (Number::U64(_), _) => return false,
            (Number::UXX(n), Number::UXX(n2)) => return n == n2,
            (Number::UXX(_), _) => return false,

            (Number::I8(n),  Number::I8(n2))  => return n == n2,
            (Number::I8(_),  _) => return false,
            (Number::I16(n), Number::I16(n2)) => return n == n2,
            (Number::I16(_), _) => return false,
            (Number::I32(n), Number::I32(n2)) => return n == n2,
            (Number::I32(_), _) => return false,
            (Number::I64(n), Number::I64(n2)) => return n == n2,
            (Number::I64(_), _) => return false,
            (Number::IXX(n), Number::IXX(n2)) => return n == n2,
            (Number::IXX(_), _) => return false,

            (Number::F8(n),  Number::F8(n2))  => return n == n2,
            (Number::F8(_),  _) => return false,
            (Number::F16(n), Number::F16(n2)) => return n == n2,
            (Number::F16(_), _) => return false,
            (Number::F32(n), Number::F32(n2)) => return n == n2,
            (Number::F32(_), _) => return false,
            (Number::F64(n), Number::F64(n2)) => return n == n2,
            (Number::F64(_), _) => return false,
            (Number::FXX(n), Number::FXX(n2)) => return n == n2,
            (Number::FXX(_), _) => return false,
        }
    }
}

impl Eq for Number {}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Data {
    Number(Number),
    Name(String),
    Array(Vec<Data>),
    FunctionRef(FunctionRef),
    StructRef(StructRef),
    ComplexType(Type),
}

impl Data {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Data::Number(number) => {
                bytes.push(0x00);
                bytes.append(&mut number.to_bytes());
            }
            Data::Name(name) => {
                bytes.push(0x01);
                bytes.append(&mut Wrapper::index_to_bytes(name.len()));
                bytes.append(&mut name.as_bytes().to_vec());
            }
            Data::Array(values) => {
                bytes.push(0x02);
                bytes.append(&mut Wrapper::index_to_bytes(values.len()));
                for val in values {
                    bytes.append(&mut val.to_bytes(wrapper));
                }
            }
            Data::FunctionRef(funcref) => {
                bytes.push(0x03);
                bytes.append(&mut funcref.to_bytes(wrapper));
            }
            Data::StructRef(structref) => {
                bytes.push(0x04);
                bytes.append(&mut structref.to_bytes(wrapper));
            }
            Data::ComplexType(typ) => {
                bytes.push(0x05);
                bytes.append(&mut typ.to_bytes_raw(wrapper));
            }
        }

        return bytes;
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionRef {
    pub module: Vec<String>,
    pub function: Vec<String>,
    pub name: String,
}

impl FunctionRef {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut Wrapper::index_to_bytes(self.module.len()));
        for name in &self.module {
            bytes.append(&mut wrapper.add_data(Data::Name(name.clone())));
        }
        
        bytes.append(&mut Wrapper::index_to_bytes(self.function.len()));
        for name in &self.function {
            bytes.append(&mut wrapper.add_data(Data::Name(name.clone())));
        }

        bytes.append(&mut wrapper.add_data(Data::Name(self.name.clone())));

        return bytes;
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StructRef {
    pub module: Vec<String>,
    pub function: Vec<String>,
    pub name: String,
}

impl StructRef {
    pub fn to_bytes(&self, wrapper: &mut Wrapper) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut Wrapper::index_to_bytes(self.module.len()));
        for name in &self.module {
            bytes.append(&mut wrapper.add_data(Data::Name(name.clone())));
        }
        
        bytes.append(&mut Wrapper::index_to_bytes(self.function.len()));
        for name in &self.function {
            bytes.append(&mut wrapper.add_data(Data::Name(name.clone())));
        }

        bytes.append(&mut wrapper.add_data(Data::Name(self.name.clone())));

        return bytes;
    }
}
