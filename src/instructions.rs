// No, this code was not generated.

// TODO: add a way to specify a size for immediates

#[macro_export]
/// Used to get an immediate value
/// 
/// Takes in `SIGNED(x)` | `UNSIGNED(x)` | `DECIMAL(x)`
macro_rules! immediate {
    (SIGNED($x:expr)) => {
        Value::SIGNED($x as i64)
    };
    (UNSIGNED($x:expr)) => {
        Value::UNSIGNED($x as u64)
    };
    (DECIMAL($x:expr)) => {
        Value::DECIMAL($x as f64)
    };
    (NAME($x:expr)) => {
        Value::NAME($x.to_string())
    }
}

#[macro_export]
/// Used to get an identifier.
/// 
/// Takes in a `String`
macro_rules! ident {
    ($s:expr) => {
        Value::IDENT($s.to_string())
    };
}

#[macro_export]
/// Used to get a type.
/// 
/// Takes in `VOID` | `I8` | `I16` | `I32` | `I64` | `U8` | `U16` | `U32` | `U64` | `F16` | `F32` | `F64` | `POINTER` | `TYPE` | `STRUCT` | `NAME`
/// 
/// or any combination of the above.
/// 
/// (note: the only combination that will work is any number of `POINTER` followed by any other type.)
macro_rules! rbtype {
    ($($t:tt),*) => {
        Value::TYPE(vec![
            $(
                rbtype!(@single $t)
            ),*
        ])
    };
    (@single POINTER) => {
        Type::POINTER
    };
    (@single VOID) => {
        Type::VOID
    };
    (@single I8) => {
        Type::I8
    };
    (@single I16) => {
        Type::I16
    };
    (@single I32) => {
        Type::I32
    };
    (@single I64) => {
        Type::I64
    };
    (@single U8) => {
        Type::U8
    };
    (@single U16) => {
        Type::U16
    };
    (@single U32) => {
        Type::U32
    };
    (@single U64) => {
        Type::U64
    };
    (@single F16) => {
        Type::F16
    };
    (@single F32) => {
        Type::F32
    };
    (@single F64) => {
        Type::F64
    };
    (@single TYPE) => {
        Type::TYPE
    };
    (@single STRUCT) => {
        Type::STRUCT
    };
    (@single NAME) => {
        Type::NAME
    };
}


#[macro_export]
/// Used to get a dynamic identifier.
/// 
/// Takes in a `String`
macro_rules! dynamic_ident {
    ($s:expr) => {
        Value::DYNAMIC_IDENT($s)
    };
}

#[macro_export]
/// Used to get a name.
/// 
/// Takes in a `String`
macro_rules! name {
    ($s:expr) => {
        Value::NAME($s.to_string())
    };
}

#[macro_export]
/// Used to get a condition.
/// 
/// Takes in `==` | `!=` | `>=` | `>` | `<=` | `<`
macro_rules! cond {
    (==) => {
        Value::UNSIGNED(0)
    };
    (!=) => {
        Value::UNSIGNED(1)
    };
    (>=) => {
        Value::UNSIGNED(2)
    };
    (>) => {
        Value::UNSIGNED(3)
    };
    (<=) => {
        Value::UNSIGNED(4)
    };
    (<) => {
        Value::UNSIGNED(5)
    };
}

#[macro_export]
/// Does nothing.
macro_rules! nop {
    () => {
        vec![0x00]
    }
}

// stack instructions
#[macro_export]
/// Pushes value `a` onto the stack.
/// 
/// `a`: `immediate!` | `ident!`
macro_rules! push {
    ($value:expr) => {
         match $value {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_) | Value::NAME(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x01);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));

                res
            }
            (Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x02);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}] passed to `push` instruction", $value)
        }
    }
}

/// Pops the top value off the stack and puts it into `a`.
/// 
/// `a`: `ident!`
#[macro_export]
macro_rules! pop {
    ($out:expr) => {
        match $out {
            (Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x03);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}] passed to the `pop` instruction", $out)
        }
    }
}

#[macro_export]
/// Copies a value from a index `a` in the stack `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `ident!`
macro_rules! peek {
    ($value:expr, $out:expr) => {
         match ($value, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x04);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(name), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x05);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `peek` instruction", $value, $out)
        }
    }
}


// call instruction
/// Calls `a`.
/// If a variable is passed in, it calls the function that the variable is referencing.
/// 
/// `a`: `name!` | `ident!`
#[macro_export]
macro_rules! call {
    ($func:expr) => {
        match $func {
            (Value::NAME(func)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x06);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&func));

                res
            }
            (Value::IDENT(func)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x07);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&func));

                res
            }
            _ => panic!("invalid arguments [{}] passed to `call` instruction", $func),
        }
    }
}


// math instructions
#[macro_export]
/// Takes values `a` and `b`, and puts the sum in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! add {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x08);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x09);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `add` instruction", $left, $right, $out),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, and puts the difference in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! sub {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0C);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0D);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0E);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `sub` instruction", $left, $right, $out),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, and puts the product in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! mul {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x10);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x11);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x12);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x13);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `mul` instruction", $left, $right, $out),
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, and puts the quotient in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! div {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x14);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x15);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x16);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x17);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `div` instruction", $left, $right, $out),
        }
    }
}


// jump instructions
#[macro_export]
/// Jumps to instruction `a` (by index) in the current scope
/// 
/// `a`: `immediate!` | `ident!`
macro_rules! jmp {
    ($index:expr) => {
        match $index {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x18);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x19);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}] passed to `jmp` instruction", $index)
        }
    }
}

#[macro_export]
/// Jumps to instruction `c` (by index) in the current scope if `a` != `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! jne {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1C);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1D);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1E);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x1F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x20);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x21);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `jne` instruction", $left, $right, $index)
        }
    }
}

#[macro_export]
/// Jumps to instruction `c` (by index) in the current scope if `a` == `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! je {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x22);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x23);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x24);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x25);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x26);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x27);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x28);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x29);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `je` instruction", $left, $right, $index)
        }
    }
}

#[macro_export]
/// Jumps to instruction `c` (by index) in the current scope if `a` >= `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! jge {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2C);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2D);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2E);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x2F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x30);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x31);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `jge` instruction", $left, $right, $index)
        }
    }
}

#[macro_export]
/// Jumps to instruction `c` (by index) in the current scope if `a` > `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! jg {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x32);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x33);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x34);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x35);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x36);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x37);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x38);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x39);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `jg instruction", $left, $right, $index)
        }
    }
}

#[macro_export]
/// Jumps to instruction `c` (by index) in the current scope if `a` <= `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! jle {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3C);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3D);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3E);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x3F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x40);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x41);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `jle` instruction", $left, $right, $index)
        }
    }
}

#[macro_export]
/// Jumps to instruction `c` (by index) in the current scope if `a` < `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! jl {
    ($left:expr, $right:expr, $index:expr) => {
        match ($left, $right, $index) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x42);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x43);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x44);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x45);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x46);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x47);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x48);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(index)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x49);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&index));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `jl` instruction", $left, $right, $index)
        }
    }
}


// move instruction
#[macro_export]
/// Moves value `a` into `b`.
/// 
/// `a`: `immediate!` | `ident!` | `dynamic_ident!`
/// 
/// `b`: `ident!` | `dynamic_ident!`
macro_rules! mov {
    ($value:expr, $out:expr) => {
        match ($value, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_) | Value::NAME(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(value), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::DYNAMIC_IDENT(value), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4C);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_) | Value::NAME(_), Value::DYNAMIC_IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4D);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(value), Value::DYNAMIC_IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4E);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::DYNAMIC_IDENT(value), Value::DYNAMIC_IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x4F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `mov` instruction", $value, $out)
        }
    }
}


// bitwise instructions
#[macro_export]
/// Takes values `a` and `b`, ANDs them, and puts the result in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! and {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x50);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x51);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x52);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x53);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `and` instruction", $left, $right, $out)
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, ORs them, and puts the result in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! or {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x54);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x55);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x56);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x57);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `or` instruction", $left, $right, $out)
        }
    }
}

#[macro_export]
/// Takes values `a` and `b`, XORs them, and puts the result in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! xor {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x58);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x59);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `xor` instruction", $left, $right, $out)
        }
    }
}

#[macro_export]
/// Takes value `a`, NOTs it, and puts the result in `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! not {
    ($value:expr, $out:expr) => {
        match ($value, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5C);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(value), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5D);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `and` instruction", $value, $out)
        }
    }
}

#[macro_export]
/// Performs `a` << `b`, and puts the result in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! lsh {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5E);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x5F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x60);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x61);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `lsh` instruction", $left, $right, $out)
        }
    }
}

#[macro_export]
/// Performs `a` >> `b`, and puts the result in `c`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! rsh {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x62);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x63);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x64);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x65);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `rsh` instruction", $left, $right, $out)
        }
    }
}


// variable instruction
#[macro_export]
/// Defines a variable with type `a` and name `b`
/// 
/// If an identifier is passed in for `a` it will use the type stored in that identifier, allowing for dynamic types.
/// 
/// If an identifier is passed in for `b` it will use the name stored in that identifier, allowing for dynamic names.
/// 
/// `a`: `rbtype!` | `ident!`
/// 
/// `b`: `name!` | `ident!`
macro_rules! var {
    ($type:expr, $name:expr) => {
        match ($type, $name) {
            (Value::TYPE(typ), Value::NAME(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x66);
                res.append(&mut rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(typ), Value::NAME(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x67);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::TYPE(typ), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x68);
                res.append(&mut rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(typ), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x69);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `var` instruction", $type, $name)
        }
    }
}


// return function
#[macro_export]
/// Returns from a function.
/// 
/// If a value is given, it returns that value.
/// 
/// `a` (optional): `immediate!` | `ident!`
macro_rules! ret {
    () => {
        vec![0x6A]
    };
    ($value:expr) => {
        match $value {
            Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6B);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));

                res
            }
            Value::IDENT(name) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6C);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}] passed to `ret` instruction", $value)
        }
    }
}


// pointer instructions
#[macro_export]
/// Dereferences the pointer `a` and clones the value into `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `ident!`
macro_rules! deref {
    ($pointer:expr, $out:expr) => {
        match ($pointer, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6D);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6E);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `deref` instruction", $pointer, $out)
        }
    };
}

#[macro_export]
/// Creates a reference to `a` and stores it in `b`.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `ident!`
macro_rules! r#ref { // rust :why:
    ($pointer:expr, $out:expr) => {
        match ($pointer, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x6F);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x70);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `ref` instruction", $pointer, $out)
        }
    };
}


// instantiate instruction
#[macro_export]
/// Instantiates the struct `a` and stores it in `b`.
/// 
/// `a`: `name!` | `ident!`
/// 
/// `b`: `ident!`
macro_rules! inst {
    ($struct:expr, $out:expr) => {
        match ($struct, $out) {
            (Value::NAME(r#struct), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x71);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&r#struct));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            (Value::IDENT(r#struct), Value::IDENT(name)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x72);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&r#struct));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&name));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `inst` instruction", $struct, $out)
        }
    };
}


// modulo instruction
#[macro_export]
/// Performs `a` mod `b` and stores the result in `c`
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! r#mod {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x73);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x74);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x75);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(left), Value::IDENT(right), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x76);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `mod` instruction", $left, $right, $out)
        }
    }
}


// more pointer instructions
#[macro_export]
/// Moves `a` into `b` with offset `c`
/// 
/// Equivalent to `b[c] = a`
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! pmov {
    ($value:expr, $pointer:expr, $offset:expr) => {
        match ($value, $pointer, $offset) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(pointer), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x77);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$offset));

                res
            }
            (Value::IDENT(value), Value::IDENT(pointer), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x78);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$offset));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(pointer), Value::IDENT(offset)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x79);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&offset));

                res
            }
            (Value::IDENT(value), Value::IDENT(pointer), Value::IDENT(offset)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7A);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&value));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&offset));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `pmov` instruction", $value, $pointer, $offset)
        }
    }
}

#[macro_export]
/// Allocates a pointer with type `a`, size `b`, and puts the address in `c`
/// 
/// If an identifier is passed in for `a` it will use the type stored in that identifier, allowing for dynamic types.
/// 
/// `a`: `type!` | `ident!`
/// 
/// `b`: `immediate!` | `ident!`
/// 
/// `c`: `ident!`
macro_rules! alloc {
    ($typ:expr, $size:expr, $out:expr) => {
        match ($typ, $size, $out) {
            (Value::TYPE(typ), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7B);
                res.append(&mut rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$size));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(typ), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7C);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$size));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::TYPE(typ), Value::IDENT(size), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7D);
                res.append(&mut rainbow_wrapper::conversions::to_types(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&size));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(typ), Value::IDENT(size), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7E);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&typ));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&size));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `alloc` instruction", $typ, $size, $out)
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
/// `a`: `immediate!` | `ident!`
/// 
/// `b` (optional): `immediate!` | `ident!` (optional)
macro_rules! free {
    ($pointer:expr) => {
        match $pointer {
            Value::IDENT(pointer) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x7F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));

                res
            }
            _ => panic!("invalid arguments [{}] passed to `free` instruction", $pointer)
        }
    };
    ($pointer:expr, $size:expr) => {
        match ($pointer, $size) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x80);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$size));

                res
            }
            (Value::IDENT(pointer), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x81);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$size));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(size)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x82);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&size));

                res
            }
            (Value::IDENT(pointer), Value::IDENT(size)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x83);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&pointer));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&size));

                res
            }
            _ => panic!("invalid arguments [{}, {}] passed to `free` instruction", $pointer, $size)
        }
    }
}


// callc instruction
#[macro_export]
/// Calls the function at address `a`, with return type `b` and argument count `c`.
/// 
/// The function must be loaded from a DLL, with its address resolved and executed from memory.
/// 
/// `a`: `immediate!` | `ident!`
/// 
/// `b`: `type!` | `ident!`
/// 
/// `c`: `immediate!` | `ident!`
macro_rules! callc {
    ($addr:expr, $ret:expr, $args:expr) => {
        match ($addr, $ret, $args) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::TYPE(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x84);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::IDENT(addr), Value::TYPE(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x85);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x86);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::IDENT(addr), Value::IDENT(ret), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x87);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$args));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::TYPE(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x88);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            (Value::IDENT(addr), Value::TYPE(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x89);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::conversions::to_types(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8A);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$addr));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            (Value::IDENT(addr), Value::IDENT(ret), Value::IDENT(args)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8B);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&addr));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&ret));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&args));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}] passed to `callc` instruction", $addr, $ret, $args)
        }
    };
}

#[macro_export]
/// Compares `b` and `c` with condition `a` and puts 0 or 1 in `d` depending on the result.
/// 
/// a: `immediate!` | `cond!` | `ident!`
/// 
/// b: `immediate!` | `ident!`
/// 
/// c: `immediate!` | `ident!`
/// 
/// d: `ident!`
macro_rules! cmp {
    ($cond:expr, $a:expr, $b:expr, $out:expr) => {
        match ($cond, $a, $b, $out) {
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8C);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$cond));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$a));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(cond), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8D);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&cond));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$a));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(a), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8E);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$cond));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&a));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(cond), Value::IDENT(a), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x8F);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&cond));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&a));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(b), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x90);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$cond));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$a));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(cond), Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(b), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x91);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&cond));
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$a));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::SIGNED(_) | Value::UNSIGNED(_) | Value::DECIMAL(_), Value::IDENT(a), Value::IDENT(b), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x92);
                res.append(&mut rainbow_wrapper::conversions::to_immediate(&$cond));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&a));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            (Value::IDENT(cond), Value::IDENT(a), Value::IDENT(b), Value::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x93);
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&cond));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&a));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&b));
                res.append(&mut rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            }
            _ => panic!("invalid arguments [{}, {}, {}, {}] passed to `callc` instruction", $cond, $a, $b, $out)
        }
    };
}