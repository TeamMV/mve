use crate::ui::xml::lexer::{Literal, Token};
use crate::ui::xml::tree::{Attribute, Child, NamedNode, Node, UnnamedNode};
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        if let Token::Tag = token {
            nodes.push(parse_node(&mut iter))
        } else {
            panic!("UI syntax error: Expected '<' but found '{token}'");
        }
    }

    nodes
}

fn parse_node(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let ident = iter
        .next()
        .expect("UI syntax error: Expected identifier but found EOF");

    let tag = match ident {
        Token::CloseTag => {
            let children = parse_children(iter, None);
            return Node::Unnamed(UnnamedNode { children });
        }
        Token::Ident(i) => i,
        _ => panic!("UI syntax error: Expected identifier but found {ident}"),
    };

    let mut id = String::new();
    let mut class = Vec::new();
    let mut attributes = Vec::new();

    let mut token = iter
        .next()
        .expect("UI syntax error: Expected identifier or '<' but found EOF");
    while !(token == Token::CloseTag || token == Token::InlineCloseTag) {
        let Token::Ident(name) = token else {
            panic!("UI syntax error: Expected identifier but found {token}")
        };

        let next = iter
            .next()
            .expect("UI syntax error: Expected '=' but found EOF");
        if next != Token::Equals {
            panic!("UI syntax error: Expected '=' but found {next}");
        }

        let next = iter
            .next()
            .expect("UI syntax error: Expected '=' but found EOF");
        let Token::Literal(value) = next else {
            panic!("UI syntax error: Expected literal but found {next}")
        };

        match name.as_str() {
            "id" => {
                let Literal::String(str) = value else {
                    panic!("UI syntax error: Literal for 'id' must be a string, but {value} was provided");
                };
                id = str;
            }
            "class" => {
                let Literal::String(str) = value else {
                    panic!("UI syntax error: Literal for 'class' must be a string, but {value} was provided");
                };
                class = str.split_whitespace().map(ToString::to_string).collect();
            }
            _ => attributes.push(Attribute { name, value }),
        }

        token = iter
            .next()
            .expect("UI syntax error: Expected identifier or '<' but found EOF");
    }

    let children = if token == Token::CloseTag {
        parse_children(iter, Some(tag.clone()))
    } else {
        Vec::new()
    };

    Node::Named(NamedNode {
        tag,
        id,
        class,
        attributes,
        children,
    })
}

fn parse_children(iter: &mut Peekable<IntoIter<Token>>, closing: Option<String>) -> Vec<Child> {
    let mut children = Vec::new();
    while let Some(token) = iter.next() {
        match token {
            Token::Tag => children.push(Child::Node(parse_node(iter))),
            Token::Contents(c) => children.push(Child::String(
                c.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" "),
            )),
            Token::ClosingTag => {
                let next = iter
                    .next()
                    .expect("UI syntax error: Expected identifier or '>' but found EOF");
                if let Some(tag) = closing {
                    let Token::Ident(name) = next else {
                        panic!("UI syntax error: Expected identifier but found {next}");
                    };
                    if tag != name {
                        panic!("UI syntax error: Expected closing tag for {tag} but found closing tag for {name}");
                    }
                    let next = iter
                        .next()
                        .expect("UI syntax error: Expected '>' but found EOF");
                    if next != Token::CloseTag {
                        panic!("UI syntax error: Expected '>' but found {next}");
                    }
                } else {
                    if next != Token::CloseTag {
                        panic!("UI syntax error: Expected '>' but found {next}");
                    }
                }
                return children;
            }
            t => {
                panic!(
                    "UI syntax error: Expected contents or '</{}>' but found {}",
                    closing.unwrap_or(String::new()),
                    t
                );
            }
        }
    }
    panic!(
        "UI syntax error: Expected '</{}>' but found EOF",
        closing.unwrap_or(String::new())
    );
}
