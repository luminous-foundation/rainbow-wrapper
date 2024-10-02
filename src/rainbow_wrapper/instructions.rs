#[macro_export]
macro_rules! add {
    ($left:expr, $right:expr, $out:expr) => {
        match ($left, $right, $out) {
            (Types::SIGNED(_) | Types::UNSIGNED(_) | Types::DECIMAL(_), Types::SIGNED(_) | Types::UNSIGNED(_) | Types::DECIMAL(_), Types::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                // i do not know how to make the fully qualified names shorter
                // pls make a PR if you do
                res.push(0x08);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate($left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate($right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            },
            // Case: ident, number
            (Types::IDENT(left), Types::SIGNED(_) | Types::UNSIGNED(_) | Types::DECIMAL(_), Types::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x09);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate($right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            },
            // Case: number, ident
            (Types::SIGNED(_) | Types::UNSIGNED(_) | Types::DECIMAL(_), Types::IDENT(right), Types::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0A);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_immediate($left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            },
            // Case: ident, ident
            (Types::IDENT(left), Types::IDENT(right), Types::IDENT(out)) => {
                let mut res: Vec<u8> = Vec::new();

                res.push(0x0B);
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&left));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&right));
                res.append(&mut rainbow_wrapper::rainbow_wrapper::conversions::to_bytecode_string(&out));

                res
            },
            _ => panic!("invalid arguments passed to `add` macro"),
        }
    };
}