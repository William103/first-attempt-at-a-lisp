use crate::tokenizer::{TokenIterator, TokenType};

/// This enum represents all the possible expressions. It kind of ended up being almost a copy of
/// `Value`, and I frequently needed to switch between them. I'm not sure if that's a good idea,
/// but I have **ABSOLUTELY NO CLUE** what I'm doing, so whatever.
#[derive(Clone, Debug)]
pub enum Expression {
    Number(f64),
    Integer(isize),
    Identifier(String),
    SExpression(Box<Expression>, Vec<Expression>),
    Lambda(Vec<String>, Box<Expression>),
    Define(String, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Bool(bool),
    Pair(Box<Expression>, Box<Expression>),
    Nil,
}

/// This is a very important function. It takes an iterator over the tokens (which ended up
/// basically being a wrapper for a `Vec` on accident) and parses it into and `Expression`. It's a
/// pretty big, messy, recursive pile of pattern matching, but not quite as bad as
/// `eval_expression`. This was extremely difficult to write at first, but ended up not being too
/// bad once I figured it out. Lisp's syntax is just so wonderfully simple.
pub fn parse_expression(current: &mut TokenIterator) -> Result<Expression, String> {
    match &current.get_state().ok_or("Invalid state!") {
        Ok(TokenType::OpenParen) => {
            let next = current.next().ok_or("Unexpected EOF!");
            match next {
                Ok(TokenType::CloseParen) => Ok(Expression::Nil),
                Ok(TokenType::If) => {
                    current.next();
                    let cond = parse_expression(current)?;
                    current.next();
                    let if_branch = parse_expression(current)?;
                    current.next();
                    let else_branch = parse_expression(current)?;
                    Ok(Expression::If(
                        Box::new(cond),
                        Box::new(if_branch),
                        Box::new(else_branch),
                    ))
                }
                Ok(TokenType::Define) => {
                    if let Ok(TokenType::Identifier(s)) = current.next().ok_or("Unexpected EOF!") {
                        current.next();
                        let expr = parse_expression(current)?;
                        Ok(Expression::Define(s, Box::new(expr)))
                    } else {
                        Err(format!("Expected identifier after define!"))
                    }
                }
                Ok(TokenType::Lambda) => {
                    if let Ok(TokenType::OpenParen) = current.next().ok_or("Unexpected EOF!") {
                    } else {
                        return Err(format!("Expected '(' after 'lambda'!"));
                    }
                    let mut args = Vec::new();
                    loop {
                        match current.next().ok_or("Unexpected EOF!")? {
                            TokenType::Identifier(s) => args.push(s),
                            TokenType::CloseParen => break,
                            t => return Err(format!("Invalid token {:?}!", t)),
                        }
                    }
                    current.next();
                    let expr = Box::new(parse_expression(current)?);
                    current.next();
                    Ok(Expression::Lambda(args, expr))
                }
                _ => {
                    let car = match next? {
                        TokenType::CloseParen => Expression::Nil,
                        _ => parse_expression(current).expect("Error parsing function"),
                    };
                    let mut cdr = Vec::new();
                    loop {
                        match current.next().ok_or("Unexpected EOF!")? {
                            TokenType::CloseParen => break,
                            _ => cdr.push(parse_expression(current)?),
                        }
                    }
                    Ok(Expression::SExpression(Box::new(car), cdr))
                }
            }
        }
        Ok(TokenType::CloseParen) => Err(format!("Unexpected ')'!")),
        Ok(TokenType::Identifier(s)) => Ok(Expression::Identifier(s.to_string())),
        Ok(TokenType::Integer(n)) => Ok(Expression::Integer(*n)),
        Ok(TokenType::Number(n)) => Ok(Expression::Number(*n)),
        Ok(TokenType::Lambda) => Err(format!("Lambda not expected in this position!")),
        Ok(TokenType::Define) => Err(format!("Define not expected in this position!")),
        Ok(TokenType::If) => Err(format!("If not expected in this position!")),
        Ok(TokenType::True) => Ok(Expression::Bool(true)),
        Ok(TokenType::False) => Ok(Expression::Bool(false)),
        Ok(TokenType::SingleQuote) => {
            if let Some(TokenType::OpenParen) = current.next() {
                let mut vals = Vec::new();
                loop {
                    if let Some(TokenType::CloseParen) = current.next() {
                        break;
                    } else {
                        vals.push(parse_expression(current)?);
                    }
                }
                vals.iter().rfold(Ok(Expression::Nil), |acc, x| {
                    Ok(Expression::Pair(
                        Box::new(x.clone()),
                        Box::new(acc.unwrap()),
                    ))
                })
            } else {
                Err(format!("Expected '(' after quote!"))
            }
        }
        Err(_) => Err(format!("Error!")),
    }
}
