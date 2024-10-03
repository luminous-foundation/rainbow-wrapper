use rainbow_wrapper::{add, rainbow_wrapper::types::{Type, Types}, var};

// testing!
pub fn main() {
    let mut res: Vec<u8> = Vec::new();

    res.append(&mut var!(Types::TYPE(Type::U8), Types::NAME("sum".to_string())));
    res.append(&mut add!(Types::UNSIGNED(1), Types::UNSIGNED(2), Types::IDENT("sum".to_string())));

    println!("{res:?}");
}
