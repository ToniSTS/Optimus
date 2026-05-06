use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(skip r"//.*")]
pub enum Token {
    // Keywords
    #[token("mut")]
    Mut,
    #[token("ref")]
    Ref,
    #[token("int")]
    IntType,
    #[token("float")]
    FloatType,
    #[token("bool")]
    BoolType,
    #[token("string")]
    StringType,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("class")]
    Class,
    #[token("module")]
    Module,
    #[token("import")]
    Import,
    #[token("new")]
    New,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,
    #[token("print")]
    Print,

    // Symbols & Operators
    #[token("==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("=")]
    Assign,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("!")]
    Bang,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // Dynamic values
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),

    #[regex(r"[0-9]*\.[0-9]+", |lex| lex.slice().to_string())]
    Float(String),

    #[regex(r#"\"([^\"\\]|\\.)*\""#, |lex| {
        let s = lex.slice();
        s[1..s.len() - 1].to_string()
    })]
    String(String),
}
