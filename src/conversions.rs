use super::types::{Type, Value};

/// Convert a string into a bytecode string.
pub fn to_bytecode_string(text: &String) -> Vec<u8> {
    if text.len() >= 256 {
        panic!("string \"{text}\" provided to to_bytecode_string is too long (length {} > 255)", text.len());
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
        Value::NAME(v) => {
            res.push(0x0F);
            res.append(&mut to_bytecode_string(v));
        }
        _ => panic!("Invalid type {value:?} given to `to_immediate`")
    }

    return res;
}

/// Convert a wrapped type into bytes.
pub fn to_type(typ: &Type) -> Vec<u8> {
    match typ {
        Type::VOID => vec![0x00],
        Type::I8 => vec![0x01],
        Type::I16 => vec![0x02],
        Type::I32 => vec![0x03],
        Type::I64 => vec![0x04],
        Type::U8 => vec![0x05],
        Type::U16 => vec![0x06],
        Type::U32 => vec![0x07],
        Type::U64 => vec![0x08],
        Type::F16 => vec![0x09],
        Type::F32 => vec![0x0A],
        Type::F64 => vec![0x0B],
        Type::POINTER => vec![0x0C],
        Type::TYPE => vec![0x0D],
        Type::STRUCT(typ) => {
            let mut res = vec![0x0E];
            res.append(&mut to_bytecode_string(typ));
            res
        }
        Type::NAME   => vec![0x0F],
    }
}

/// Converts a full type into bytes.
pub fn to_types(typ: &Vec<Type>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for t in typ {
        res.append(&mut to_type(t));
    }

    return res;
}
