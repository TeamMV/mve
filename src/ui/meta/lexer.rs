use crate::ui::meta::token::{Keyword, Literal, Operator, Token};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Lexer {
    input: Peekable<IntoIter<char>>,
    revert: Vec<Token>,
    done: bool,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            input: code.chars().collect::<Vec<_>>().into_iter().peekable(),
            revert: Vec::with_capacity(3),
            done: false,
        }
    }

    pub fn revert(&mut self, token: Token) {
        self.revert.push(token);
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.revert.is_empty() {
            return self.revert.pop();
        }

        while self.input.peek().is_some_and(|x| x.is_ascii_whitespace()) {
            self.input.next();
        }
        return match self.input.next() {
            Some(c) => {
                return match c {
                    '/' => {
                        if self.input.peek().is_some_and(|x| *x == '/') {
                            while self.input.peek().is_some_and(|x| *x != '\n') {
                                self.input.next();
                            }
                            return self.next();
                        }
                        if self.input.peek().is_some_and(|x| *x == '*') {
                            self.input.next();
                            while self.input.next().is_some_and(|x| x != '*') {
                                if self.input.peek().is_some_and(|x| *x == '/') {
                                    break;
                                }
                            }
                            self.input.next();
                            return self.next();
                        }

                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::Divide));
                        }

                        Some(Token::Operator(Operator::Divide))
                    }
                    '#' => Some(Token::Hash),
                    '*' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::Multiply));
                        }

                        Some(Token::Operator(Operator::Multiply))
                    }

                    '+' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::Plus));
                        }

                        Some(Token::Operator(Operator::Plus))
                    }

                    '-' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::Minus));
                        }

                        if self.input.peek().is_some_and(|x| *x == '>') {
                            self.input.next();
                            return Some(Token::Arrow);
                        }

                        Some(Token::Operator(Operator::Minus))
                    }

                    '%' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::Modulo));
                        }

                        Some(Token::Operator(Operator::Modulo))
                    }

                    '=' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::Operator(Operator::Eq));
                        }

                        Some(Token::Assign)
                    }

                    '!' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::Operator(Operator::NotEq));
                        }

                        Some(Token::Operator(Operator::Not))
                    }

                    '<' => {
                        return match self.input.peek() {
                            Some(c) => {
                                let c = *c;
                                return match c {
                                    '<' => {
                                        self.input.next();
                                        if self.input.peek().is_some_and(|x| *x == '=') {
                                            self.input.next();
                                            return Some(Token::OperatorAssign(Operator::Lsh));
                                        }
                                        Some(Token::Operator(Operator::Lsh))
                                    }

                                    '=' => {
                                        self.input.next();
                                        Some(Token::Operator(Operator::LessEq))
                                    }

                                    _ => Some(Token::Operator(Operator::Less)),
                                };
                            }

                            _ => None,
                        }
                    }

                    '>' => {
                        return match self.input.peek() {
                            Some(c) => {
                                let c = *c;
                                return match c {
                                    '>' => {
                                        self.input.next();
                                        if self.input.peek().is_some_and(|x| *x == '>') {
                                            self.input.next();
                                            if self.input.peek().is_some_and(|x| *x == '=') {
                                                self.input.next();
                                                return Some(Token::OperatorAssign(Operator::ARsh));
                                            }
                                            return Some(Token::Operator(Operator::ARsh));
                                        }
                                        if self.input.peek().is_some_and(|x| *x == '=') {
                                            self.input.next();
                                            return Some(Token::OperatorAssign(Operator::Rsh));
                                        }
                                        Some(Token::Operator(Operator::Rsh))
                                    }

                                    '=' => {
                                        self.input.next();
                                        Some(Token::Operator(Operator::GreaterEq))
                                    }

                                    _ => Some(Token::Operator(Operator::Greater)),
                                };
                            }

                            _ => None,
                        }
                    }

                    ',' => Some(Token::Comma),

                    ';' => Some(Token::Semicolon),

                    ':' => {
                        if self.input.peek().is_some_and(|x| *x == ':') {
                            self.input.next();
                            return Some(Token::DColon);
                        }

                        Some(Token::Colon)
                    }

                    '?' => Some(Token::QMark),

                    '.' => Some(Token::Dot),

                    '(' => Some(Token::LParen),

                    ')' => Some(Token::RParen),

                    '[' => Some(Token::LBracket),

                    ']' => Some(Token::RBracket),

                    '{' => Some(Token::LBrace),

                    '}' => Some(Token::RBrace),

                    '$' => Some(Token::Dollar),

                    '&' => {
                        if self.input.peek().is_some_and(|x| *x == '&') {
                            self.input.next();
                            return Some(Token::Operator(Operator::LAnd));
                        }

                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::BAnd));
                        }

                        Some(Token::Operator(Operator::BAnd))
                    }

                    '|' => {
                        if self.input.peek().is_some_and(|x| *x == '|') {
                            self.input.next();
                            return Some(Token::Operator(Operator::LOr));
                        }

                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::BOr));
                        }

                        Some(Token::Operator(Operator::BOr))
                    }

                    '^' => {
                        if self.input.peek().is_some_and(|x| *x == '=') {
                            self.input.next();
                            return Some(Token::OperatorAssign(Operator::BXor));
                        }

                        Some(Token::Operator(Operator::BXor))
                    }

                    _ => {
                        let mut s: String = String::new();
                        if c.is_ascii_alphabetic() || c == '_' {
                            s.push(c);
                            while self
                                .input
                                .peek()
                                .is_some_and(|x| (*x).is_ascii_alphanumeric() || *x == '_')
                            {
                                s.push(self.input.next().unwrap());
                            }

                            let keyword: Result<Keyword, ()> = s.clone().try_into();
                            return if let Ok(keyword) = keyword {
                                Some(Token::Keyword(keyword))
                            } else {
                                Some(Token::Ident(s))
                            };
                        }

                        if c.is_ascii_digit() {
                            s.push(c);
                            while self.input.peek().is_some_and(|x| (*x).is_ascii_digit()) {
                                s.push(self.input.next().unwrap());
                            }

                            if self.input.peek().is_some_and(|x| *x == '.') {
                                s.push('.');
                            } else {
                                return Some(Token::Literal(Literal::Int(
                                    s.parse::<i64>().unwrap(),
                                )));
                            }
                            self.input.next();
                            while self.input.peek().is_some_and(|x| (*x).is_ascii_digit()) {
                                s.push(self.input.next().unwrap());
                            }

                            return Some(Token::Literal(Literal::Float(s.parse::<f64>().unwrap())));
                        }

                        if c == '"' {
                            while self.input.peek().is_some_and(|x| *x != '"') {
                                let c = self.input.next().unwrap();
                                s.push(c);
                                if c == '\\' {
                                    s.push(self.input.next().unwrap());
                                }
                            }
                            self.input.next();
                            return Some(Token::Literal(Literal::String(s)));
                        }

                        if c == '\'' {
                            let mut buf = String::new();
                            let mut fmt = false;
                            let c = self.input.next().unwrap();
                            buf.push(c);
                            if c == '\\' {
                                fmt = true;
                                buf.push(self.input.next().unwrap());
                            }
                            let mut c = self.input.peek().unwrap();

                            if *c == '\'' {
                                return if fmt {
                                    Some(Token::Literal(Literal::String(buf)))
                                } else {
                                    Some(Token::Literal(Literal::Char(buf.pop().unwrap())))
                                };
                            }

                            while c.is_alphanumeric() || *c == '_' {
                                buf.push(*c);
                                self.input.next().unwrap();
                                c = self.input.peek().unwrap();
                            }

                            return Some(Token::Lifetime(buf));
                        }

                        panic!("ERROR: Invalid syntax! {}", c);
                    }
                };
            }

            None => {
                if self.done {
                    return None;
                }
                self.done = true;
                Some(Token::EOF)
            }
        };
    }
}
