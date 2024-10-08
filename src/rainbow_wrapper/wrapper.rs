use std::collections::HashSet;

use super::{conversions::{to_bytecode_string, to_immediate, to_types}, r#extern::Extern, types::{Type, Value}};

pub struct Wrapper {
    pub bytes: Vec<u8>,
    pub strings: HashSet<String>,
    pub imports: HashSet<String>,
    pub externs: HashSet<Extern>
}

impl Wrapper {
    /// Creates a new wrapper.
    pub fn new() -> Wrapper {
        Wrapper { bytes: Vec::new(), strings: HashSet::new(), imports: HashSet::new(), externs: HashSet::new() }
    }

    /// Push an array of bytes to the wrapper.
    /// 
    /// These will be emitted as code when you run the `emit` function.
    pub fn push(&mut self, mut bytes: Vec<u8>) {
        self.bytes.append(&mut bytes);
    }

    /// Push a string to the wrapper.
    /// 
    /// Strings are emitted as part of the Rainbow "data section" to be used.
    /// Any UTF-8 string can be used.
    pub fn push_string(&mut self, string: &String) {
        self.strings.insert(string.to_string());
    }

    /// Push an import to the wrapper.
    /// 
    /// Imports are emitted as imports to the top of the file.
    /// 
    /// Imports must also be `.rbb` files.
    pub fn push_import(&mut self, import: &String) {
        self.imports.insert(import.to_string());
    }

    /// Push an extern to the wrapper.
    /// 
    /// Externs are emitted below imports.
    /// 
    /// Externs are used to communicate with compiled code from a dynamically linked library.
    pub fn push_extern(&mut self, r#extern: Extern) {
        self.externs.insert(r#extern);
    }

    /// Merge this wrapper with another wrapper.
    /// 
    /// This is used if you have two wrappers you need to combine.
    /// 
    /// Does not modify the other wrapper.
    pub fn merge(&mut self, other: &Wrapper) {
        self.bytes.append(&mut other.bytes.clone());
        self.strings.extend(other.strings.clone());
        self.imports.extend(other.imports.clone());
        self.externs.extend(other.externs.clone());
    }

    /// Emits the final bytes of the program, ready to be executed.
    /// 
    /// *Only* run when the program is done being added to the wrapper.
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