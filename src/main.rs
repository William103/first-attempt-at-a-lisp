use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum TokenType {
    OpenParen,
    CloseParen,
    Lambda,
    Define,
    If,
    Identifier(String),
    Number(f64),
}

#[derive(Debug)]
struct TokenIterator {
    data: Vec<String>,
    index: usize,
}

impl Iterator for TokenIterator {
    type Item = TokenType;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index >= self.data.len() {
            return None;
        }
        self.get_state()
    }
}

impl TokenIterator {
    fn get_state(&self) -> Option<TokenType> {
        if self.index >= self.data.len() {
            return None;
        }
        let current = &self.data[self.index];
        match current.as_str() {
            "(" => Some(TokenType::OpenParen),
            ")" => Some(TokenType::CloseParen),
            s => {
                if let Ok(n) = s.parse::<f64>() {
                    Some(TokenType::Number(n))
                } else {
                    if s == "lambda" {
                        Some(TokenType::Lambda)
                    } else if s == "define" {
                        Some(TokenType::Define)
                    } else if s == "if" {
                        Some(TokenType::If)
                    } else {
                        Some(TokenType::Identifier(s.to_string()))
                    }
                }
            }
        }
    }
}

fn tokenize(s: &String) -> TokenIterator {
    let mut v = Vec::new();
    let mut c = s.chars();
    let mut tempstr = String::new();
    loop {
        let c = c.next();
        match c {
            Some(c) => {
                if c == '(' {
                    if !tempstr.is_empty() {
                        v.push(tempstr);
                        tempstr = String::new();
                    }
                    v.push(String::from("("));
                } else if c == ')' {
                    if !tempstr.is_empty() {
                        v.push(tempstr);
                        tempstr = String::new();
                    }
                    v.push(String::from(")"));
                } else if c.is_whitespace() && !tempstr.is_empty() {
                    v.push(tempstr);
                    tempstr = String::new();
                } else if !c.is_whitespace() {
                    tempstr.push(c);
                }
            }
            None => break,
        }
    }
    if !tempstr.is_empty() {
        v.push(tempstr);
    }
    TokenIterator { data: v, index: 0 }
}

#[derive(Clone, Debug)]
enum Expression {
    Number(f64),
    Identifier(String),
    SExpression(Box<Expression>, Vec<Expression>),
    Lambda(Vec<String>, Box<Expression>),
    Define(String, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Nil,
}

fn parse_expression(current: &mut TokenIterator) -> Option<Expression> {
    match &current.get_state() {
        Some(TokenType::OpenParen) => {
            let next = current.next();
            if let Some(TokenType::If) = next {
                current.next();
                let cond = parse_expression(current)?;
                current.next();
                let if_branch = parse_expression(current)?;
                current.next();
                let else_branch = parse_expression(current)?;
                return Some(Expression::If(
                    Box::new(cond),
                    Box::new(if_branch),
                    Box::new(else_branch),
                ));
            }
            if let Some(TokenType::Define) = next {
                if let Some(TokenType::Identifier(s)) = current.next() {
                    current.next();
                    let expr = parse_expression(current)?;
                    return Some(Expression::Define(s, Box::new(expr)));
                } else {
                    panic!("Expected identifier after define!");
                }
            }
            if let Some(TokenType::Lambda) = next {
                if let Some(TokenType::OpenParen) = current.next() {
                } else {
                    panic!("Expected '(' after 'lambda'!");
                }
                let mut args = Vec::new();
                loop {
                    match current.next() {
                        Some(TokenType::Identifier(s)) => args.push(s),
                        Some(TokenType::CloseParen) => break,
                        None => return None,
                        _ => panic!("Expected identifier or ')' in args list for lambda"),
                    }
                }
                current.next();
                let expr = Box::new(parse_expression(current)?);
                current.next();
                return Some(Expression::Lambda(args, expr));
            }
            let car = match next {
                Some(TokenType::CloseParen) => Expression::Nil,
                Some(_) => parse_expression(current).expect("Error parsing function"),
                None => panic!("Error! Unexpected end of file!"),
            };
            let mut cdr = Vec::new();
            loop {
                match current.next() {
                    Some(TokenType::CloseParen) => break,
                    Some(_) => cdr.push(parse_expression(current)?),
                    None => panic!("Error! Unexpected end of file!"),
                }
            }
            Some(Expression::SExpression(Box::new(car), cdr))
        }
        Some(TokenType::CloseParen) => panic!("Unexpected ')'!"),
        Some(TokenType::Identifier(s)) => Some(Expression::Identifier(s.to_string())),
        Some(TokenType::Number(n)) => Some(Expression::Number(*n)),
        Some(TokenType::Lambda) => panic!("Lambda not expected in this position!"),
        Some(TokenType::Define) => panic!("Define not expected in this position!"),
        Some(TokenType::If) => panic!("If not expected in this position!"),
        None => panic!("Error!"),
    }
}

#[derive(Clone, Debug)]
enum Value {
    Number(f64),
    Function(Vec<String>, Expression),
    Nil,
}

fn check_environment(expr: Expression, env: &HashMap<String, Value>) -> Option<Expression> {
    match expr {
        Expression::Identifier(s) => match env.get(&s) {
            Some(Value::Number(n)) => Some(Expression::Number(*n)),
            Some(Value::Function(params, body)) => {
                Some(Expression::Lambda(params.clone(), Box::new(body.clone())))
            }
            Some(Value::Nil) => Some(Expression::Nil),
            None => Some(Expression::Identifier(s.clone())),
        },
        Expression::Lambda(params, body) => Some(Expression::Lambda(
            params
                .iter()
                .cloned()
                .filter(|p| !env.contains_key(p))
                .collect(),
            Box::new(check_environment(*body, env)?),
        )),
        Expression::SExpression(head, tail) => Some(Expression::SExpression(
            Box::new(check_environment(*head, env)?),
            tail.iter()
                .map(|e| check_environment(e.clone(), env).unwrap()) // TODO: better error handling here
                .collect(),
        )),
        e => Some(e),
    }
}

fn eval_expression(expr: &Expression, env: &mut HashMap<String, Value>) -> Option<Value> {
    match expr {
        Expression::Number(n) => Some(Value::Number(*n)),
        Expression::Identifier(s) => env.get(s).cloned(),
        Expression::Nil => Some(Value::Nil),
        Expression::Lambda(params, body) => Some(Value::Function(
            params.clone(),
            check_environment(*body.clone(), env)?,
        )),
        Expression::Define(s, expr) => {
            let res = eval_expression(expr, env)?;
            env.insert(s.clone(), res);
            None
        }
        Expression::If(cond, if_branch, else_branch) => {
            let cond = eval_expression(&**cond, env)?;
            match cond {
                Value::Number(n) => {
                    if n != 0.0 {
                        eval_expression(&**if_branch, env)
                    } else {
                        eval_expression(&**else_branch, env)
                    }
                }
                _ => None,
            }
        }
        Expression::SExpression(head, tail) => {
            let args = tail
                .iter()
                .map(|v| {
                    eval_expression(v, env).expect(format!("Invalid expression!\nexpr:\t{:?}\nenv:\t{:?}", v, env).as_str())
                })
                .collect::<Vec<Value>>();
            match &**head {
                Expression::Identifier(s) => {
                    if s.as_str() == "+" {
                        return args.iter().fold(Some(Value::Number(0.0)), |acc, x| {
                            if let Value::Number(n) = x {
                                if let Some(Value::Number(n2)) = acc {
                                    return Some(Value::Number(n2 + *n));
                                }
                            }
                            panic!("{:?} is not a number!", x);
                        });
                    } else if s.as_str() == "*" {
                        return args.iter().fold(Some(Value::Number(1.0)), |acc, x| {
                            if let Value::Number(n) = x {
                                if let Some(Value::Number(n2)) = acc {
                                    return Some(Value::Number(n2 * *n));
                                }
                            }
                            panic!("{:?} is not a number!", x);
                        });
                    } else {
                        let f = env.get(s)?;
                        if let Value::Function(params, body) = f {
                            let mut map = env.clone();
                            assert!(args.len() <= params.len());
                            for (i, arg) in args.iter().enumerate() {
                                map.insert(params[i].clone(), arg.clone());
                            }
                            return eval_expression(&body, &mut map);
                        } else {
                            panic!("{} is not a known function!", s);
                        }
                    }
                }
                _ => (),
            }
            let res = eval_expression(&**head, env)?;
            match res {
                Value::Function(params, body) => {
                    let mut map = env.clone();
                    assert!(args.len() <= params.len()); // NOTE assert! here
                    for (i, arg) in args.iter().enumerate() {
                        map.insert(params[i].clone(), arg.clone());
                    }
                    eval_expression(&body, &mut map)
                }
                Value::Number(n) => panic!("{} is a number, not a function!", n),
                Value::Nil => panic!("Nil is not callable!"),
            }
        }
    }
}

fn main_loop() {
    let mut env: HashMap<String, Value> = HashMap::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("ready> ");
        stdout.flush().ok().expect("Error flushing!");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Error reading string!");

        if input == "env\n" {
            println!("{:?}", env);
            continue;
        }

        let mut tokens = tokenize(&input);
        let parsed = parse_expression(&mut tokens).expect("Error parsing input!");
        match eval_expression(&parsed, &mut env) {
            Some(Value::Number(n)) => println!("{}", n),
            _ => (),
        }
    }
}

fn main() {
    main_loop();
}
