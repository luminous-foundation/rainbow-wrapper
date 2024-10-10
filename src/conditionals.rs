#[macro_export]
macro_rules! block_cond {
    ($vec:expr, $type:expr, $left:expr, $condition:expr, $right:expr, $body:expr) => {
        {
            $vec.push(0xF7);
            $vec.push($type); // rust analyzer is a bit confused but it works
            $vec.append(&mut to_bytecode_string(&$left.to_string()));
            $vec.push($condition);
            $vec.append(&mut to_bytecode_string(&$right.to_string()));
            $vec.push(0xFE);
            $vec.append(&mut $body.clone());
            $vec.push(0xFD);
        }
    };
}

#[macro_export]
macro_rules! block {
    ($res:expr, $type:expr, $left:expr, $cond:expr, $right:expr, $body:expr) => {
        match $cond.to_string().as_str() { // this weird looking line is to allow String and &str to work
            "==" => { block_cond!($res, 0x00, $left, 0x00, $right, $body); }
            "!=" => { block_cond!($res, 0x00, $left, 0x01, $right, $body); }
            ">=" => { block_cond!($res, 0x00, $left, 0x02, $right, $body); }
            ">" =>  { block_cond!($res, 0x00, $left, 0x03, $right, $body); }
            "<=" => { block_cond!($res, 0x00, $left, 0x04, $right, $body); }
            "<" =>  { block_cond!($res, 0x00, $left, 0x05, $right, $body); }
            _ =>    { panic!("invalid condition passed to elseif_block"); }
        };
    };
}

#[macro_export]
macro_rules! if_block {
    ($left:expr, $cond:expr, $right:expr, $body:expr) => {
        {
            let mut res: Vec<u8> = Vec::new();
            
            block!(res, 0x00, $left, $cond, $right, $body);

            res
        }
    }
}

#[macro_export]
macro_rules! elseif_block {
    ($left:expr, $cond:expr, $right:expr, $body:expr) => {
        {
            let mut res: Vec<u8> = Vec::new();
            
            block!(res, 0x01, $left, $cond, $right, $body);

            res
        }
    }
}

#[macro_export]
macro_rules! else_block {
    ($body:expr) => {
        {
            let mut res: Vec<u8> = Vec::new();

            res.push(0xF7);
            res.push(0x02);
            res.push(0xFE);
            res.append(&mut $body.clone());
            res.push(0xFD);

            res
        }
    };
}

#[macro_export]
macro_rules! end_block {
    () => {
        vec![0xF7, 0x03]
    };
}