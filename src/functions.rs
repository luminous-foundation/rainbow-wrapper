use super::{conversions::{to_bytecode_string, to_types}, types::Type};

/// Argument struct.
/// 
/// `name`: name of the argument
/// 
/// `typ`: type of the argument
#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub typ: Vec<Type>
}

/// Returns the bytes for a function.
/// 
/// `name`: name of the function
/// 
/// `args`: arguments of the function
/// 
/// `return_type`: return type of the function
/// 
/// `body`: body of the function, in bytecode
pub fn generate_function(name: &String, args: &Vec<Arg>, return_type: &Vec<Type>, body: &Vec<u8>) -> Vec<u8> {
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

/// Returns the bytes for a scope with the body `body`.
/// 
/// `body`: body of the scope, in bytecode
pub fn generate_scope(body: &Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(0xFE);

    res.append(&mut body.clone());

    res.push(0xFD);

    return res;
}