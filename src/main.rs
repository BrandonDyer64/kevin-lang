mod lexer;
use lexer::{ Lexer, Token };

mod parser;
use parser::{ Parser, Expr };

extern crate spinners;
use spinners::{ Spinner, Spinners };
use std::thread::sleep;
use std::time::Duration;

static SOURCE: &'static str = r#"
let a = 6
"#;

fn main() {
    let sp = Spinner::new(Spinners::Dots12, "Building".into());
    let mut lexer = Lexer::new(SOURCE);
    let tokens = lexer.by_ref().collect();
    let parser = Parser::new(tokens);
    sp.stop();
}