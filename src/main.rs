use rainbow_wrapper::{add, chunks::Type, instructions::Instruction, je, jge, number, Data, Wrapper};

pub fn main() {
    let mut test = Wrapper::new();

    // fun fact: this whole thing is basically untested, i have no idea if the bytes it outputs are correct
    //           i literally just see a bunch of numbers and assume that it looks good
    test.add_file_import("io.rbb".to_string(), "io".to_string());

    test.module_begin("test".to_string());

        test.add_instruction(add!((Type::U8, 2); (Type::U8, 3); "first"));
        test.add_instruction(add!("first", (Type::U8, 4); "second"));
        test.add_instruction(add!(pop, (Type::U8, 5); "third"));

        test.code_begin();

            test.add_instruction(je!(pop, pop));
            
        test.code_end();

        test.add_instruction(jge!(pop, "fakeVar"));

        test.struct_start("testStruct".to_string());

            test.add_var(Type::U8, "a".to_string(), Some(Data::Number(number!(u8 32))));

        test.struct_end();

        test.function_start("foo".to_string(), Type::Void, vec![]);

            test.add_metadata("this is a function".to_string());
            
        test.function_end();

        test.function_start("typeCastTest".to_string(), Type::F32, vec![(Type::I32, "val".to_string())]);
        
        let func = test.function_end();
        test.add_type_cast(Type::I32, Type::F32, func);

    test.module_end();

    let bytes = test.emit();
    println!("{:?}", bytes);
    println!("generated {} bytes", bytes.len());
    fox::disk::write_bytes("test.rbb", &bytes).unwrap();
}
