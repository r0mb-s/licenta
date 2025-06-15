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
    NegationOperator,
    SemiColon,
    Colon,
    Comma,
    OpenBracket,
    CloseBracket,
    OpenArray,
    CloseArray,
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
