
use super::super::lexer::Opr;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Import {
        main: Option<String>,
        inners: Vec<String>,
        from: String
    },

    Export {
        main: Option<String>,
        inners: Vec<String>
    },

    Scope {
        exprs: Vec<Expr>
    },

    Assignment {
        changeable: bool,
        mutable: bool,
        name: String,
        expr: Box<Expr>
    },

    Binary {
        op: Opr,
        left: Box<Expr>,
        right: Box<Expr>
    },

    Call {
        fn_name: String,
        args: Vec<Expr>
    },

    Conditional {
        cond: Box<Expr>,
        consequence: Box<Expr>,
        alternative: Box<Expr>
    },

    Switch {
        cond: Box<Expr>,
        paths: Vec<(Expr, Expr)>
    },

    Function {
        name: Option<String>,
        expr: Box<Expr>
    },

    Integer(u64),
    Float(f64),
    Variable(String),
    Str(String)
}