use rainbow_wrapper::{instructions::Instruction, Wrapper};

pub fn main() {
    let mut test = Wrapper::new();
    test.code_begin();
    test.code_begin();

    test.add_instruction(Instruction::JE_S_S);

    test.code_end();

    test.add_instruction(Instruction::JGE_S_V("testing bitch".to_string()));
    test.code_end();

    println!("{:?}", test.emit())
}
