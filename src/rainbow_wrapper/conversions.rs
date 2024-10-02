use super::types::Types;

pub fn to_bytecode_string(text: &String) -> Vec<u8> {
    if text.len() >= 256 {
        panic!("string {text} provided to to_bytecode_string is too long (length {} > 255)", text.len());
    }

    let mut res: Vec<u8> = Vec::new();

    res.push(text.len() as u8);
    for c in text.bytes() {
        res.push(c as u8);
    }

    return res;
}

// TODO: compress types if possible, we want to use the smallest bit-width at some point to make programs smaller
pub fn to_immediate(value: Types) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    match value {
        Types::SIGNED(v) => {
            res.push(0x04);
            res.append(&mut v.to_be_bytes().to_vec());
        },
        Types::UNSIGNED(v) => todo!(),
        Types::DECIMAL(v) => todo!(),
        _ => panic!("invalid type {} passed to to_immediate", value),
    }

    return res;
}