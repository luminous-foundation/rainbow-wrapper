use rainbow_wrapper::{add, rainbow_wrapper::types::Types};

// testing!
pub fn main() {
    println!("{:?}", add!(Types::SIGNED(22), Types::SIGNED(22), Types::IDENT("out".to_string())));
}
