use super::lexer::Token;

pub mod expr;
pub use expr::Expr;

pub struct Parser {
    tokens: Vec<(Token, usize)>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<(Token, usize)>) -> Self {
        Parser { tokens: tokens }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        match self.current()? {
            Fun => self.parse_function(),
            _ => self.parse_toplevel_expr(),
        }
    }

    fn curr(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn current(&self) -> Result<Token, &'static str> {
        if self.pos >= self.tokens.len() {
            Err("Unexpected end of file.")
        } else {
            Ok(self.tokens[self.pos].clone())
        }
    }

    fn advance(&mut self) -> Result<(), &'static str> {
        let npos = self.pos + 1;

        self.pos = npos;

        if npos < self.tokens.len() {
            Ok(())
        } else {
            Err("Unexpected end of file.")
        }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
