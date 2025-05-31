use std::collections::HashMap;
use std::iter::Peekable;
use std::u8;
use std::vec::IntoIter;

use crate::ast::ASTNode;
use crate::fsm::FiniteStateMachine;
use crate::symbol_table::{self, SymbolTable};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Peekable<IntoIter<Token>>,
    automata: FiniteStateMachine,
    symbol_table: &'a mut SymbolTable,
}

impl<'a> Parser<'a> {
    pub fn new(token_vec: Vec<Token>, symbol_table: &'a mut SymbolTable) -> Self {
        symbol_table.reset_level();
        Parser {
            tokens: token_vec.into_iter().peekable(),
            automata: FiniteStateMachine::new(),
            symbol_table,
        }
    }

    pub fn parse(&mut self) -> ASTNode {
        while let Some(_) = self.tokens.peek() {
            let response = self.automata.step(self.tokens.next().unwrap());
            if response.1 == -1 {
                return ASTNode::Error;
            } else if let Some(segment) = response.0 {
                if segment[0].ttype == TokenType::Var {
                    return Parser::parse_assignment(self, &segment);
                } else if segment[0].ttype == TokenType::If {
                    return Parser::parse_if(self, &segment);
                } else if segment[0].ttype == TokenType::EndIf {
                    self.symbol_table.down();
                    return ASTNode::EndIf;
                } else if segment[0].ttype == TokenType::While {
                    return Parser::parse_while(self, &segment);
                } else if segment[0].ttype == TokenType::EndWhile {
                    self.symbol_table.down();
                    return ASTNode::EndWhile;
                } else if segment[0].ttype == TokenType::Func {
                    return Parser::parse_func_def(&segment);
                } else if segment[0].ttype == TokenType::Endfunc {
                    return ASTNode::EndFunctionDef;
                } else if segment[0].ttype == TokenType::Call {
                    return Parser::parse_func_call(&segment);
                } else if segment[0].ttype == TokenType::Print {
                    return Parser::parse_print(self, &segment);
                }
            }
        }

        ASTNode::End
    }

    fn parse_assignment(&mut self, segment: &Vec<Token>) -> ASTNode {
        if let Some(var_name) = &segment[1].value {
            if segment[2].ttype == TokenType::AssignmentOperator {
                self.symbol_table.add_to_table(var_name.to_string().clone());
                let aux: ASTNode = ASTNode::Assignment {
                    var_name: var_name.to_string(),
                    expr: Box::new(Self::parse_expression(self, &segment[3..].to_vec())),
                };
                return aux;
            }
        }

        ASTNode::Error
    }

    fn parse_if(&mut self, segment: &Vec<Token>) -> ASTNode {
        let mut comp_op_index: usize = 0;

        for i in 1..segment.len() {
            if segment[i].ttype == TokenType::ComparisonOperator {
                comp_op_index = i;
                break;
            }
        }

        let left_part: &Vec<Token> = &segment[1..comp_op_index].to_vec();
        let right_part: &Vec<Token> = &segment[(comp_op_index + 1)..segment.len()].to_vec();

        let left_ast: Box<ASTNode> = Box::new(Self::parse_expression(self, left_part));
        if *left_ast == ASTNode::Error {
            return ASTNode::Error;
        }
        let right_ast: Box<ASTNode> = Box::new(Self::parse_expression(self, right_part));
        if *right_ast == ASTNode::Error {
            return ASTNode::Error;
        }

        self.symbol_table.up();

        ASTNode::IfOperation {
            first_half: left_ast,
            comparison_op: segment[comp_op_index].value.clone().unwrap(),
            second_half: right_ast,
        }
    }

    fn parse_while(&mut self, segment: &Vec<Token>) -> ASTNode {
        let mut comp_op_index: usize = 0;

        for i in 1..segment.len() {
            if segment[i].ttype == TokenType::ComparisonOperator {
                comp_op_index = i;
                break;
            }
        }

        let left_part: &Vec<Token> = &segment[1..comp_op_index].to_vec();
        let right_part: &Vec<Token> = &segment[comp_op_index..segment.len()].to_vec();

        let left_ast: Box<ASTNode> = Box::new(Self::parse_expression(self, left_part));
        if *left_ast == ASTNode::Error {
            return ASTNode::Error;
        }
        let right_ast: Box<ASTNode> = Box::new(Self::parse_expression(self, right_part));
        if *right_ast == ASTNode::Error {
            return ASTNode::Error;
        }

        self.symbol_table.up();

        ASTNode::WhileOperation {
            first_half: left_ast,
            comparison_op: segment[comp_op_index].value.clone().unwrap(),
            second_half: right_ast,
        }
    }

    fn parse_expression(&mut self, segment: &Vec<Token>) -> ASTNode {
        let rpn_tokens: Vec<Token> = Parser::convert_to_rpn(&segment);
        let mut intermediate_stack: Vec<ASTNode> = vec![];

        for token in rpn_tokens {
            if token.ttype == TokenType::IntLiteral {
                intermediate_stack.push(ASTNode::Literal(
                    token.value.expect("Somethin wrong with token!"),
                ));
            } else if token.ttype == TokenType::Variable {
                if !self.symbol_table.check_table(token.value.clone().unwrap()) {
                    return ASTNode::Error;
                }
                intermediate_stack.push(ASTNode::Variable(
                    token.value.expect("Somethin wrong with token!"),
                ));
            } else if token.ttype == TokenType::BinaryOperator {
                if let (Some(right), Some(left)) =
                    (intermediate_stack.pop(), intermediate_stack.pop())
                {
                    let aux: ASTNode = ASTNode::BinaryOperation {
                        op: token.value.expect("Wrong with token!"),
                        left: Box::new(left.clone()),
                        right: Box::new(right.clone()),
                    };
                    intermediate_stack.push(aux);
                } else {
                    return ASTNode::Error;
                }
            }
        }

        if intermediate_stack.len() != 1 {
            return ASTNode::Error;
        } else {
            let aux: ASTNode = intermediate_stack.pop().expect("Shouldn't be empty!");
            return aux;
        }
    }

    fn convert_to_rpn(tokens: &Vec<Token>) -> Vec<Token> {
        let precedence: HashMap<String, u8> = HashMap::from([
            ("(".to_string(), 10),
            (")".to_string(), 10),
            ("+".to_string(), 1),
            ("-".to_string(), 1),
            ("*".to_string(), 2),
            ("/".to_string(), 2),
            ("^".to_string(), 3),
        ]);

        let mut result: Vec<Token> = vec![];

        let mut operator_stack: Vec<Token> = vec![];

        for token in tokens {
            if token.ttype == TokenType::BinaryOperator {
                while let Some(top_of_stack) = operator_stack.last() {
                    let token_val = token.value.as_deref().expect("Weird token!");
                    let top_stack_val = top_of_stack.value.as_deref().expect("Weird token 2!");

                    if precedence[token_val] < precedence[top_stack_val] && top_stack_val != "(" {
                        result.push(
                            operator_stack
                                .pop()
                                .expect("something very wrong with operator stack!"),
                        );
                    } else {
                        break;
                    }
                }

                operator_stack.push(token.clone());
            } else if token.ttype == TokenType::IntLiteral || token.ttype == TokenType::Variable {
                result.push(token.clone());
            } else if token.ttype == TokenType::OpenBracket {
                operator_stack.push(token.clone());
            } else if token.ttype == TokenType::CloseBracket {
                while let Some(top_of_stack) = operator_stack.last() {
                    if top_of_stack.ttype != TokenType::OpenBracket {
                        result.push(
                            operator_stack
                                .pop()
                                .expect("Something wrong with operator stack!"),
                        );
                    } else {
                        operator_stack.pop().expect("IDK");
                        break;
                    }
                }
            }
        }

        while let Some(_) = operator_stack.last() {
            result.push(
                operator_stack
                    .pop()
                    .expect("Something very wrong with stack 2!"),
            );
        }

        result
    }

    fn parse_func_def(segment: &Vec<Token>) -> ASTNode {
        if segment.len() == 3 {
            return ASTNode::FunctionDef {
                name: segment[1].value.clone().expect("Something wrong with func"),
                parameters: None,
            };
        } else {
            let mut params: Option<Vec<String>> = Some(Vec::new());
            for i in 3..segment.len() {
                if segment[i].ttype == TokenType::Comma {
                    continue;
                }
                if let Some(ref mut vec) = params {
                    vec.push(
                        segment[i]
                            .value
                            .clone()
                            .expect("Something wrong with func def"),
                    );
                }
            }
            return ASTNode::FunctionDef {
                name: segment[1].value.clone().expect("Something wrong with func"),
                parameters: params,
            };
        }
    }

    fn parse_func_call(segment: &Vec<Token>) -> ASTNode {
        if segment.len() == 2 {
            return ASTNode::FuntionCall {
                name: segment[1].value.clone().expect("Something wrong with func"),
                parameters: None,
            };
        } else {
            let mut params: Option<Vec<(String, String)>> = Some(Vec::new());
            for i in (3..segment.len()).step_by(4) {
                if let Some(ref mut vec) = params {
                    if segment.len() - 4 == i {
                        break;
                    } else {
                        vec.push((
                            segment[i]
                                .value
                                .clone()
                                .expect("Something wrong with func def"),
                            segment[i + 2]
                                .value
                                .clone()
                                .expect("Something wrong with func def"),
                        ));
                    }
                }
            }
            return ASTNode::FuntionCall {
                name: segment[1].value.clone().expect("Something wrong with func"),
                parameters: params,
            };
        }
    }

    fn parse_print(&mut self, segment: &Vec<Token>) -> ASTNode {
        if segment.len() == 2 {
            return ASTNode::Print(segment[1].value.clone().unwrap());
        }

        ASTNode::Error
    }
}
