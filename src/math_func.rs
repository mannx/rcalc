// use crate::prelude::*;

pub fn add_op(rhs: f64, lhs: f64) -> f64 {
    rhs + lhs
}

pub fn sub_op(rhs: f64, lhs: f64) -> f64 {
    lhs - rhs
}

pub fn mul_op(rhs: f64, lhs: f64) -> f64 {
    rhs * lhs
}

pub fn div_op(rhs: f64, lhs: f64) -> f64 {
    lhs / rhs
}

pub fn get_top2(stack: &mut Vec<f64>) -> Option<(f64, f64)> {
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

pub fn math_func(stack: &mut Vec<f64>, f: fn(f64, f64) -> f64) -> Option<f64> {
    let (a, b) = match get_top2(stack) {
        Some((a, b)) => (a, b),
        None => return None,
    };

    return Some(f(a, b));
}

pub fn sum_op(stack: &mut Vec<f64>) -> Option<f64> {
    let total = stack.iter().sum();
    stack.clear();
    return Some(total);
}

// swap_stacks swaps the top 2 items on the stack, returns false if error or less than 2 items
pub fn swap_stack(stack: &mut Vec<f64>) -> bool {
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
