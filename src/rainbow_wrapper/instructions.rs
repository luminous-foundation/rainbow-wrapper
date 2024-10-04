// No, this code was not generated.

#[macro_export]
/// Does nothing.
macro_rules! nop {
    () => {
        vec![0x00]
    }
}

// stack instructions
#[macro_export]
/// Pushes a value onto the stack.
/// 
/// arguments: `immediate/identifier`
macro_rules! push {
    ($value:expr) => {
         match $value {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x01);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));

                res
            }
            (Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x02);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments passed to `push` instruction")
        }
    }
}

/// Pops the top value off the stack and puts it into the given variable.
/// 
/// arguments: `identifier`
#[macro_export]
macro_rules! pop {
    ($out:expr) => {
        match $out {
            (Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x03);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments passed to the `pop` instruction")
        }
    }
}

#[macro_export]
/// Copies a value from a point in the stack into a variable.
/// 
/// arguments: `immediate/identifier`, `identifier`
macro_rules! peek {
    ($value:expr, $out:expr) => {
         match ($value, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x04);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(name), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x05);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `peek` instruction")
        }
    }
}


// call instruction
/// Calls the given function.
/// If a variable is passed in, it calls the function that the variable is referencing.
/// 
/// arguments: `function/identifier`
#[macro_export]
macro_rules! call {
    ($func:expr) => {
        match $func {
            (Value::NAME(func)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x06);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&func));

                res
            }
            (Value::IDENT(func)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x07);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&func));

                res
            }
            _ => panic!("invalid arguments passed to `call` instruction"),
        }
    }
}


// math instructions
#[macro_export]
/// Takes values `a` and `b`, and puts the sum in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! add {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                // i do not know how to make the fully qualified names shorter
                // pls make a PR if you do
                res.push(0x08);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x09);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `add` instruction"),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, and puts the difference in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! sub {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `sub` instruction"),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, and puts the product in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! mul {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x10);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x11);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x12);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x13);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `mul` instruction"),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, and puts the quotient in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! div {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x14);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x15);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x16);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x17);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `div` instruction"),
        }
    }
}


// jump instructions
#[macro_export]
/// Jumps to the given instruction (by index) in the current scope
/// 
/// arguments: `immediate/identifier`
macro_rules! jmp {
    ($index:expr) => {
        match $index {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x18);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x19);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments passed to `jmp` instruction")
        }
    }
}

#[macro_export]
/// Jumps to the given instruction (by index) in the current scope if the two values are not equal.
/// 
/// The values to be compared are the first two arguments, the last one is the index to jump to.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `immediate/identifier`
macro_rules! jne {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x20);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x21);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments passed to `jne` instruction")
        }
    }
}

#[macro_export]
/// Jumps to the given instruction (by index) in the current scope if the two values are equal.
/// 
/// The values to be compared are the first two arguments, the last one is the index to jump to.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `immediate/identifier`
macro_rules! je {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x22);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x23);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x24);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x25);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x26);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x27);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x28);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x29);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments passed to `je` instruction")
        }
    }
}

#[macro_export]
/// Jumps to the given instruction (by index) in the current scope if `a` is greater than or equal to `b`.
/// 
/// The values to be compared are the first two arguments, the last one is the index to jump to.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `immediate/identifier`
macro_rules! jge {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x30);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x31);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments passed to `jge` instruction")
        }
    }
}

#[macro_export]
/// Jumps to the given instruction (by index) in the current scope if `a` is greater than `b`.
/// 
/// The values to be compared are the first two arguments, the last one is the index to jump to.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `immediate/identifier`
macro_rules! jg {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x32);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x33);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x34);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x35);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x36);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x37);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x38);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x39);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments passed to `jg` instruction")
        }
    }
}

#[macro_export]
/// Jumps to the given instruction (by index) in the current scope if `a` is less than or equal to `b`.
/// 
/// The values to be compared are the first two arguments, the last one is the index to jump to.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `immediate/identifier`
macro_rules! jle {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x40);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x41);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments passed to `jle` instruction")
        }
    }
}

#[macro_export]
/// Jumps to the given instruction (by index) in the current scope if `a` is less than `b`.
/// 
/// The values to be compared are the first two arguments, the last one is the index to jump to.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `immediate/identifier`
macro_rules! jl {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x42);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x43);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x44);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x45);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x46);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x47);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x48);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x49);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments passed to `jl` instruction")
        }
    }
}


// move instruction
#[macro_export]
/// Moves value `a` into variable `b`.
/// 
/// If a `VARIDENT` is provided, it will use the variable that the `VARIDENT` points to.
/// 
/// arguments: `immediate/identifier/variable_identifier`, `identifier/variable_identifier`
macro_rules! mov {
    ($value:expr, $var:expr) => {
        match ($value, $var) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(value), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::VARIDENT(value), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::DYNAMIC_IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(value), Value::DYNAMIC_IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::DYNAMIC_IDENT(value), Value::DYNAMIC_IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("incorrect arguments passed to `mov` instruction")
        }
    }
}


// bitwise instructions
#[macro_export]
/// Takes values `a` and `b`, ANDs them, and puts the result in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! and {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x50);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x51);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x52);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x53);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `and` instruction"),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, ORs them, and puts the result in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! or {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x54);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x55);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x56);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x57);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `or` instruction"),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, XORs them, and puts the result in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! xor {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x58);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x59);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `xor` instruction"),
        }
    }
}

#[macro_export]
/// Takes value `a`, NOTs it, and puts the result in `b`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! not {
    ($value:expr, $out:expr) => {
        match $value, $out {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(value), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `not` instruction"),
        }
    }
}

#[macro_export]
/// Performs `a` << `b`, and puts the result in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! lsh {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x60);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x61);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `lsh` instruction"),
        }
    }
}

#[macro_export]
/// Performs `a` >> `b`, and puts the result in `c`.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! rsh {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x62);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x63);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x64);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x65);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `rsh` instruction"),
        }
    }
}


// variable instruction
#[macro_export]
/// Defines a variable with type `a` and name `b`
/// 
/// If an identifier is passed in for either argument it will use the value stored in that variable.
/// 
/// arguments: `type/identifier`, `name/identifier`
macro_rules! var {
    ($type:expr, $name:expr) => {
        match ($type, $name) {
            (Value::TYPE(typ), Value::NAME(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x66);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(typ), Value::NAME(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x67);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::TYPE(typ), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x68);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(typ), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x69);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("incorrect arguments passed to `var` instruction")
        }
    }
}


// return function
#[macro_export]
/// Returns from a function.
/// 
/// If a value is given, it returns that value.
/// 
/// arguments: [immediate/identifier] (optional)
macro_rules! ret {
    () => {
        vec![0x6A]
    };
    ($value:expr) => {
        match $value {
            Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));

                res
            }
            Value::IDENT(name) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("incorrect arguments passed to `ret` instruction")
        }
    }
}


// pointer instructions
#[macro_export]
/// Dereferences the given pointer `a` and clones the value into `b`
/// 
/// arguments: `immediate/identifier`, `identifier`
macro_rules! deref {
    ($pointer:expr, $out:expr) => {
        match ($pointer, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("incorrect arguments passed to `deref` instruction")
        }
    };
}

#[macro_export]
/// Creates a reference to `a` and stores it in `b`
/// 
/// arguments: `immediate/identifier`, `identifier`
macro_rules! r#ref {
    ($pointer:expr, $out:expr) => {
        match ($pointer, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x70);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("incorrect arguments passed to `ref` instruction")
        }
    };
}


// instantiate instruction
#[macro_export]
/// Instantiates the given struct and stores it in `b`
/// 
/// arguments: `name/identifier`, `identifier`
macro_rules! inst {
    ($pointer:expr, $out:expr) => {
        match ($pointer, $out) {
            (Value::NAME(r#struct), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x71);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&r#struct));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(r#struct), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x72);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&r#struct));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("incorrect arguments passed to `inst` instruction")
        }
    };
}


// modulo instruction
#[macro_export]
/// Performs `a` mod `b` and stores the result in `c`
/// 
/// arguments: `immediate/identifier`, `immediate/identifier`, `identifier`
macro_rules! r#mod {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x73);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x74);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x75);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x76);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `mod` instruction")
        }
    }
}


// more pointer instructions
#[macro_export]
/// Moves `a` into `b` with offset `c`
/// 
/// `b[c] = a`
/// 
/// arguments: `immediate/identifier`, `identifier`, `immediate/identifier`
macro_rules! pmov {
    ($value:expr, $pointer:expr, $offset:expr) => {
        match ($value, $pointer, $offset) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(pointer), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x77);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&$offset));

                res
            }
            (Value::IDENT(value), Value::IDENT(pointer), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x78);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$offset));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(pointer), Value::IDENT(offset)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x79);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&offset));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(pointer), Value::IDENT(offset)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&offset));

                res
            }
            _ => panic!("invalid arguments passed to `pmov` instruction")
        }
    }
}

#[macro_export]
/// Allocates a pointer with type `a`, size `b`, and puts the address in `c`
/// 
/// arguments: `type/identifier`, `immediate/identifier`, `identifier`
macro_rules! alloc {
    ($typ:expr, $size:expr, $var:expr) => {
        match ($typ, $size, $var) {
            (Value::TYPE(typ), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$size));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(typ), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7C);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$size));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::TYPE(typ), Value::IDENT(size), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7D);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&size));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(typ), Value::IDENT(size), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7E);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&size));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments passed to `alloc` instruction")
        }
    };
}

#[macro_export]
/// Frees a the pointer `a`.
/// 
/// If a size is given, it frees that many bytes.
/// 
/// If no size is given, it expects an identifier, and it frees the whole pointer.
/// 
/// arguments: `immediate/identifier`, `immediate/identifier` (optional)
macro_rules! free {
    ($pointer:expr) => {
        match $pointer {
            Value::IDENT(pointer) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7F);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));

                res
            }
            _ => panic!("invalid arguments passed to `free` instruction")
        }
    };
    ($pointer:expr, $size:expr) => {
        match ($pointer, $size) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x80);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$size));

                res
            }
            (Value::IDENT(pointer), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x81);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$size));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(size)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x82);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&size));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(size)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x83);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&size));

                res
            }
            _ => panic!("invalid arguments passed to `free` instruction")
        }
    }
}


// callc instruction
#[macro_export]
/// Calls the function at address `a`, with return type `b` and argument count `c`.
/// 
/// The function must be a compiled binary function loaded into memory.
/// 
/// arguments: `immediate/identifier`, `type/identifier`, `immediate/identifier`
macro_rules! callc {
    ($addr:expr, $ret:expr, $args:expr) => {
        match ($addr, $ret, $args) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::TYPE(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x84);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::IDENT(addr), Value::TYPE(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x85);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x86);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::IDENT(addr), Value::IDENT(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x87);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::SIGNED(_) | ::UNSIGNED(_) | Value::DECIMAL(_), Value::TYPE(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x88);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            (Value::IDENT(addr), Value::TYPE(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x89);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            (Value::IDENT(addr), Value::IDENT(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            _ => panic!("invalid arguments passed to `callc` instruction")
        }
    };
}