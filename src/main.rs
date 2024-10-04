use rainbow_wrapper::{add, rainbow_wrapper::{types::{Type, Value}, wrapper::Wrapper}, var};

// testing!
pub fn main() {
    let mut res: Wrapper = Wrapper::new();

    res.push_string(&"test".to_string());
    res.push_string(&"test".to_string());
    res.push_string(&"test2".to_string());

    println!("{:?}", res.emit());
}
