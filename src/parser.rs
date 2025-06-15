use std::collections::HashMap;
use std::env::var;
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
                } else if segment[0].ttype == TokenType::Variable {
                    return Parser::parse_assignment(self, &segment);
                }
            }
        }

        ASTNode::End
    }

    fn parse_assignment(&mut self, segment: &Vec<Token>) -> ASTNode {
        if segment[0].ttype == TokenType::Var {
            if let Some(var_name) = &segment[1].value {
                if segment.len() > 2 {
                    if segment[2].ttype == TokenType::OpenArray {
                        let mut end: usize = 0;
                        for tk in segment {
                            end += 1;
                            if tk.ttype == TokenType::CloseArray {
                                break;
                            }
                        }

                        // let aux: ASTNode = ASTNode::Array {
                        //     arr_name: var_name.to_string(),
                        //     index: Box::new(Self::parse_expression(
                        //         self,
                        //         &segment[3..end].to_vec(),
                        //     )),
                        // };

                        let aux2: ASTNode = ASTNode::ArrayDeclaration {
                            arr_name: var_name.to_string(),
                            size: Box::new(Self::parse_expression(self, &segment[3..end].to_vec())),
                        };

                        if let ASTNode::ArrayDeclaration { arr_name: _, size } = aux2.clone() {
                            match *size {
                                ASTNode::Literal(val) => self.symbol_table.add_to_table(var_name.to_string().clone(), "arr".to_string(), val.parse().unwrap()),
                                ASTNode::Variable(var) => print!(""),
                                _ => println!("Index is another type of ASTNode"),
                            }
                        }

                        

                        return aux2;
                    } else if segment[2].ttype == TokenType::AssignmentOperator {
                        self.symbol_table
                            .add_to_table(var_name.to_string().clone(), "int".to_string(), 0);
                        let aux: ASTNode = ASTNode::Assignment {
                            var_name: var_name.to_string(),
                            expr: Box::new(Self::parse_expression(self, &segment[3..].to_vec())),
                        };
                        return aux;
                    }
                } else if segment.len() == 2 {
                    self.symbol_table
                        .add_to_table(var_name.to_string().clone(), "int".to_string(), 0);
                    let aux: ASTNode = ASTNode::Assignment {
                        var_name: var_name.to_string(),
                        expr: Box::new(ASTNode::Literal("0".to_string())),
                    };
                    return aux;
                }
            }
        } else {
            if let Some(var_name) = &segment[0].value {
                if segment[1].ttype == TokenType::AssignmentOperator {
                    let aux: ASTNode = ASTNode::Assignment {
                        var_name: var_name.to_string(),
                        expr: Box::new(Self::parse_expression(self, &segment[2..].to_vec())),
                    };
                    return aux;
                }

                let mut eq_index = -1;
                for tok in 0..segment.len() {
                    if segment[tok].ttype == TokenType::AssignmentOperator {
                        eq_index = tok as i32;
                        break;
                    }
                }

                if eq_index != -1 {
                    // let aux: ASTNode = ASTNode::Assignment {
                    //     var_name: Box::new(ASTNode::Array {
                    //         arr_name: var_name.to_string(),
                    //         index: Box::new(Self::parse_expression(
                    //             self,
                    //             &segment[2..eq_index as usize].to_vec(),
                    //         )),
                    //     }),
                    //     expr: Box::new(Self::parse_expression(
                    //         self,
                    //         &segment[eq_index as usize + 1..].to_vec(),
                    //     )),
                    // };
                    let aux: ASTNode = ASTNode::ArrayAssignment {
                        arr_name: var_name.to_string(),
                        position: Box::new(Self::parse_expression(self, &segment[2..eq_index as usize].to_vec())),
                        value: Box::new(Self::parse_expression(self, &segment[eq_index as usize + 1..].to_vec(),))
                    };
                    return aux;
                }
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
            content: Vec::new(),
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
            println!("CCCCCCCCCCCCCCCCCCCCCCCCCC");
            return ASTNode::Error;
        }
        let right_ast: Box<ASTNode> = Box::new(Self::parse_expression(self, right_part));
        if *right_ast == ASTNode::Error {
            println!("DDDDDDDDDDDDDDDDDDDDDDDDDDD");
            return ASTNode::Error;
        }

        self.symbol_table.up();

        ASTNode::WhileOperation {
            first_half: left_ast,
            comparison_op: segment[comp_op_index].value.clone().unwrap(),
            second_half: right_ast,
            content: Vec::new(),
        }
    }

    //

    fn parse_expression(&mut self, segment: &Vec<Token>) -> ASTNode {
        let rpn_tokens: Vec<Token> = Parser::convert_to_rpn(&segment);
        println!("RPN: {:?}", rpn_tokens);
        let mut intermediate_stack: Vec<ASTNode> = vec![];

        let mut i = 0;
        while i < rpn_tokens.len() {
            let token = &rpn_tokens[i];

            match token.ttype {
                TokenType::IntLiteral => {
                    intermediate_stack.push(ASTNode::Literal(
                        token.value.clone().expect("Missing literal value"),
                    ));
                    i += 1;
                }
                TokenType::Variable => {
                    let var_name = token.value.clone().expect("Missing variable name");
                    if let Some(symb) = self.symbol_table.check_table(var_name.clone()) {
                        if symb.vtype == "int" {
                            intermediate_stack.push(ASTNode::Variable(var_name.clone()));
                        } else if symb.vtype == "arr" {
                            return ASTNode::Array {
                                arr_name: symb.vname.clone(),
                                index: Box::new(Self::parse_expression(
                                    self,
                                    &segment[2..].to_vec(),
                                )),
                            };
                        }
                    }
                    i += 1;

                    // Check if next token is OpenArray
                    // if i + 1 < rpn_tokens.len() && rpn_tokens[i + 1].ttype == TokenType::OpenArray {
                    //     // Find matching CloseArray
                    //     let mut depth = 1;
                    //     let mut j = i + 2;

                    //     while j < rpn_tokens.len() && depth > 0 {
                    //         match rpn_tokens[j].ttype {
                    //             TokenType::OpenArray => depth += 1,
                    //             TokenType::CloseArray => depth -= 1,
                    //             _ => {}
                    //         }
                    //         j += 1;
                    //     }

                    //     if depth != 0 {
                    //         return ASTNode::Error; // Mismatched brackets
                    //     }

                    //     let index_tokens = &rpn_tokens[i + 2..j - 1];
                    //     let index_expr = self.parse_expression(&index_tokens.to_vec());

                    //     intermediate_stack.push(ASTNode::Array {
                    //         arr_name: var_name,
                    //         index: Box::new(index_expr),
                    //     });

                    //     i = j; // move past ]
                    // } else {
                    //     intermediate_stack.push(ASTNode::Variable(var_name));
                    //     i += 1;
                    // }
                }
                TokenType::BinaryOperator => {
                    if let (Some(right), Some(left)) =
                        (intermediate_stack.pop(), intermediate_stack.pop())
                    {
                        let aux = ASTNode::BinaryOperation {
                            op: token.value.clone().expect("Missing operator value"),
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                        intermediate_stack.push(aux);
                    } else {
                        return ASTNode::Error;
                    }
                    i += 1;
                }
                _ => {
                    return ASTNode::Error; // Unexpected token
                }
            }
        }

        if intermediate_stack.len() != 1 {
            ASTNode::Error
        } else {
            intermediate_stack.pop().unwrap()
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
        if segment.len() >= 2 {
            return ASTNode::Print(Box::new(Self::parse_expression(
                self,
                &segment[1..].to_vec(),
            )));
        }

        ASTNode::Error
    }
}
