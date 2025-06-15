use std::usize;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Assignment {
        var_name: String,
        expr: Box<ASTNode>,
    },
    ArrayDeclaration {
        arr_name: String,
        size: Box<ASTNode>,
    },
    ArrayAssignment {
        arr_name: String,
        position: Box<ASTNode>,
        value: Box<ASTNode>,
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
        content: Vec<Box<ASTNode>>,
    },
    EndIf,
    WhileOperation {
        first_half: Box<ASTNode>,
        comparison_op: String,
        second_half: Box<ASTNode>,
        content: Vec<Box<ASTNode>>,
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
    Array {
        arr_name: String,
        index: Box<ASTNode>,
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
    pub nodes: Vec<ASTNode>,
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

    pub fn transform_ast(&mut self){
        let mut new_nodes = Vec::new();
        let mut iter = self.nodes.clone().into_iter();

        while let Some(node) = iter.next() {
            match node {
                ASTNode::WhileOperation { first_half, comparison_op, second_half, .. } => {
                    let mut content = Vec::new();

                    while let Some(next_node) = iter.next() {
                        match next_node {
                            ASTNode::EndWhile => break,
                            _ => content.push(Box::new(next_node)),
                        }
                    }

                    new_nodes.push(ASTNode::WhileOperation { first_half, comparison_op, second_half, content });
                }
                ASTNode::IfOperation { first_half, comparison_op, second_half, .. } => {
                    let mut content = Vec::new();

                    while let Some(next_node) = iter.next() {
                        match next_node {
                            ASTNode::EndIf => break,
                            _ => content.push(Box::new(next_node)),
                        }
                    }

                    new_nodes.push(ASTNode::IfOperation { first_half, comparison_op, second_half, content });
                }
                other => new_nodes.push(other),
            }
        }
        self.nodes = new_nodes;
    }

     pub fn bttr_transform_ast(&mut self) {
        fn collect_block<I>(iter: &mut I, end_marker: ASTNode) -> Vec<Box<ASTNode>>
        where
            I: Iterator<Item = ASTNode>,
        {
            let mut content = Vec::new();

            while let Some(node) = iter.next() {
                match node {
                    ASTNode::IfOperation { first_half, comparison_op, second_half, .. } => {
                        let nested_content = collect_block(iter, ASTNode::EndIf);
                        content.push(Box::new(ASTNode::IfOperation {
                            first_half,
                            comparison_op,
                            second_half,
                            content: nested_content,
                        }));
                    }
                    ASTNode::WhileOperation { first_half, comparison_op, second_half, .. } => {
                        let nested_content = collect_block(iter, ASTNode::EndWhile);
                        content.push(Box::new(ASTNode::WhileOperation {
                            first_half,
                            comparison_op,
                            second_half,
                            content: nested_content,
                        }));
                    }
                    node if node == end_marker => break,
                    other => content.push(Box::new(other)),
                }
            }

            content
        }

        let mut new_nodes = Vec::new();
        let mut iter = self.nodes.clone().into_iter();

        while let Some(node) = iter.next() {
            match node {
                ASTNode::IfOperation { first_half, comparison_op, second_half, .. } => {
                    let content = collect_block(&mut iter, ASTNode::EndIf);
                    new_nodes.push(ASTNode::IfOperation {
                        first_half,
                        comparison_op,
                        second_half,
                        content,
                    });
                }
                ASTNode::WhileOperation { first_half, comparison_op, second_half, .. } => {
                    let content = collect_block(&mut iter, ASTNode::EndWhile);
                    new_nodes.push(ASTNode::WhileOperation {
                        first_half,
                        comparison_op,
                        second_half,
                        content,
                    });
                }
                other => new_nodes.push(other),
            }
        }

        self.nodes = new_nodes;
    }
}
