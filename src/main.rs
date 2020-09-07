#[derive(Clone, Debug)]
enum TokenType {
    OpenParen,
    CloseParen,
    Identifier(String),
    Number(f64),
}

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
        let current = &self.data[self.index];
        match current.as_str() {
            "(" => Some(TokenType::OpenParen),
            ")" => Some(TokenType::CloseParen),
            s => {
                if let Ok(n) = s.parse::<f64>() {
                    Some(TokenType::Number(n))
                } else {
                    Some(TokenType::Identifier(s.to_string()))
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
    TokenIterator { data: v, index: 0 }
}

#[derive(Debug)]
enum Expression {
    Number(f64),
    Identifier(String),
    SExpression(Box<Expression>, Vec<Expression>),
    Nil,
}

fn parse_expression(current: &mut TokenIterator) -> Option<Expression> {
    match &current.get_state() {
        Some(TokenType::OpenParen) => {
            let car = match current.next() {
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
        Some(TokenType::CloseParen) => unreachable!(),
        Some(TokenType::Identifier(s)) => Some(Expression::Identifier(s.to_string())),
        Some(TokenType::Number(n)) => Some(Expression::Number(*n)),
        None => panic!("Error!"),
    }
}

fn eval_expression(expr: &Expression) -> f64 {
    match &expr {
        Expression::Number(n) => *n,
        Expression::Identifier(s) => panic!("{} doesn't evaluate to a number!", s),
        Expression::Nil => panic!("Nil doesn't evaluate to a number!"),
        Expression::SExpression(func, args) => match &**func {
            Expression::Number(n) => panic!("{} is a number, not a function!", n),
            Expression::Identifier(s) => match s.as_str() {
                "+" => args.iter().map(|e| eval_expression(e)).sum(),
                "*" => args.iter().map(|e| eval_expression(e)).product(),
                s => panic!("{} unsupported function!", s),
            },
            Expression::Nil => panic!("Nil is an invalid function!"),
            Expression::SExpression(_, _) => panic!("No first class functions yet!"),
        },
    }
}

fn main() {
    let src = "(+ 3 (* 4 (+ 1 1)) 3)";
    let mut test = tokenize(&src.to_string());
    let res = parse_expression(&mut test).unwrap();
    println!("{:?}\n", eval_expression(&res));
}
