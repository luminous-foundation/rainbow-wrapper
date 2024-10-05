use ::rainbow_wrapper::{rainbow_wrapper::{types::{Type, Value}, wrapper::Wrapper}};

use ::rainbow_wrapper::*;

// testing!
pub fn main() {
    let mut res: Wrapper = Wrapper::new();

    res.push(nop!());
    
    res.push(push!(immediate!(SIGNED(32))));
    res.push(pop!(ident!("out".to_string())));
    res.push(peek!(immediate!(UNSIGNED(3)), ident!("out2".to_string())))    ;
    
    res.push(call!(name!("myFunc".to_string())));
    
    res.push(add!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3".to_string())));
    res.push(sub!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3".to_string())));
    res.push(mul!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3".to_string())));
    res.push(div!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3".to_string())));

    res.push(jmp!(ident!("addr".to_string())));
    res.push(jne!(immediate!(DECIMAL(12.3)), ident!("secondNum".to_string()), ident!("addr".to_string())));
    res.push(je!(immediate!(DECIMAL(12.3)), ident!("secondNum".to_string()), ident!("addr".to_string())));
    res.push(jge!(immediate!(DECIMAL(12.3)), ident!("secondNum".to_string()), ident!("addr".to_string())));
    res.push(jg!(immediate!(DECIMAL(12.3)), ident!("secondNum".to_string()), ident!("addr".to_string())));
    res.push(jle!(immediate!(DECIMAL(12.3)), ident!("secondNum".to_string()), ident!("addr".to_string())));
    res.push(jl!(immediate!(DECIMAL(12.3)), ident!("secondNum".to_string()), ident!("addr".to_string())));

    res.push(mov!(immediate!(UNSIGNED(32)), ident!("out4".to_string())));

    res.push(and!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5".to_string())));
    res.push(or!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5".to_string())));
    res.push(xor!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5".to_string())));
    res.push(not!(immediate!(UNSIGNED(0)), ident!("out5".to_string())));
    res.push(lsh!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5".to_string())));
    res.push(rsh!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5".to_string())));

    res.push(var!(rbtype!(POINTER, U8), name!("variable".to_string())));

    res.push(ret!());
                       // decimal as a pointer :)
    res.push(deref!(immediate!(DECIMAL(3.3)), ident!("out6".to_string())));
    res.push(r#ref!(ident!("out6".to_string()), ident!("out7".to_string())));

    res.push(inst!(name!("Foo".to_string()), ident!("out8".to_string())));

    res.push(r#mod!(ident!(" a".to_string()), ident!(" . ".to_string()), ident!("out9".to_string())));

    res.push(pmov!(ident!("out4".to_string()), ident!("out5".to_string()), ident!("out10".to_string())));
    res.push(alloc!(rbtype!(POINTER, VOID), ident!("i dont wanna type anymore".to_string()), ident!("out11".to_string())));
    res.push(free!(immediate!(SIGNED(-1)), ident!("out11".to_string())));

    res.push(callc!(immediate!(UNSIGNED(0x867456AA)), rbtype!(VOID), immediate!(SIGNED(2))));

    println!("{:?}", res.emit());
}
