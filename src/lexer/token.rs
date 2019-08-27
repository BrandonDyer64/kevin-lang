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
    Extern,
    Fun,
    Ident(String),
    If,
    LBrace,
    LBracket,
    Let,
    LineBreak,
    LParen,
    Mut,
    Number(f64),
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