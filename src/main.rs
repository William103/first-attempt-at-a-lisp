mod eval;
mod parser;
mod tokenizer;
use crate::eval::{eval_expression, Value};
use crate::parser::parse_expression;
use crate::tokenizer::tokenize;

use std::collections::HashMap;
use std::io::prelude::*;

// TODO: stl?

fn main_loop() {
    let mut env: HashMap<String, Value> = HashMap::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("ready> ");
        stdout.flush().ok().expect("Error flushing!");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Error reading string!");
        loop {
            let mut parens = 0;
            for ch in input.chars() {
                match ch {
                    '(' => parens += 1,
                    ')' => parens -= 1,
                    _ => (),
                }
            }
            if parens == 0 {
                break;
            }
            stdin.read_line(&mut input).expect("Error reading string!");
        }

        if input == "env\n" {
            println!("{:#?}", env);
            continue;
        } else if input == "\n" {
            continue;
        } else if input == "exit\n" {
            break;
        }

        let mut tokens = tokenize(&input);
        let parsed = parse_expression(&mut tokens);
        if let Err(msg) = parsed {
            println!("{}", msg);
            continue;
        }
        let parsed = parsed.unwrap();
        match eval_expression(&parsed, &mut env) {
            Ok(Value::Number(n)) => println!("{}", n),
            Err(msg) => println!("{}", msg),
            _ => (),
        }
    }
}

fn main() {
    main_loop();
}
