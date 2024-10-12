use rainbow_wrapper::wrapper::Wrapper;

use rainbow_wrapper::*;

// testing!
pub fn main() {
    let mut res: Wrapper = Wrapper::new();

    res.push(nop!());
    
    res.push(push!(immediate!(SIGNED(32))));
    res.push(pop!(ident!("out")));
    res.push(peek!(immediate!(UNSIGNED(3)), ident!("out2")))    ;
    
    res.push(call!(name!("myFunc")));
    
    res.push(add!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3")));
    res.push(sub!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3")));
    res.push(mul!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3")));
    res.push(div!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out3")));

    res.push(jmp!(ident!("addr")));
    res.push(jne!(immediate!(DECIMAL(12.3)), ident!("secondNum"), ident!("addr")));
    res.push(je!(immediate!(DECIMAL(12.3)), ident!("secondNum"), ident!("addr")));
    res.push(jge!(immediate!(DECIMAL(12.3)), ident!("secondNum"), ident!("addr")));
    res.push(jg!(immediate!(DECIMAL(12.3)), ident!("secondNum"), ident!("addr")));
    res.push(jle!(immediate!(DECIMAL(12.3)), ident!("secondNum"), ident!("addr")));
    res.push(jl!(immediate!(DECIMAL(12.3)), ident!("secondNum"), ident!("addr")));

    res.push(mov!(immediate!(UNSIGNED(32)), ident!("out4")));

    res.push(and!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5")));
    res.push(or!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5")));
    res.push(xor!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5")));
    res.push(not!(immediate!(UNSIGNED(0)), ident!("out5")));
    res.push(lsh!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5")));
    res.push(rsh!(immediate!(SIGNED(34)), immediate!(SIGNED(35)), ident!("out5")));

    res.push(var!(rbtype!(POINTER, U8), name!("variable")));

    res.push(ret!());
                       // decimal as a pointer :)
    res.push(deref!(immediate!(DECIMAL(3.3)), ident!("out6")));
    res.push(r#ref!(ident!("out6"), ident!("out7")));

    res.push(inst!(name!("Foo"), ident!("out8")));

    res.push(r#mod!(ident!(" a"), ident!(" . "), ident!("out9")));

    res.push(pmov!(ident!("out4"), ident!("out5"), ident!("out10")));
    res.push(alloc!(rbtype!(POINTER, VOID), ident!("i dont wanna type anymore"), ident!("out11")));
    res.push(free!(immediate!(SIGNED(-1)), ident!("out11")));

    res.push(callc!(immediate!(UNSIGNED(0x867456AA)), rbtype!(VOID), immediate!(SIGNED(2))));

    res.push(cmp!(cond!(==), ident!("a"), ident!("b"), ident!("temp")));

    res.push(
        if_block!("PLATFORM", "==", "PLATFORM_WIN32", ret!(immediate!(SIGNED(34))))
    );
    res.push(
        elseif_block!("PLATFORM", "==", "PLATFORM_LINUX", ret!(immediate!(SIGNED(35))))
    );
    res.push(
        else_block!(ret!(immediate!(SIGNED(255))))
    );
    res.push(
        end_block!()
    );

    println!("{:?}", res.emit());
}
