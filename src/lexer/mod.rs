use std::iter::Peekable;
use std::str::Chars;
use std::ops::DerefMut;

pub mod token;
pub use token::{ Token, Opr };

pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input: input, chars: Box::new(input.chars().peekable()), pos: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {

    type Item = (Token, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let chars = self.chars.deref_mut();
        let src = self.input;
        let mut pos = skip_whitespace(self.pos, chars)?;
        let start = pos;
        pos += 1;
        let result: Option<Token> = match chars.next()? {
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semicolon),
            '\n' => Some(Token::LineBreak),
            '"' => do_quote(&src, start, &mut pos, chars),
            ':' => do_colon(&src, start, &mut pos, chars),
            '/' => do_slash(&src, start, &mut pos, chars),
            '.' | '0' ..= '9' => do_number(&src, start, &mut pos, chars),
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => do_ident(&src, start, &mut pos, chars),
            op => do_op(op, &mut pos, chars),
        };
        self.pos = pos;
        Some((result?, pos))
    }
}

fn skip_whitespace(pos: usize, chars: &mut Peekable<Chars>) -> Option<usize> {
    let mut pos = pos;
    loop {
        let ch = *chars.peek()?;
        if ch != ' ' {
            break;
        }
        chars.next();
        pos += 1;
    }
    Some(pos)
}

fn do_quote(src: &str, start: usize, pos: &mut usize, chars: &mut Peekable<Chars>) -> Option<Token> {
    loop {
        let ch = *chars.peek()?;
        if ch == '\\' {
            *pos += 1;
            chars.next();
        }
        if ch == '"' {
            *pos += 1;
            chars.next();
            break Some(Token::Str(src[start + 1..*pos - 1].to_string()));
        }
        chars.next();
        *pos += 1;
    }
}

fn do_colon(src: &str, start: usize, pos: &mut usize, chars: &mut Peekable<Chars>) -> Option<Token> {
    loop {
        let ch = *chars.peek()?;
        if ch != '_' && !ch.is_alphanumeric() {
            break;
        }
        chars.next();
        *pos += 1;
    }
    if start + 1 == *pos {
        Some(Token::Colon)
    } else {
        Some(Token::Symbol(src[start + 1..*pos].to_string()))
    }
}

fn do_slash(src: &str, start: usize, pos: &mut usize, chars: &mut Peekable<Chars>) -> Option<Token> {
    let ch = *chars.peek()?;
    *pos += 1;
    chars.next();
    match ch {
        '/' => loop {
            let ch = *chars.peek()?;
            if ch == '\n' {
                break Some(Token::Comment(src[start + 2..*pos].to_string()));
            }
            chars.next();
            *pos += 1;
        },
        '*' => loop {
            let ch = *chars.peek()?;
            if ch == '*' {
                chars.next();
                *pos += 1;
                let ch = *chars.peek()?;
                if ch == '/' {
                    break Some(Token::Comment(src[start + 2..*pos-1].to_string()));
                }
            }
            chars.next();
            *pos += 1;
        },
        _ => Some(Token::Op(Opr::Div))
    }
}

fn do_number(src: &str, start: usize, pos: &mut usize, chars: &mut Peekable<Chars>) -> Option<Token> {
    let mut is_integer = true;
    loop {
        let ch = *chars.peek()?;
        if ch != '.' && !ch.is_digit(16) {
            break if is_integer {
                Some(Token::Integer(src[start..*pos].parse::<u64>().unwrap()))
            } else {
                Some(Token::Float(src[start..*pos].parse::<f64>().unwrap()))
            };
        }
        if ch == '.' {
            is_integer = false;
        }
        chars.next();
        *pos += 1;
    }
}

fn do_ident(src: &str, start: usize, pos: &mut usize, chars: &mut Peekable<Chars>) -> Option<Token> {
    let ident = loop {
        let ch = *chars.peek()?;
        if ch != '_' && !ch.is_alphanumeric() {
            break &src[start..*pos];
        }
        chars.next();
        *pos += 1;
    };

    let token = match ident {
        "import" => Token::Import,
        "from" => Token::Fromm,
        "export" => Token::Export,
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "fn" => Token::Fun,
        "and" => Token::Op(Opr::And),
        "or" => Token::Op(Opr::Or),
        "xor" => Token::Op(Opr::Xor),
        "let" => Token::Let,
        "mut" => Token::Mut,
        "var" => Token::Var,

        ident => Token::Ident(ident.to_string())
    };
    Some(token)
}

fn do_op(op: char, pos: &mut usize, chars: &mut Peekable<Chars>) -> Option<Token> {
    let ch = *chars.peek()?;
    let mut do_next = true;
    let op = match ch {
        '=' => match op {
            '=' => Opr::Equ,
            '!' => Opr::Neq,
            '>' => Opr::Geq,
            '<' => Opr::Leq,
            '*' => Opr::AssignMul,
            '/' => Opr::AssignDiv,
            '%' => Opr::AssignMod,
            '+' => Opr::AssignAdd,
            '-' => Opr::AssignSub,
            _ => Opr::Assign
        },
        '&' if op == '&' => Opr::And,
        '>' if op == '-' => Opr::Arrow,
        '|' if op == '|' => Opr::Or,
        _ => {
            do_next = false;
            match op {
                '=' => Opr::Assign,
                '>' => Opr::Gtr,
                '<' => Opr::Les,
                '^' => Opr::Exp,
                '*' => Opr::Mul,
                '/' => Opr::Div,
                '%' => Opr::Mod,
                '+' => Opr::Add,
                '-' => Opr::Sub,
                '!' => Opr::Not,
                '?' => Opr::Drf,
                '|' => Opr::Pipe,
                _ => {
                    return None
                }
            }
        }
    };
    if do_next {
        chars.next();
        *pos += 1;
    }
    Some(Token::Op(op))
}