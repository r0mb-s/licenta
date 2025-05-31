#[derive(Debug, PartialEq, Clone, Hash, Eq, Copy)]
pub enum TokenType {
    KeyWord,
    Var,
    If,
    EndIf,
    While,
    EndWhile,
    IntLiteral,
    AssignmentOperator,
    BinaryOperator,
    ComparisonOperator,
    SemiColon,
    Colon,
    Comma,
    OpenBracket,
    CloseBracket,
    Variable,
    Func,
    Endfunc,
    Call,
    Print,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: Option<String>,
}

impl Token {
    pub fn new(ttype: TokenType, value: Option<String>) -> Self {
        Token{ ttype, value }
    }
}
