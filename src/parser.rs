use crate::tokenizer::{TokenType, TokenIterator};

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
    Nil,
}

pub fn parse_expression(current: &mut TokenIterator) -> Result<Expression, String> {
    match &current.get_state().ok_or("Invalid state!") {
        Ok(TokenType::OpenParen) => {
            // TODO: clean this code up. Organize better, convert `if let`'s to match
            let next = current.next().ok_or("Unexpected EOF!");
            if let Ok(TokenType::If) = next {
                current.next();
                let cond = parse_expression(current)?;
                current.next();
                let if_branch = parse_expression(current)?;
                current.next();
                let else_branch = parse_expression(current)?;
                return Ok(Expression::If(
                    Box::new(cond),
                    Box::new(if_branch),
                    Box::new(else_branch),
                ));
            }
            if let Ok(TokenType::Define) = next {
                if let Ok(TokenType::Identifier(s)) = current.next().ok_or("Unexpected EOF!") {
                    current.next();
                    let expr = parse_expression(current)?;
                    return Ok(Expression::Define(s, Box::new(expr)));
                } else {
                    return Err(format!("Expected identifier after define!"));
                }
            }
            if let Ok(TokenType::Lambda) = next {
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
                return Ok(Expression::Lambda(args, expr));
            }
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
        Ok(TokenType::CloseParen) => Err(format!("Unexpected ')'!")),
        Ok(TokenType::Identifier(s)) => Ok(Expression::Identifier(s.to_string())),
        Ok(TokenType::Integer(n)) => Ok(Expression::Integer(*n)),
        Ok(TokenType::Number(n)) => Ok(Expression::Number(*n)),
        Ok(TokenType::Lambda) => Err(format!("Lambda not expected in this position!")),
        Ok(TokenType::Define) => Err(format!("Define not expected in this position!")),
        Ok(TokenType::If) => Err(format!("If not expected in this position!")),
        Ok(TokenType::True) => Ok(Expression::Bool(true)),
        Ok(TokenType::False) => Ok(Expression::Bool(false)),
        Err(_) => Err(format!("Error!")),
    }
}
