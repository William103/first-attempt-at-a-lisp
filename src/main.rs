mod eval;
mod parser;
mod tokenizer;
use crate::eval::{eval_expression, Value};
use crate::parser::parse_expression;
use crate::tokenizer::tokenize;

use std::collections::HashMap;
use std::io::prelude::*;

// TODO: stl?

#[derive(Debug)]
struct StdioLinesIterator {
    stdin: std::io::Stdin,
}

impl StdioLinesIterator {
    fn new() -> StdioLinesIterator {
        StdioLinesIterator {
            stdin: std::io::stdin()
        }
    }
}

impl Iterator for StdioLinesIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut s = String::new();
        if self.stdin.read_line(&mut s).is_err() {
            return None;
        }
        Some(s)
    }
}

fn main_loop<T: Iterator<Item = String> + std::fmt::Debug>(mut lines: T, repl: bool) {
    let mut env: HashMap<String, Value> = HashMap::new();
    let mut stdout = std::io::stdout();
    loop {
        if repl {
            print!("ready> ");
            stdout.flush().ok().expect("Error flushing!");
        }
        let mut input = lines.next().expect("Unexpected EOF").to_string();
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
            input.push_str(lines.next().expect("Unexpected EOF").as_str());
        }

        if input.trim() == "env" {
            println!("{:#?}", env);
            continue;
        } else if input.trim() == "exit" {
            return;
        }

        let mut tokens = tokenize(&input);
        let parsed = parse_expression(&mut tokens);
        if let Err(msg) = parsed {
            println!("{}", msg);
            if repl {
                continue;
            } else {
                eprintln!("ERROR!!!");
                return;
            }
        }
        let parsed = parsed.unwrap();
        if repl {
            match eval_expression(&parsed, &mut env) {
                Ok(Value::Number(n)) => println!("{}", n),
                Ok(Value::Bool(b)) => println!("{}", b),
                Ok(Value::Integer(n)) => println!("{}", n),
                Err(msg) => println!("{}", msg),
                _ => (),
            }
        } else {
            match eval_expression(&parsed, &mut env) {
                Ok(_) => (),
                Err(msg) => {
                    eprintln!("ERROR!!!: {}", msg);
                    return;
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        let contents = std::fs::read_to_string(filename)
            .expect(format!("Couldn't read file {}", filename).as_str());
        main_loop(contents.lines().map(|s| s.to_string()).filter(|s| s.len() > 0), false);
    } else {
        let lines = StdioLinesIterator::new();
        main_loop(lines, true);
    }
}
