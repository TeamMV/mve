use mvutils::try_from_string;

#[derive(PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Literal(Literal),
    Lifetime(String),

    Dot,
    Comma,
    Hash,
    Semicolon,
    Colon,
    DColon,
    QMark,

    //symbols
    LParen,
    //(
    RParen,
    //)
    LBrace,
    //{
    RBrace,
    //}
    LBracket,
    //[
    RBracket,
    //]
    Assign, //=,
    Arrow,  //->
    Dollar, // $

    Operator(Operator),
    OperatorAssign(Operator),
    Keyword(Keyword),

    EOF,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Operator {
    Plus,
    //+
    Minus,
    //-
    Divide,
    // /
    Multiply,
    //*
    Modulo,
    //%
    Lsh,
    //<<
    Rsh,
    //>>
    ARsh,
    //>>>
    BAnd,
    // &
    BOr,
    // |
    BXor,
    // ^
    LOr,
    // ||
    LAnd,
    // &&
    Not,
    // !
    Eq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    NotEq,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[try_from_string]
pub enum Keyword {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    F64,
    Char,
    Bool,
    Let,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    Loop,
    True,
    False,
    In,
    Return,
    Struct,
    Trait,
    Enum,
    Fn,
    Type,
    Impl,
    Crate,
    Pub,
    Extern,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}
