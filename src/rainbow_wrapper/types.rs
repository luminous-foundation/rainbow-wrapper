use core::fmt;
use std::fmt::Formatter;

pub enum Types {
    SIGNED(i64),
    UNSIGNED(u64),
    DECIMAL(f64),
    IDENT(String),
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::SIGNED(v) => f.write_str(&v.to_string()),
            Types::UNSIGNED(v) => f.write_str(&v.to_string()),
            Types::DECIMAL(v) => f.write_str(&v.to_string()),
            Types::IDENT(v) => f.write_str(v),
        }
    }
}