#[derive(Clone, Debug)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    Lambda,
    Define,
    If,
    True,
    False,
    SingleQuote,
    Identifier(String),
    Number(f64),
    Integer(isize),
    Char(char),
    String(String),
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
        if current.starts_with("#\\") {
            let mut it = current.chars().skip(2);
            let c = it.next();
            if it.next().is_none() {
                if let Some(c) = c {
                    return Some(TokenType::Char(c));
                }
            } else {
                return None;
            }
        } else if current.starts_with("\"") {
            return Some(TokenType::String(current.clone()));
        }
        match current.as_str() {
            "'" => Some(TokenType::SingleQuote),
            "(" => Some(TokenType::OpenParen),
            ")" => Some(TokenType::CloseParen),
            "#t" => Some(TokenType::True),
            "#f" => Some(TokenType::False),
            s => {
                if let Ok(n) = s.parse::<isize>() {
                    Some(TokenType::Integer(n))
                } else {
                    if let Ok(n) = s.parse::<f64>() {
                        Some(TokenType::Number(n))
                    } else {
                        match s {
                            "lambda" => Some(TokenType::Lambda),
                            "define" => Some(TokenType::Define),
                            "if" => Some(TokenType::If),
                            _ => Some(TokenType::Identifier(s.to_string())),
                        }
                    }
                }
            }
        }
    }
}

pub fn tokenize(s: &String) -> TokenIterator {
    let mut v = Vec::new();
    let mut chars = s.chars();
    let mut tempstr = String::new();
    let mut in_string = false;
    loop {
        let c = chars.next();
        match c {
            Some(c) => {
                if !in_string {
                    if c == '"' {
                        in_string = true;
                        if !tempstr.is_empty() {
                            v.push(tempstr);
                            tempstr = String::new();
                        }
                        tempstr.push('"');
                    } else if c == '(' {
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
                } else {
                    match c {
                        '\\' => match chars.next() {
                            Some('n') => tempstr.push('\n'),
                            Some('t') => tempstr.push('\t'),
                            Some('\\') => tempstr.push('\\'),
                            Some('"') => tempstr.push('"'),
                            _ => (),
                        },
                        '"' => {
                            tempstr.push('"');
                            v.push(tempstr);
                            tempstr = String::new();
                            in_string = false;
                        }
                        c => tempstr.push(c),
                    }
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
