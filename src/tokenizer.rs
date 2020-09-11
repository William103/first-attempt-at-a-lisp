#[derive(Clone, Debug)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    Lambda,
    Define,
    If,
    Identifier(String),
    Number(f64),
}

#[derive(Debug)]
pub struct TokenIterator {
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
    pub fn get_state(&self) -> Option<TokenType> {
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
                } else { // TODO: match?
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

pub fn tokenize(s: &String) -> TokenIterator {
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