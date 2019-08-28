
mod lexer;
use lexer::{ Lexer, Token };

static SOURCE: &'static str = r#"
import Console from "console"

fn Main(args: symbol[]): symbol {
    // This is a comment
    let mut array = [i32; 3] { it }
    let some_symbol = 6 / 9 * ( 8 + 1 )
}

export Main
"#;

fn main() {
    let lexer = Lexer::new(SOURCE);
    let mut indent = 0;
    for token in lexer {
        let (token, pos) = token;
        if token == Token::RBrace
        || token == Token::RParen {
            indent -= 1;
            println!("{:>5}: {}]", pos, "    ".repeat(indent));
        }
        println!("{:>5}: {}{:?}", pos, "    ".repeat(indent), token);
        if token == Token::LBrace
        || token == Token::LParen {
            println!("{:>5}: {}[", pos, "    ".repeat(indent));
            indent += 1;
        }
    }
}