#[path = "opr.rs"]
pub mod opr;
pub use opr::Opr;

#[derive(PartialEq, Debug)]
pub enum Token {
    Binary,
    Colon,
    Comma,
    Comment(String),
    Else,
    Export,
    Fromm,
    Fun,
    Ident(String),
    If,
    Import,
    Integer(u64),
    LBrace,
    LBracket,
    Let,
    LineBreak,
    LParen,
    Mut,
    Float(f64),
    Op(Opr),
    RBrace,
    RBracket,
    RParen,
    Semicolon,
    Symbol(String),
    Then,
    Unary,
    Var,
}