use crate::prelude::*;

pub fn do_op(stack: &mut Vec<f64>, op: &Operation) -> Result<bool, &'static str> {
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
        Operation::Dup => {
            if stack.len() > 0 {
                stack.push(stack[stack.len() - 1]);
                true
            } else {
                return Err("Empty Stack");
            }
        }
        Operation::Help => {
            show_help();
            true
        }

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

fn show_help() {
    for (key, (_, desc)) in &OPERATION_PARSE {
        println!("{} => {}", key, desc);
    }
}
