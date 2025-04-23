use rainbow_wrapper::Wrapper;

pub fn main() {
    let mut test = Wrapper::new();
    test.code_begin();

    // add instructions n shit

    test.code_end();

    println!("{:?}", test.emit())
}
