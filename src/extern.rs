use super::types::Type;

/// An extern, used to communicate with compiled code from a dynamically linked library.
/// 
/// `ret_type`: the return type of the external function
/// 
/// `name`: the name of the external function (case sensitive!)
/// 
/// `arg_types`: the types of the arguments of the external function
/// 
/// `file`: the file that contains the external function
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Extern {
    pub ret_type: Vec<Type>, 
    pub name: String,
    pub access_name: String,
    pub arg_types: Vec<Vec<Type>>, 
    pub file: String
}