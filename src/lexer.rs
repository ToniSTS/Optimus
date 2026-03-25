use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // --- Keywords ---
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
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("print")]
    Print,

    // --- Symbols & Operators ---
    #[token("=")]
    Assign,
    #[token("==")]
    Equals,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token("!=")]
    NotEquals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token(";")]
    Semicolon,
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

    // --- Dynamic Values ---

    // Identifiers (variable names)
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Integers (e.g., 42)
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),

    // Floats (e.g., 3.14 or -0.5)
    // This regex looks for digits, a dot, and more digits
    #[regex(r"[0-9]*\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(String),

    // This regex looks for a quote, any characters that aren't a quote, and a closing quote
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string() // This strips the outer quotes " "
    })]
    String(String),
}
