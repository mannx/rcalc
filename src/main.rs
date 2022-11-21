use std::io;
use std::io::Write;

use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Value {
    pub val: f64,
}

fn float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((decimal, char('.'), opt(decimal)))),
    ))(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn get_dec(input: &str) -> Option<&str> {
    let res = decimal(&input);

    return match res {
        Ok((_, o)) => Some(o),
        _ => None,
    };
}

fn get_float(input: &str) -> Option<&str> {
    let res = float(&input);

    return match res {
        Ok((_, o)) => Some(o),
        _ => None,
    };
}

fn get_val(input: &str) -> Option<Value> {
    let val = match get_float(&input) {
        Some(s) => s.parse::<f64>().unwrap(),
        None => match get_dec(&input) {
            Some(s) => s.parse::<f64>().unwrap(),
            None => f64::NAN,
        },
    };

    if val == f64::NAN {
        return None;
    } else {
        return Some(Value { val });
    }
}

// fn to_val(input: &str) -> IResult<&str, Value> {
//     // check for floating value first (requires . in)
//     let res = float(&input);
//     let mut val = Value { val: 0. };
// }

fn main() {
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");

        // trim the string of any excess whitespace first
        input = input.trim().to_string();
        println!(" [input: ({})]", input);

        let res = get_val(&input);
        match res {
            Some(n) => println!(">{}", n.val),
            None => println!("> [Not a decimal]"),
        }

        // let res = float(&input);

        // match res {
        //     Ok((a, b)) => {
        //         println!("< (a) = {}", a);
        //         println!("< (b) = {}", b);
        //     }
        //     Err(e) => println!("Unable to parse: {}", e),
        // }
    }
}
