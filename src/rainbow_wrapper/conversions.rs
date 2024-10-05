use super::types::{Type, Value};

/// Convert a string into a bytecode string.
pub fn to_bytecode_string(text: &String) -> Vec<u8> {
    if text.len() >= 256 {
        panic!("string {text} provided to to_bytecode_string is too long (length {} > 255)", text.len());
    }

    let mut res: Vec<u8> = Vec::new();

    res.push(text.len() as u8);
    for c in text.bytes() {
        res.push(c);
    }

    return res;
}

// TODO: compress types if possible, we want to use the smallest bit-width at some point to make programs smaller
/// Convert a wrapped value into bytes.
pub fn to_immediate(value: &Value) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    match value {
        Value::SIGNED(v) => {
            res.push(0x04);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        Value::UNSIGNED(v) => {
            res.push(0x08);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        Value::DECIMAL(v) => {
            res.push(0x0B);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        _ => panic!("Invalid type {value:?} given to `to_immediate`")
    }

    return res;
}

/// Convert a wrapped type into bytes.
pub fn to_type(typ: &Type) -> u8 {
    match typ {
        Type::VOID => 0x00,
        Type::I8 => 0x01,
        Type::I16 => 0x02,
        Type::I32 => 0x03,
        Type::I64 => 0x04,
        Type::U8 => 0x05,
        Type::U16 => 0x06,
        Type::U32 => 0x07,
        Type::U64 => 0x08,
        Type::F16 => 0x09,
        Type::F32 => 0x0A,
        Type::F64 => 0x0B,
        Type::POINTER => 0x0C,
        Type::TYPE => 0x0D,
        Type::STRUCT => 0x0E,
        Type::NAME => 0x0F,
    }
}

/// Converts a vector of types into bytes.
pub fn to_types(typ: &Vec<Type>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for t in typ {
        res.push(to_type(t));
    }

    return res;
}