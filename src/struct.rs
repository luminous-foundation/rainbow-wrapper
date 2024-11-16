use crate::Type;

/// A struct, used to hold and organize data.
/// 
/// `name`: the name of the struct
/// 
/// `types`: the types of the variables held in the struct
/// 
/// `names`: the names of the variables held in the struct
/// 
/// `types` and `names` share the same indices.
#[derive(Debug)]
pub struct Struct {
    pub name: String,

    pub types: Vec<Vec<Type>>,
    pub names: Vec<String>,
}