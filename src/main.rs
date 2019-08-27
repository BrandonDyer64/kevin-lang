extern crate regex;
mod lexer;
use lexer::{ Lexer, Token };

static SOURCE: &'static str = r#"
fn Main(args: symbol[]): symbol {
    // This is a comment
    let mut array = [i32; 3] { it }
    let some_symbol = 6 / 9
}
"#;

fn main() {
    let lexer = Lexer::new(SOURCE);
    let mut indent = 0;
    for token in lexer {
        if token == Token::RBrace
        || token == Token::RParen {
            indent -= 1;
            println!("{}]", "    ".repeat(indent));
        }
        println!("{}{:?}", "    ".repeat(indent), token);
        if token == Token::LBrace
        || token == Token::LParen {
            println!("{}[", "    ".repeat(indent));
            indent += 1;
        }
    }
}