use std::usize;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Assignment {
        var_name: String,
        expr: Box<ASTNode>,
    },
    BinaryOperation {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    IfOperation {
        first_half: Box<ASTNode>,
        comparison_op: String,
        second_half: Box<ASTNode>,
    },
    EndIf,
    WhileOperation {
        first_half: Box<ASTNode>,
        comparison_op: String,
        second_half: Box<ASTNode>,
    },
    EndWhile,
    FunctionDef {
        name: String,
        parameters: Option<Vec<String>>,
    },
    EndFunctionDef,
    FuntionCall {
        name: String,
        parameters: Option<Vec<(String, String)>>,
    },
    Print(Box<ASTNode>),
    Literal(String),
    Variable(String),
    Start,
    End,
    Error,
}

#[derive(Debug)]
pub struct AST {
    nodes: Vec<ASTNode>,
}

impl AST {
    pub fn new(nodes: Vec<ASTNode>) -> Self {
        AST { nodes }
    }

    pub fn add_node(&mut self, node: ASTNode) {
        self.nodes.push(node);
    }

    pub fn get_node(&self, index: usize) -> Option<&ASTNode> {
        self.nodes.get(index)
    }

    pub fn get_nodes(&self) -> Vec<ASTNode> {
        self.nodes.clone()
    }

    pub fn print_nodes(&self) {
        for node in &self.nodes {
            Self::print_node(node);
        }
    }
    pub fn print_node(node: &ASTNode) {
        match node {
            ASTNode::BinaryOperation { op, left, right } => {
                println!("Binary Operation: {}", op,);
                Self::print_node(left);
                Self::print_node(right);
            }
            ASTNode::IfOperation {
                first_half,
                comparison_op,
                second_half,
            } => {
                println!("It's and If");
                Self::print_node(first_half);
                println!("Operator: {} ", comparison_op,);
                Self::print_node(second_half);
            }
            ASTNode::WhileOperation {
                first_half,
                comparison_op,
                second_half,
            } => {
                println!(
                    "It's a while: {:?} {} {:?}",
                    first_half, comparison_op, second_half
                );
            }
            ASTNode::Assignment { var_name, expr } => {
                println!("It's an Assignment: {}", var_name);
                Self::print_node(expr)
            }
            ASTNode::Literal(value) => {
                println!("It's a Literal: {}", value);
            }
            ASTNode::Variable(var_name) => {
                println!("It's a Variable: {}", var_name);
            }
            ASTNode::Start => {
                println!("It's a Start node");
            }
            ASTNode::End => {
                println!("It's an End node");
            }
            ASTNode::Error => {
                println!("It's an Error node");
            }
            ASTNode::EndIf => {
                println!("It's an EndIf node");
            }
            ASTNode::EndWhile => {
                println!("It's an EndWhile node");
            }
            ASTNode::FunctionDef { name, parameters } => {
                println!("It's a function with the name \'{}\'", name);
                print!("With parameters:");
                if let Some(params) = &parameters {
                    for param in params {
                        print!(" {},", param);
                    }
                }
                println!();
            }
            ASTNode::EndFunctionDef => {
                println!("It's an EndFunctionDef node");
            }
            ASTNode::FuntionCall { name, parameters } => {
                println!("It's a call function with the name \'{}\'", name);
                print!("With parameters:");
                if let Some(params) = &parameters {
                    for param in params {
                        print!(" {} = {},", param.0, param.1);
                    }
                }
                println!();
            }
            ASTNode::Print(astnode) => {
                println!("It's a print node");
                Self::print_node(astnode);
            },
        }
    }
}
