use std::collections::HashSet;

use super::{conversions::{to_bytecode_string, to_immediate, to_types}, r#extern::Extern, types::{Type, Value}};

pub struct Wrapper {
    pub bytes: Vec<u8>,
    pub strings: HashSet<String>,
    pub imports: HashSet<String>,
    pub externs: HashSet<Extern>
}

impl Wrapper {
    pub fn new() -> Wrapper {
        Wrapper { bytes: Vec::new(), strings: HashSet::new(), imports: HashSet::new(), externs: HashSet::new() }
    }

    pub fn push(&mut self, mut bytes: Vec<u8>) {
        self.bytes.append(&mut bytes);
    }

    pub fn push_string(&mut self, string: &String) {
        self.strings.insert(string.to_string());
    }

    pub fn push_import(&mut self, import: &String) {
        self.imports.insert(import.to_string());
    }

    pub fn push_extern(&mut self, r#extern: Extern) {
        self.externs.insert(r#extern);
    }

    pub fn emit(&mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        for import in self.imports.iter() {
            res.push(0xFA);
            res.append(&mut to_bytecode_string(import));
        }

        for ext in self.externs.iter() {
            res.push(0xF9);
            res.append(&mut to_types(&ext.ret_type));
            res.append(&mut to_bytecode_string(&ext.name));
            for arg in &ext.arg_types {
                res.append(&mut to_types(&arg));
            }
            res.push(0xF8);
            res.append(&mut to_bytecode_string(&ext.file));
        }

        res.append(&mut self.bytes);

        if self.strings.len() > 0 {
            res.push(0xFC);
            for str in self.strings.iter() {
                res.append(&mut to_bytecode_string(str));
                res.append(&mut to_types(&vec![Type::POINTER, Type::U8]));
                res.append(&mut to_immediate(&Value::UNSIGNED(str.len() as u64)));
                res.append(&mut to_bytecode_string(str)[1..].to_vec());
            }
        }

        return res;
    }
}