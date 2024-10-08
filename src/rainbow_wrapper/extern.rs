use super::types::Type;

#[derive(Hash, PartialEq, Eq)]
pub struct Extern {
    pub ret_type: Vec<Type>, 
    pub name: String,
    pub arg_types: Vec<Vec<Type>>, 
    pub file: String
}