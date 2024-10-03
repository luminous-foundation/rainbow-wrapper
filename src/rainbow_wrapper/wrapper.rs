use std::collections::HashSet;

use super::{conversions::{to_bytecode_string, to_immediate, to_types}, types::{Type, Types}};

pub struct Wrapper {
    pub bytes: Vec<u8>,
    pub strings: HashSet<String>,
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper { bytes: Vec::new(), strings: HashSet::new() }
    }

    pub fn push_bytes(&mut self, mut bytes: Vec<u8>) {
        self.bytes.append(&mut bytes);
    }

    pub fn push_string(&mut self, string: &String) {
        self.strings.insert(string.to_string());
    }

    pub fn emit(&mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        res.append(&mut self.bytes);

        res.push(0xFC);
        for str in self.strings.iter() {
            res.append(&mut to_bytecode_string(str));
            res.append(&mut to_types(&vec![Type::POINTER, Type::U8]));
            res.append(&mut to_immediate(&Types::UNSIGNED(str.len() as u64)));
            res.append(&mut to_bytecode_string(str));
        }

        return res;
    }
}