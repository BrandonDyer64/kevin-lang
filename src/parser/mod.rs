mod lexer;
use lexer::{ Lexer, Token };

mod expr;
use expr::Expr;

use std::iter::Peekable;
use std::ops::DerefMut;

pub struct Parser<'a> {
    lexer: Box<Peekable<Lexer<'a>>>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { lexer: Box::new(input.peekable()) }
    }
}

impl<'a> Iterator for Parser<'a> {

    type Item = (Expr, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
    }
}
