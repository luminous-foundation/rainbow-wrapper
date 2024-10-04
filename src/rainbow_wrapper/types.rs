use core::fmt;
use std::fmt::Formatter;

/// Rainbow types.
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
pub enum Value {
    /// A signed number.
    SIGNED(i64),

    /// An unsigned number.
    UNSIGNED(u64),

    /// A decimal number.
    DECIMAL(f64),

    /// An identifier
    /// 
    /// Used to identify variables.
    IDENT(String),

    /// A name of something.
    NAME(String),
    
    /// A variable identifier.
    /// 
    /// This variable holds the name of another variable to be used.
    DYNAMIC_IDENT(String),

    /// A type.
    TYPE(Vec<Type>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::SIGNED(v) => f.write_str(&v.to_string()),
            Value::UNSIGNED(v) => f.write_str(&v.to_string()),
            Value::DECIMAL(v) => f.write_str(&v.to_string()),
            Value::IDENT(v) => f.write_str(v),
            Value::NAME(v) => f.write_str(&("`".to_string() + v + "`")),
            Value::DYNAMIC_IDENT(v) => f.write_str(&("[".to_string() + v + "]")),
            Value::TYPE(v) => {
                let mut str: String = String::new();

                for t in v {
                    str += &format!("{}", t);
                }

                f.write_str(&("(".to_string() + str.as_str() + ")"))
            }
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