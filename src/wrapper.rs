use sha2::{Sha256, Digest};
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
    /// Merges everything.
    /// 
    /// Does not modify the other wrapper.
    pub fn merge(&mut self, other: &Wrapper) {
        self.bytes.append(&mut other.bytes.clone());
        self.strings.extend(other.strings.clone());
        self.imports.extend(other.imports.clone());
        self.externs.extend(other.externs.clone());
    }
    
    /// Merge another wrapper's data into this wrapper.
    /// (strings, imports, externs)
    /// 
    /// This is used if you have another wrapper that has data that you need in your main wrapper.
    /// 
    /// Does not modify the other wrapper.
    pub fn merge_data(&mut self, other: &Wrapper) {
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
                if str.len() < 44 {
                    res.append(&mut to_bytecode_string(&(" ".to_string() + str)));
                } else {
                    res.append(&mut to_bytecode_string(&(" ".to_string() + &hash_to_base64(str))));
                }
                res.append(&mut to_types(&vec![Type::POINTER, Type::U8]));
                res.append(&mut to_immediate(&Value::UNSIGNED(str.len() as u64)));
                res.append(&mut str.bytes().collect());
            }
        }

        return res;
    }

    pub fn get_string_name(str: &String) -> String {
        if str.len() < 44 {
            return " ".to_string() + str;
        } else {
            return " ".to_string() + &hash_to_base64(str);
        }
    }
}

fn base64_encode(data: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();

    for chunk in data.chunks(3) {
        let mut buffer = [0u8; 3];
        buffer[..chunk.len()].copy_from_slice(chunk);

        let b1 = buffer[0] >> 2;
        let b2 = ((buffer[0] & 0b00000011) << 4) | (buffer[1] >> 4);
        let b3 = ((buffer[1] & 0b00001111) << 2) | (buffer[2] >> 6);
        let b4 = buffer[2] & 0b00111111;

        result.push(BASE64_CHARS[b1 as usize]);
        result.push(BASE64_CHARS[b2 as usize]);

        if chunk.len() > 1 {
            result.push(BASE64_CHARS[b3 as usize]);
        } else {
            result.push(b'=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_CHARS[b4 as usize]);
        } else {
            result.push(b'=');
        }
    }

    String::from_utf8(result).unwrap()
}

fn hash_to_base64(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    base64_encode(&result)
}