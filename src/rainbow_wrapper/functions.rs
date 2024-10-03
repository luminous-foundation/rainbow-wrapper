use super::{conversions::{to_bytecode_string, to_types}, types::Type};

pub struct Arg {
    pub name: String,
    pub typ: Vec<Type>
}

pub fn generate_function(name: &String, args: Vec<Arg>, return_type: Vec<Type>, body: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(0xFF);

    res.append(&mut to_types(&return_type));

    res.append(&mut to_bytecode_string(name));

    for arg in args {
        res.append(&mut to_types(&arg.typ));
        res.append(&mut to_bytecode_string(&arg.name));
    }

    res.append(&mut generate_scope(body));

    return res;
}

pub fn generate_scope(mut body: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(0xFE);

    res.append(&mut body);

    res.push(0xFD);

    return res;
}