use core::fmt;
use std::fmt::Formatter;

/// Rainbow types.
/// 
/// Used with Value::TYPE
#[derive(Debug)]
pub enum Type {
    VOID,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F16,
    F32,
    F64,
    POINTER,
    TYPE,
    STRUCT,
    NAME,
}

/// Rainbow wrapper types.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Value {
    /// An identifier.
    /// 
    /// Used to identify variables.
    /// 
    /// Use `ident!` to create this value.
    IDENT(String),

    /// A name of something.
    NAME(String),
    
    /// A variable identifier.
    /// 
    /// This variable holds the name of another variable to be used.
    /// 
    /// Used for certain cases of the `mov` instruction, where you need to access values in a dynamically named variable.
    DYNAMIC_IDENT(String),

    /// A type.
    TYPE(Vec<Type>),
    
    /// A signed number.
    /// 
    /// Used for numbers that are never negative.
    /// 
    /// Use `immediate!` to create this value.
    SIGNED(i64),

    /// An unsigned number.
    /// 
    /// Used for numbers that can be negative.
    /// 
    /// Use `immediate!` to create this value.
    UNSIGNED(u64),

    /// A decimal number.
    /// 
    /// Used for numbers that have a decimal.
    /// 
    /// Use `immediate!` to create this value.
    DECIMAL(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::IDENT(s) => f.write_str(s),
            Value::NAME(v) => f.write_str(&("`".to_string() + v + "`")),
            Value::DYNAMIC_IDENT(v) => f.write_str(&("[".to_string() + v + "]")),
            Value::TYPE(v) => {
                let mut str: String = String::new();

                for t in v {
                    str += &format!("{}", t);
                }

                f.write_str(&("(".to_string() + str.as_str() + ")"))
            },
            Value::SIGNED(v) => f.write_str(&v.to_string()),
            Value::UNSIGNED(v) => f.write_str(&v.to_string()),
            Value::DECIMAL(v) => f.write_str(&v.to_string()),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::VOID => f.write_str("void"),
            Type::I8 => f.write_str("i8"),
            Type::I16 => f.write_str("i16"),
            Type::I32 => f.write_str("i32"),
            Type::I64 => f.write_str("i64"),
            Type::U8 => f.write_str("u8"),
            Type::U16 => f.write_str("u16"),
            Type::U32 => f.write_str("u32"),
            Type::U64 => f.write_str("u64"),
            Type::F16 => f.write_str("f16"),
            Type::F32 => f.write_str("f32"),
            Type::F64 => f.write_str("f64"),
            Type::POINTER => f.write_str("pointer"),
            Type::TYPE => f.write_str("type"),
            Type::STRUCT => f.write_str("struct"),
            Type::NAME => f.write_str("name"),
        }
    }
}