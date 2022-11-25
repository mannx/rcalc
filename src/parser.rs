// use crate::prelude::*;
use phf::phf_map;

#[derive(PartialEq, Clone)]
pub enum Operation {
    Value(f64), // value to push to stack
    Add,
    Sub,
    Mul,
    Div,
    Sum,   // Sum the entire stack togther
    Clear, // clear the entire stack
    Drop,  // drop the top most item
    Swap,  // swap the top 2 items
    Dup,   // duplicate the top item
    Nth,   // duplicate the nth item to the top of the stack (TOS -> nth)
    Help,  // show commands available
}

pub static OPERATION_PARSE: phf::Map<&'static str, (Operation, &'static str)> = phf_map! {
    "+" => (Operation::Add, "Add"),
    "-" => (Operation::Sub, "Sub"),
    "*" => (Operation::Mul, "Multiply"),
    "/" => (Operation::Div, "Division"),
    "=" => (Operation::Sum, "Sum entire stack"),
    "c" => (Operation::Clear, "Clear entire stack"),
    "d" => (Operation::Drop, "Drop top item"),
    "s" => (Operation::Swap, "Swap top 2 items"),
    "." => (Operation::Dup, "Duplicate top item"),
    "n" => (Operation::Nth, "Duplicate the Nth item on the stack to the top"),
    "?" => ( Operation::Help,"Show commands available"),
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
        Some((op, _)) => Some(op),
        None => None,
    };
}

// parse the input and extract the operation if any
// returns None if unable to parse
pub fn parse_input(input: &str) -> Option<Operation> {
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
