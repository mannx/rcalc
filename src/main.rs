#![allow(non_snake_case)]
use std::io;
use std::io::Write;

use phf::phf_map;

#[derive(PartialEq, Clone)]
enum Operation {
    Value(f64), // value to push to stack
    Add,
    Sub,
    Mul,
    Div,
    Sum,   // Sum the entire stack togther
    Clear, // clear the entire stack
    Drop,  // drop the top most item
    Swap,  // swap the top 2 items
}

static OPERATION_PARSE: phf::Map<&'static str, Operation> = phf_map! {
    "+" => Operation::Add,
    "-" => Operation::Sub,
    "*" => Operation::Mul,
    "/" => Operation::Div,
    "=" => Operation::Sum,
    "c" => Operation::Clear,
    "d" => Operation::Drop,
    "s" => Operation::Swap,
};

fn get_val(input: &str) -> Option<f64> {
    let float = input.parse::<f64>();
    match float {
        Ok(f) => return Some(f),
        Err(_) => (),
    }

    let dec = input.parse::<i32>();
    match dec {
        Ok(i) => return Some(f64::from(i)),
        Err(_) => (),
    }

    return None;
}

// checks if this is operation type (anything not a value)
fn get_op(input: &str) -> Option<Operation> {
    let op = OPERATION_PARSE.get(input).cloned();
    return match op {
        Some(op) => Some(op),
        None => None,
    };
}

// parse the input and extract the operation if any
// returns None if unable to parse
fn parse_input(input: &str) -> Option<Operation> {
    match get_op(&input) {
        Some(o) => return Some(o),
        None => (),
    }

    match get_val(&input) {
        Some(i) => return Some(Operation::Value(i)),
        None => (),
    }

    return None;
}

fn display_stack(stack: &Vec<f64>) {
    let mut lock = io::stdout().lock();
    for i in stack {
        write!(lock, "{} ", i).expect("Unable to write to stdout!");
    }

    print!("\n\r");
    io::stdout().flush().expect("Unable to flush stdout!");
}

fn add_op(rhs: f64, lhs: f64) -> f64 {
    rhs + lhs
}

fn sub_op(rhs: f64, lhs: f64) -> f64 {
    rhs - lhs
}

fn mul_op(rhs: f64, lhs: f64) -> f64 {
    rhs * lhs
}

fn div_op(rhs: f64, lhs: f64) -> f64 {
    lhs / rhs
}

fn get_top2(stack: &mut Vec<f64>) -> Option<(f64, f64)> {
    let a = match stack.pop() {
        Some(n) => n,
        None => return None,
    };

    let b = match stack.pop() {
        Some(n) => n,
        None => return None,
    };

    return Some((a, b));
}

fn math_func(stack: &mut Vec<f64>, f: fn(f64, f64) -> f64) -> Option<f64> {
    // let a = match stack.pop() {
    //     Some(n) => n,
    //     None => return None,
    // };

    // let b = match stack.pop() {
    //     Some(n) => n,
    //     None => return None,
    // };

    // return Some(f(a, b));
    let (a, b) = match get_top2(stack) {
        Some((a, b)) => (a, b),
        None => return None,
    };

    return Some(f(a, b));
}

fn sum_op(stack: &mut Vec<f64>) -> Option<f64> {
    let total = stack.iter().sum();
    stack.clear();
    return Some(total);
}

// swap_stacks swaps the top 2 items on the stack, returns false if error or less than 2 items
fn swap_stack(stack: &mut Vec<f64>) -> bool {
    if stack.len() < 2 {
        return false;
    }

    let (a, b) = match get_top2(stack) {
        Some((a, b)) => (a, b),
        None => return false,
    };

    stack.push(a);
    stack.push(b);
    return true;
}

fn do_op(stack: &mut Vec<f64>, op: &Operation) -> Result<bool, &'static str> {
    // match block for functions that don't return any data
    // if we return true, skip the rest of the function
    let ret = match op {
        Operation::Clear => {
            stack.clear();
            true
        }
        Operation::Drop => {
            stack.pop();
            true
        }
        Operation::Swap => swap_stack(stack),
        _ => false,
    };

    if ret == true {
        return Ok(true);
    }

    let res = match op {
        Operation::Add => math_func(stack, add_op),
        Operation::Sub => math_func(stack, sub_op),
        Operation::Mul => math_func(stack, mul_op),
        Operation::Div => math_func(stack, div_op),
        Operation::Sum => sum_op(stack),
        _ => None,
    };

    match res {
        Some(n) => {
            stack.push(n);
            return Ok(true); // retunr the new value
        }
        None => return Err("Unable to perform operation"),
    }
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
                    println!("< --INVALID INPUT --");
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
