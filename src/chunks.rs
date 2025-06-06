use std::{fmt::Display, hash::{Hash, Hasher}};

use half::f16;

use crate::{code::CodeChunk, conditional_parsing::ConditionalParsingChunk, data::DataChunk, imports::ImportChunk, metadata::MetadataChunk, modules::ModuleChunk, runtime_constants::RuntimeConstantChunk, type_cast::TypeCastChunk, WrapperCore};

// VEc eXtended
// Allows addition of Vecs to inline vecs
#[macro_export]
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

/// The `Chunk` enum
/// Defines every type of data chunk present in a Rainbow file
#[derive(Debug, Clone)]
pub enum Chunk {
    Code(CodeChunk),
    Module(ModuleChunk),
    Data(DataChunk),
    Metadata(MetadataChunk),
    TypeCast(TypeCastChunk),
    ConditionalParsing(ConditionalParsingChunk),
    RuntimeConstant(RuntimeConstantChunk),
    Imports(ImportChunk),
}

impl Chunk {
    pub fn to_bytes(self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        match self {
            Chunk::Code(_)               => bytes.push(0x00),
            Chunk::Module(_)             => bytes.push(0x01),
            Chunk::Data(_)               => bytes.push(0x02),
            Chunk::Metadata(_)           => bytes.push(0x03),
            Chunk::TypeCast(_)           => bytes.push(0x04),
            Chunk::ConditionalParsing(_) => bytes.push(0x05),
            Chunk::RuntimeConstant(_)    => bytes.push(0x06),
            Chunk::Imports(_)            => bytes.push(0x07),
        };

        let mut sub_one = false;
        let mut chunk_bytes = match self {
            Chunk::Code(c)               => { sub_one = true; c.to_bytes(wrapper) },
            Chunk::Module(c)             => { sub_one = true; c.to_bytes(wrapper) },
            Chunk::Data(c)               => c.to_bytes(wrapper),
            Chunk::Metadata(c)           => c.to_bytes(),
            Chunk::TypeCast(c)           => c.to_bytes(wrapper),
            Chunk::ConditionalParsing(c) => c.to_bytes(wrapper),
            Chunk::RuntimeConstant(c)    => c.to_bytes(wrapper),
            Chunk::Imports(c)            => c.to_bytes(wrapper),
        };

        if sub_one {
            bytes.append(&mut WrapperCore::num_to_bytes(chunk_bytes.len() - 1));
        } else {
            bytes.append(&mut WrapperCore::num_to_bytes(chunk_bytes.len()));
        }
        bytes.append(&mut chunk_bytes);

        return bytes;
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Chunk::Code(_)               => "Code",
            Chunk::Module(_)             => "Module",
            Chunk::Data(_)               => "Data",
            Chunk::Metadata(_)           => "Metadata",
            Chunk::TypeCast(_)           => "TypeCast",
            Chunk::ConditionalParsing(_) => "ConditionalParsing",
            Chunk::RuntimeConstant(_)    => "RuntimeConstant",
            Chunk::Imports(_)            => "Import",
        }
    }
}

// stuff shared between chunks
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    FXX(u64, u64), // exponent bits, mantissa bits
    Struct(StructRef),
    Name,
    Type,
    FuncRef,
    StructRef,

    // modifiers
    Pointer(Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::UXX(size) => write!(f, "uXX({size})"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::IXX(size) => write!(f, "iXX({size})"),
            Type::F8 => write!(f, "f8"),
            Type::F16 => write!(f, "f16"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::FXX(exponent, mantissa) => write!(f, "fXX({exponent}, {mantissa})"),
            Type::Struct(structref) => write!(f, "struct({})", structref.name),
            Type::Name => write!(f, "name"),
            Type::Type => write!(f, "type"),
            Type::FuncRef => write!(f, "funcref"),
            Type::StructRef => write!(f, "structref"),

            // modifiers
            Type::Pointer(t) => write!(f, "*{}", format!("{t}")), // yummy recursion
        }
    }
}

impl Type {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        return match self {
            Type::UXX(_)    => vex![0x05 ; wrapper.add_data(Data::ComplexType(self.clone()))],
            Type::IXX(_)    => vex![0x0A ; wrapper.add_data(Data::ComplexType(self.clone()))],
            Type::FXX(_, _) => vex![0x0F ; wrapper.add_data(Data::ComplexType(self.clone()))],
            Type::Struct(r) => vex![0x10 ; wrapper.add_data(Data::StructRef(r.clone()))],

            _ => self.to_bytes_raw(wrapper)
        }
    }

    pub fn get_byte(&self) -> Vec<u8> {
        match self {
            Type::Void       => vec![0x00],

            Type::U8         => vec![0x01],
            Type::U16        => vec![0x02],
            Type::U32        => vec![0x03],
            Type::U64        => vec![0x04],
            Type::UXX(_)     => vec![0x05],

            Type::I8         => vec![0x06],
            Type::I16        => vec![0x07],
            Type::I32        => vec![0x08],
            Type::I64        => vec![0x09],
            Type::IXX(_)     => vec![0x0A],

            Type::F8         => vec![0x0B],
            Type::F16        => vec![0x0C],
            Type::F32        => vec![0x0D],
            Type::F64        => vec![0x0E],
            Type::FXX(_, _)  => vec![0x0F],

            Type::Struct(_)  => vec![0x10],
            Type::Name       => vec![0x11],
            Type::Type       => vec![0x12],
            Type::FuncRef    => vec![0x13],
            Type::StructRef  => vec![0x14],

            // modifiers
            Type::Pointer(_) => vec![0x15],
        }
    }

    // TODO (low priority): optimize sizes of numbers to be smallest fit
    pub fn to_bytes_raw(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        match self {
            Type::UXX(s)     => vex![0x05, ; wrapper.add_data(Data::Number(Number::U64(*s)))],
            Type::IXX(s)     => vex![0x0A, ; wrapper.add_data(Data::Number(Number::U64(*s)))],
            Type::FXX(e, m)  => vex![0x0F, ; wrapper.add_data(Data::Number(Number::U64(*e))), wrapper.add_data(Data::Number(Number::U64(*m)))],
            Type::Struct(r)  => vex![0x10 ; wrapper.add_data(Data::StructRef(r.clone()))],

            // modifiers
            Type::Pointer(t) => vex![0x15 ; t.to_bytes(wrapper)],

            _ => self.get_byte(),
        }
    }
}

#[derive(Debug, Clone)]
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
    FXX(Vec<u8>, u64, u64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U8(n)  => write!(f, "u8({n:?})"),
            Number::U16(n) => write!(f, "u16({n:?})"),
            Number::U32(n) => write!(f, "u32({n:?})"),
            Number::U64(n) => write!(f, "u64({n:?})"),
            Number::UXX(n) => write!(f, "uxx({n:?})"),

            Number::I8(n)  => write!(f, "i8({n:?})"),
            Number::I16(n) => write!(f, "i16({n:?})"),
            Number::I32(n) => write!(f, "i32({n:?})"),
            Number::I64(n) => write!(f, "i64({n:?})"),
            Number::IXX(n) => write!(f, "ixx({n:?})"),

            Number::F8(n)  => write!(f, "f8({n:?})"),
            Number::F16(n) => write!(f, "f16({n:?})"),
            Number::F32(n) => write!(f, "f32({n:?})"),
            Number::F64(n) => write!(f, "f64({n:?})"),
            Number::FXX(n, _, _) => write!(f, "fxx({n:?})"),
        }
    }
}

impl Number {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Number::U8(n)  => vec![*n],
            Number::U16(n) => n.to_ne_bytes().to_vec(),
            Number::U32(n) => n.to_ne_bytes().to_vec(),
            Number::U64(n) => n.to_ne_bytes().to_vec(),
            Number::UXX(n) => n.clone(),

            Number::I8(n)  => n.to_ne_bytes().to_vec(),
            Number::I16(n) => n.to_ne_bytes().to_vec(),
            Number::I32(n) => n.to_ne_bytes().to_vec(),
            Number::I64(n) => n.to_ne_bytes().to_vec(),
            Number::IXX(n) => n.clone(),

            Number::F8(n)  => vec![*n],
            Number::F16(n) => n.to_ne_bytes().to_vec(),
            Number::F32(n) => n.to_ne_bytes().to_vec(),
            Number::F64(n) => n.to_ne_bytes().to_vec(),
            Number::FXX(n, _, _) => n.clone(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Number::U8(_)  => Type::U8,
            Number::U16(_) => Type::U16,
            Number::U32(_) => Type::U32,
            Number::U64(_) => Type::U64,
            Number::UXX(s) => Type::UXX(s.len() as u64),

            Number::I8(_)  => Type::I8,
            Number::I16(_) => Type::I16,
            Number::I32(_) => Type::I32,
            Number::I64(_) => Type::I64,
            Number::IXX(s) => Type::IXX(s.len() as u64),

            Number::F8(_)  => Type::F8,
            Number::F16(_) => Type::F16,
            Number::F32(_) => Type::F32,
            Number::F64(_) => Type::F64,
            Number::FXX(_, e, m) => Type::FXX(*e, *m),
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
            Number::FXX(n, e, m) => vex![; e.to_ne_bytes().to_vec(), m.to_ne_bytes().to_vec(), n.clone()].hash(h),
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
            (Number::FXX(n, e, m), Number::FXX(n2, e2, m2)) => return n == n2 && e == e2 && m == m2,
            (Number::FXX(_, _, _), _) => return false,
        }
    }
}

impl Eq for Number {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Data {
    Number(Number),
    Text(String),
    Array(Vec<Data>),
    FuncRef(FuncRef),
    StructRef(StructRef),
    ComplexType(Type),
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Number(n)      => write!(f, "{n}"),
            Data::Text(s)        => write!(f, "\"{s}\""),
            Data::Array(a)       => {
                write!(f, "[")?;
                for i in 0..a.len() {
                    write!(f, "{}", a[i])?;
                    if i < a.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Data::FuncRef(r)     => write!(f, "{r}"),
            Data::StructRef(r)   => write!(f, "{r}"),
            Data::ComplexType(t) => write!(f, "{t}"),
        }
    }
}

impl Data {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            Data::Number(number) => {
                bytes.push(0x00);
                bytes.append(&mut number.get_type().to_bytes(wrapper));
                bytes.append(&mut number.to_bytes());
            }
            Data::Text(name) => {
                bytes.push(0x01);
                bytes.append(&mut WrapperCore::num_to_bytes(name.len()));
                bytes.append(&mut name.as_bytes().to_vec());
            }
            // TODO MUST FIX BEFORE PARSER: figure out how to make string constants less dumb
            // probably just make all simple values (numbers lol) stored directly instead of by reference
            Data::Array(values) => {
                bytes.push(0x02);
                bytes.append(&mut WrapperCore::num_to_bytes(values.len()));
                for val in values {
                    bytes.append(&mut val.to_bytes(wrapper));
                }
            }
            Data::FuncRef(funcref) => {
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

    pub fn get_type(&self) -> Type {
        match self {
            Data::Number(n)      => n.get_type(),
            Data::Text(_)        => Type::Name,
            Data::Array(d)       => Type::Pointer(Box::new(d[0].get_type())),
            Data::FuncRef(_)     => Type::FuncRef,
            Data::StructRef(_)   => Type::StructRef,
            Data::ComplexType(_) => Type::Type,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FuncRef {
    pub module: Vec<String>,
    pub function: Vec<String>,
    pub name: String,
    pub is_extern: bool,
}

impl Display for FuncRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_extern {
            write!(f, "extern ")?;
        }

        if self.module.len() > 0 {
            for i in 0..self.module.len() {
                write!(f, "{}", self.module[i])?;
                write!(f, ".")?;
            }
        }

        if self.function.len() > 0 {
            for i in 0..self.function.len() {
                write!(f, "{}", self.function[i])?;
                write!(f, ".")?;
            }
        }

        write!(f, "{}()", self.name)
    }
}

impl FuncRef {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut WrapperCore::num_to_bytes(self.module.len()));
        for name in &self.module {
            let val = &mut wrapper.add_data(Data::Text(name.clone()));
            bytes.append(val);
        }
        
        bytes.append(&mut WrapperCore::num_to_bytes(self.function.len()));
        for name in &self.function {
            bytes.append(&mut wrapper.add_data(Data::Text(name.clone())));
        }

        bytes.append(&mut wrapper.add_data(Data::Text(self.name.clone())));
        bytes.push(self.is_extern as u8);

        return bytes;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StructRef {
    pub module: Vec<String>,
    pub function: Vec<String>,
    pub name: String,
}

impl Display for StructRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.module.len() > 0 {
            for i in 0..self.module.len() {
                write!(f, "{}", self.module[i])?;
                write!(f, ".")?;
            }
        }

        if self.function.len() > 0 {
            for i in 0..self.function.len() {
                write!(f, "{}", self.function[i])?;
                write!(f, ".")?;
            }
        }

        write!(f, "{}()", self.name)
    }
}

impl StructRef {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut WrapperCore::num_to_bytes(self.module.len()));
        for name in &self.module {
            bytes.append(&mut wrapper.add_data(Data::Text(name.clone())));
        }
        
        bytes.append(&mut WrapperCore::num_to_bytes(self.function.len()));
        for name in &self.function {
            bytes.append(&mut wrapper.add_data(Data::Text(name.clone())));
        }

        bytes.append(&mut wrapper.add_data(Data::Text(self.name.clone())));

        return bytes;
    }
}
