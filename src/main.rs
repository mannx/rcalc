#![allow(non_snake_case)]

mod math_func;
mod operation;
mod parser;

mod prelude {
    pub use crate::math_func::*;
    pub use crate::operation::*;
    pub use crate::parser::*;

    pub use std::io;
    pub use std::io::Write;
}

use crate::prelude::*;

fn display_stack(stack: &Vec<f64>) {
    let mut lock = io::stdout().lock();
    for i in stack {
        write!(lock, "{} ", i).expect("Unable to write to stdout!");
    }

    print!("\n\r");
    io::stdout().flush().expect("Unable to flush stdout!");
}

fn main() {
    let mut stack: Vec<f64> = Vec::new();

    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");

        // trim the string of any excess whitespace first
        // and split into individual portions
        input = input.trim().to_string();
        let inpVec = input.split_whitespace().collect::<Vec<&str>>();

        for inp in inpVec {
            let val = parse_input(&inp);
            match val {
                None => {
                    println!("< --INVALID INPUT -- ? to see all options");
                    continue;
                }
                Some(op) => match op {
                    Operation::Value(v) => {
                        stack.push(v);
                    }
                    op => {
                        match do_op(&mut stack, &op) {
                            Ok(_) => (),
                            Err(e) => println!("  -- Error: {}", e),
                        }
                        ()
                    }
                },
            }
        }
        display_stack(&stack);
    }
}
