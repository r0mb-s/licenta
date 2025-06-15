use std::collections::{HashMap, HashSet};

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct FiniteStateMachine {
    cur_state: i8,
    segment: Vec<Token>,
    state_transitions: HashMap<(i8, TokenType), i8>,
    final_states: HashSet<i8>,
}

impl FiniteStateMachine {
    pub fn new() -> Self {
        FiniteStateMachine {
            cur_state: 0,
            segment: Vec::new(),
            state_transitions: HashMap::from([
                // Expressions
                ((0, TokenType::OpenBracket), 1),
                ((0, TokenType::IntLiteral), 3),
                ((0, TokenType::Variable), 3),
                ((1, TokenType::OpenBracket), 1),
                ((1, TokenType::IntLiteral), 2),
                ((1, TokenType::Variable), 2),
                ((2, TokenType::CloseBracket), 3),
                ((2, TokenType::BinaryOperator), 1),
                ((2, TokenType::SemiColon), 99),
                ((3, TokenType::CloseBracket), 3),
                ((3, TokenType::BinaryOperator), 1),
                ((3, TokenType::SemiColon), 99),

                ((3, TokenType::AssignmentOperator), 6),
                
                ((2, TokenType::AssignmentOperator), 0),

                // Assignations
                ((0, TokenType::Var), 4),
                ((4, TokenType::Variable), 5),
                ((5, TokenType::AssignmentOperator), 6),
                ((5, TokenType::SemiColon), 99),
                ((6, TokenType::OpenBracket), 7),
                ((6, TokenType::IntLiteral), 9),
                ((6, TokenType::Variable), 9),
                ((7, TokenType::OpenBracket), 7),
                ((7, TokenType::IntLiteral), 8),
                ((7, TokenType::Variable), 8),
                ((8, TokenType::CloseBracket), 9),
                ((8, TokenType::BinaryOperator), 7),
                ((8, TokenType::SemiColon), 99),
                ((9, TokenType::CloseBracket), 9),
                ((9, TokenType::BinaryOperator), 7),
                ((9, TokenType::SemiColon), 99),

                ((5, TokenType::OpenArray), 32),
                ((32, TokenType::IntLiteral), 33),
                ((33, TokenType::CloseArray), 34),
                ((34, TokenType::SemiColon), 99),

                // If / While
                ((0, TokenType::If), 10),
                ((0, TokenType::While), 10),
                //
                ((10, TokenType::OpenBracket), 11),
                ((10, TokenType::IntLiteral), 13),
                ((10, TokenType::Variable), 13),
                ((11, TokenType::OpenBracket), 11),
                ((11, TokenType::IntLiteral), 12),
                ((11, TokenType::Variable), 12),
                ((12, TokenType::CloseBracket), 13),
                ((12, TokenType::BinaryOperator), 11),
                ((12, TokenType::SemiColon), 14),
                ((12, TokenType::OpenArray), 10),
                ((13, TokenType::CloseBracket), 13),
                ((13, TokenType::BinaryOperator), 11),
                ((13, TokenType::SemiColon), 14),
                ((13, TokenType::OpenArray), 10),
                //
                ((14, TokenType::CloseArray), 14),
                ((14, TokenType::SemiColon), 14),
                ((14, TokenType::ComparisonOperator), 15),
                //
                ((15, TokenType::OpenBracket), 16),
                ((15, TokenType::IntLiteral), 18),
                ((15, TokenType::Variable), 18),
                ((16, TokenType::OpenBracket), 16),
                ((16, TokenType::IntLiteral), 17),
                ((16, TokenType::Variable), 17),
                ((17, TokenType::CloseBracket), 18),
                ((17, TokenType::BinaryOperator), 16),
                ((17, TokenType::SemiColon), 99),
                ((17, TokenType::OpenArray), 15),
                ((18, TokenType::CloseBracket), 18),
                ((18, TokenType::BinaryOperator), 16),
                ((18, TokenType::SemiColon), 99),
                ((18, TokenType::OpenArray), 15),
                //
                ((99, TokenType::CloseArray), 99),
                ((99, TokenType::SemiColon), 99),

                // Call function
                ((0, TokenType::Call), 19),
                ((19, TokenType::Variable), 20),
                ((20, TokenType::Colon), 21),
                ((20, TokenType::SemiColon), 99),
                ((21, TokenType::Variable), 22),
                ((22, TokenType::AssignmentOperator), 23),
                ((23, TokenType::IntLiteral), 24),
                ((23, TokenType::Variable), 24),
                ((24, TokenType::SemiColon), 99),
                ((24, TokenType::Comma), 21),

                // Define function
                ((0, TokenType::Func), 25),
                ((25, TokenType::Variable), 26),
                ((26, TokenType::Colon), 27),
                ((26, TokenType::SemiColon), 99),
                ((27, TokenType::Variable), 28),
                ((28, TokenType::Comma), 27),
                ((28, TokenType::SemiColon), 99),

                // Endif, endwhile, endfunc
                ((0, TokenType::EndIf), 29),
                ((0, TokenType::EndWhile), 29),
                ((0, TokenType::Endfunc), 29),
                ((29, TokenType::SemiColon), 99),

                // Print
                ((0, TokenType::Print), 30),
                ((30, TokenType::OpenBracket), 36),
                ((30, TokenType::IntLiteral), 38),
                ((30, TokenType::Variable), 38),
                ((36, TokenType::OpenBracket), 36),
                ((36, TokenType::IntLiteral), 37),
                ((36, TokenType::Variable), 37),
                ((37, TokenType::CloseBracket), 38),
                ((37, TokenType::BinaryOperator), 36),
                ((37, TokenType::SemiColon), 99),
                ((38, TokenType::CloseBracket), 38),
                ((38, TokenType::BinaryOperator), 36),
                ((38, TokenType::SemiColon), 99),
            ]),
            final_states: HashSet::from([99]),
        }
    }

    pub fn step(&mut self, token: Token) -> (Option<Vec<Token>>, i8) {
        if !self
            .state_transitions
            .contains_key(&(self.cur_state, token.ttype))
        {
            println!("Failed on node: {} with token {:?}", self.cur_state, token.ttype);
            return (None, -1);
            
        }

        self.cur_state = self.state_transitions[&(self.cur_state, token.ttype)];
        self.segment.push(token);

        if self.final_states.contains(&self.cur_state) {
            self.segment.pop();
            let temp_stack: Option<Vec<Token>> = Some(self.segment.clone());
            self.segment.clear();
            let final_state: i8 = self.cur_state;
            self.cur_state = 0;
            return (temp_stack, final_state);
        }

        (None, self.cur_state)
    }
}
