use super::types::{Type, Types};

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
pub fn to_immediate(value: &Types) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    match value {
        Types::SIGNED(v) => {
            res.push(0x04);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        Types::UNSIGNED(v) => {
            res.push(0x08);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        Types::DECIMAL(v) => {
            res.push(0x0B);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        _ => panic!("invalid type {} passed to to_immediate", value),
    }

    return res;
}

/// Convert a wrapped type into bytes.
pub fn to_type(typ: &Type) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    match typ {
        Type::VOID => res.push(0x00),
        Type::I8 => res.push(0x01),
        Type::I16 => res.push(0x02),
        Type::I32 => res.push(0x03),
        Type::I64 => res.push(0x04),
        Type::U8 => res.push(0x05),
        Type::U16 => res.push(0x06),
        Type::U32 => res.push(0x07),
        Type::U64 => res.push(0x08),
        Type::F16 => res.push(0x09),
        Type::F32 => res.push(0x0A),
        Type::F64 => res.push(0x0B),
        Type::POINTER => res.push(0x0C),
        Type::TYPE => res.push(0x0D),
        Type::STRUCT => res.push(0x0E),
        Type::NAME => res.push(0x0F),
    }

    return res;
}