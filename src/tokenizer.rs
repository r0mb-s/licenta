use crate::token::{Token, TokenType};

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    _body: &'a str,
    iterator: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(body: &'a str) -> Self {
        Tokenizer {
            _body: body,
            iterator: body.chars().peekable(),
        }
    }

    pub fn is_done(&mut self) -> bool {
        self.iterator.peek().is_none()
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut idk: String = Default::default();

        let stop_chars: Vec<char> =
            vec![';', '(', ')', '=', '+', '-', '*', '/', '^', '<', '>', '!', ',', ':'];

        while let Some(ch) = self.iterator.peek() {
            if (*ch).is_whitespace() || stop_chars.contains(ch) {
                break;
            }

            let test: char = self.iterator.next()?;
            idk.push(test);
        }

        while let Some(ch) = self.iterator.peek() {
            if (*ch).is_whitespace() {
                self.iterator.next();
            } else if idk.is_empty() && stop_chars.contains(ch) {
                let character: char = *ch;
                self.iterator.next();
                let (ttype, tvalue): (TokenType, Option<String>) = match character {
                    ';' => (TokenType::SemiColon, Some(';'.to_string())),
                    ',' => (TokenType::Comma, Some(','.to_string())),
                    ':' => (TokenType::Colon, Some(':'.to_string())),
                    '(' => (TokenType::OpenBracket, Some('('.to_string())),
                    ')' => (TokenType::CloseBracket, Some(')'.to_string())),
                    '+' | '-' | '*' | '/' | '^' => {
                        (TokenType::BinaryOperator, Some(character.to_string()))
                    }
                    '<' | '>' => (TokenType::ComparisonOperator, Some(character.to_string())),
                    '=' => (TokenType::AssignmentOperator, Some('='.to_string())),
                    _ => (TokenType::Error, None),
                };
                return Some(Token::new(ttype, tvalue));
            } else {
                break;
            }
        }

        if !idk.is_empty() {
            let (mut ttype, mut tvalue): (TokenType, Option<String>) = match idk.as_str() {
                "return" => (TokenType::KeyWord, None),
                "var" => (TokenType::Var, None),
                "if" => (TokenType::If, None),
                "endif" => (TokenType::EndIf, None),
                "while" => (TokenType::While, None),
                "endwhile" => (TokenType::EndWhile, None),
                "func" => (TokenType::Func, None),
                "endfunc" => (TokenType::Endfunc, None),
                "call" => (TokenType::Call, None),
                "print" => (TokenType::Print, None),
                _ => (TokenType::Error, None),
            };

            if ttype == TokenType::Error {
                if idk.as_str().parse::<u8>().is_ok() {
                    ttype = TokenType::IntLiteral;
                    tvalue = Some(idk.clone());
                } else {
                    ttype = TokenType::Variable;
                    tvalue = Some(idk.clone());
                }
            }

            return Some(Token::new(ttype, tvalue));
        } else {
            return None;
        }
    }
    pub fn fix_comparison_operators(tokens: &mut Vec<Token>) {
        for index in 0..tokens.len() - 1 {
            if tokens[index].ttype == TokenType::AssignmentOperator {
                if index + 1 < tokens.len()
                    && (tokens[index + 1].ttype == TokenType::ComparisonOperator
                        || tokens[index + 1].ttype == TokenType::AssignmentOperator)
                {
                    let mut aux: String = String::new();

                    if let Some(val) = tokens[index].value.as_ref() {
                        aux.push_str(val);
                    }
                    if let Some(val) = tokens[index + 1].value.as_ref() {
                        aux.push_str(val);
                    }

                    tokens[index].value = Some(aux);
                    tokens[index].ttype = TokenType::ComparisonOperator;

                    tokens.remove(index + 1);
                }
            }
        }
    }
}
