use crate::ui::xml::lexer::Literal;

#[derive(Debug, Clone)]
pub enum Node {
    Named(NamedNode),
    Unnamed(UnnamedNode),
}

#[derive(Debug, Clone)]
pub enum Child {
    Node(Node),
    String(String),
}

#[derive(Debug, Clone)]
pub struct NamedNode {
    pub tag: String,
    pub id: String,
    pub class: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Child>,
}

#[derive(Debug, Clone)]
pub struct UnnamedNode {
    pub children: Vec<Child>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: Literal,
}
