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
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let chars = self.chars.deref_mut();
        let src = self.input;
        let mut pos = skip_whitespace(self.pos, chars);
        
        let start = pos;
        let next = chars.next();

        if next.is_none() {
            return None;
        }

        pos += 1;
        
        let result = match next.unwrap() {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '\n' => Token::LineBreak,
            ':' => do_colon(&src, start, &mut pos, chars),
            '/' => {
                let ch = match chars.peek() {
                    Some(ch) => *ch,
                    None => return None
                };
                match ch {
                    '/' => {
                        loop {
                            let ch = match chars.peek() {
                                Some(ch) => *ch,
                                None => return None
                            };

                            if ch == '\n' {
                                break;
                            }
                            chars.next();
                            pos += 1;
                        }
                        Token::Comment(src[start + 2..pos].to_string())
                    }
                    _ => Token::Op(Opr::Div)
                }
            },

            '.' | '0' ..= '9' => {
                // Parse number literal
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return None
                    };

                    // Parse float.
                    if ch != '.' && !ch.is_digit(16) {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                Token::Number(src[start..pos].parse().unwrap())
            },

            'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                // Parse identifier
                loop {
                    let ch = match chars.peek() {
                        Some(ch) => *ch,
                        None => return None
                    };

                    // A word-like identifier only contains underscores and alphanumeric characters.
                    if ch != '_' && !ch.is_alphanumeric() {
                        break;
                    }

                    chars.next();
                    pos += 1;
                }

                match &src[start..pos] {
                    "extern" => Token::Extern,
                    "if" => Token::If,
                    "then" => Token::Then,
                    "else" => Token::Else,
                    "fn" => Token::Fun,
                    "and" => Token::Op(Opr::And),
                    "or" => Token::Op(Opr::Or),
                    "xor" => Token::Op(Opr::Xor),
                    "let" => Token::Let,
                    "mut" => Token::Mut,
                    "unary" => Token::Unary,
                    "binary" => Token::Binary,
                    "var" => Token::Var,

                    ident => Token::Ident(ident.to_string())
                }
            },

            op => {
                // Parse operator
                let ch = match chars.peek() {
                    Some(ch) => *ch,
                    None => return None
                };

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
                            _ => panic!(op),
                        }
                    }
                };

                if do_next {
                    chars.next();
                    pos += 1;
                }

                Token::Op(op)
            }
        };
        self.pos = pos;
        return Some(result);
    }
}

fn skip_whitespace(pos: usize, chars: &mut Peekable<Chars>) -> usize {
    let mut pos = pos;
    loop {
        {
            let ch = match chars.peek() {
                Some(ch) => *ch,
                None => return pos
            };
            if ch != ' ' {
                break;
            }
        }
        chars.next();
        pos += 1;
    }
    pos
}

fn do_colon(src: &str, start: usize, pos: &mut usize, chars: &mut Peekable<Chars>) -> Token {
    {
        loop {
            let ch = match chars.peek() {
                Some(ch) => *ch,
                None => return Token::Colon
            };

            if ch != '_' && !ch.is_alphanumeric() {
                break;
            }
            chars.next();
            *pos += 1;
        }
        if start + 1 == *pos {
            Token::Colon
        } else {
            Token::Symbol(src[start + 1..*pos].to_string())
        }
    }
}