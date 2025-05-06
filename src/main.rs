use rainbow_wrapper::{add, instructions::Instruction, je, jge, Wrapper};

pub fn main() {
    let mut test = Wrapper::new();

    test.module_begin("test".to_string());

        test.add_instruction(add!(u8 2, u8 3, "first"));
        test.add_instruction(add!("first", u8 4, "second"));
        test.add_instruction(add!(pop, u8 5, "third"));

        test.code_begin();

            test.add_instruction(je!(pop, pop));
            
        test.code_end();

        test.add_instruction(jge!(pop, "fakeVar"));

        test.function_start("foo".to_string(), vec![]);

        test.function_end();
        
    test.module_end();
    
    test.type_cast_begin();
    test.type_cast_end();

    let bytes = test.emit();
    println!("{:?}", bytes);
}
