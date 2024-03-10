use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Tag,
    ClosingTag,
    CloseTag,
    InlineCloseTag,
    Ident(String),
    Literal(Literal),
    Equals,
    Contents(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Tag => "<".to_string(),
            Token::ClosingTag => "</".to_string(),
            Token::CloseTag => ">".to_string(),
            Token::InlineCloseTag => "/>".to_string(),
            Token::Ident(i) => i.to_string(),
            Token::Literal(i) => i.to_string(),
            Token::Equals => "=".to_string(),
            Token::Contents(c) => c.to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
}

impl Display for Literal {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Literal::String(s) => format!("\"{s}\""),
            Literal::Int(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
        };
        write!(f1, "{}", str)
    }
}

pub fn tokenize(xml: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = xml.chars().peekable();
    let mut inside = false;

    while let Some(c) = iter.next() {
        match c {
            '<' => {
                if iter.peek() == Some(&'<') {
                    iter.next();
                    extract_contents(&mut tokens, &mut iter, &mut inside, '<');
                } else if iter.peek() == Some(&'/') {
                    iter.next();
                    tokens.push(Token::ClosingTag);
                    inside = true;
                } else {
                    tokens.push(Token::Tag);
                    inside = true;
                }
            }
            '>' if inside => {
                tokens.push(Token::CloseTag);
                inside = false;
            }
            '/' if inside => {
                if iter.peek() == Some(&'>') {
                    iter.next();
                    tokens.push(Token::InlineCloseTag);
                    inside = false;
                }
            }
            '"' if inside => {
                let mut value = String::new();
                while let Some(&next) = iter.peek() {
                    if next == '"' {
                        iter.next();
                        break;
                    }
                    value.push(iter.next().unwrap());
                }
                tokens.push(Token::Literal(Literal::String(value)));
            }
            '=' if inside => tokens.push(Token::Equals),
            _ if inside => {
                if c.is_alphabetic() || c == '_' {
                    let mut ident = c.to_string();
                    while let Some(&next) = iter.peek() {
                        if !next.is_alphanumeric() && next != '_' {
                            break;
                        }
                        ident.push(iter.next().unwrap());
                    }
                    tokens.push(Token::Ident(ident));
                }
                else if c.is_numeric() {
                    let mut num = c.to_string();
                    let mut is_float = false;

                    while let Some(&next) = iter.peek() {
                        if next == '.' {
                            is_float = true;
                        }

                        if !next.is_numeric() && next != '_' && next != '.' {
                            break;
                        }
                        num.push(iter.next().unwrap());
                    }

                    let number = num.replace("_", "");
                    let literal = if is_float {
                        Literal::Float(number.parse().unwrap())
                    } else {
                        Literal::Int(number.parse().unwrap())
                    };
                    tokens.push(Token::Literal(literal));
                }
            }
            _ if !c.is_whitespace() && !inside => {
                extract_contents(&mut tokens, &mut iter, &mut inside, c);
            }
            _ => {}
        }
    }

    tokens
}

fn extract_contents(tokens: &mut Vec<Token>, iter: &mut Peekable<Chars>, inside: &mut bool, c: char) {
    let mut contents = c.to_string();
    let mut open = false;
    let mut open_closing = false;
    while let Some(&next) = iter.peek() {
        if next == '<' {
            iter.next();
            if let Some(&'<') = iter.peek() {
                contents.push('<');
                continue;
            } else if let Some(&'/') = iter.peek() {
                open_closing = true;
                break;
            } else {
                open = true;
                break;
            }
        }
        contents.push(iter.next().unwrap());
    }
    tokens.push(Token::Contents(contents));
    if open {
        tokens.push(Token::Tag);
        *inside = true;
    } else if open_closing {
        tokens.push(Token::ClosingTag);
        *inside = true;
    }
}
