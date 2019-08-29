mod lexer;
use lexer::{ Lexer, Token };

extern crate spinners;
use spinners::{ Spinner, Spinners };
use std::thread::sleep;
use std::time::Duration;

static SOURCE: &'static str = r#"
import Console from "console"
import Env from "env"

fn Main(): symbol {
    // This is a comment
    let mut array = [i32; 3] { it }
    let some_symbol = 6 / 9 * ( 8 + 1 )
    let args = Env::GetArgs()
}

box Token {
    pos: u32
    type: symbol {
        :float(f64)
        :ident(string)
        :integer(u64)
    }
    fc (pos, type)
}

let token = Token(12, Token::type:float(32.1))

let printable = switch token.type {
    :fn { "fn" }
    :brace_r { "{" }
    :brace_l { "}" }
    :float(value) { "${value}" }
    :integer(value) { "${value}" }
}

export Main
"#;

fn main() {
    let sp = Spinner::new(Spinners::Line2, "Lexing".into());
    sleep(Duration::from_secs(3));
    sp.message("Parsing".into());
    sleep(Duration::from_secs(3));
    sp.message("Compiling".into());
    sleep(Duration::from_secs(3));
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
    sp.stop();
}