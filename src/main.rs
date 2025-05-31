mod ast;
mod parser;
mod fsm;
mod token;
mod tokenizer;
mod symbol_table;
mod assembler;

use assembler::Assembler;
use ast::{ASTNode, AST};
use parser::Parser;
use symbol_table::SymbolTable;
use token::Token;
use tokenizer::Tokenizer;

fn main() -> std::io::Result<()> {
    if std::env::args().len() != 2 {
        println!("Incorrect usage");
        println!("Correct usage: idk source.idk");
        std::process::exit(101);
    }
    
    let argv: Vec<String> = std::env::args().collect();
    let source_file_path: &str = &argv[1];
    let source_code = std::fs::read_to_string(source_file_path)?;
    let mut tokenizer: Tokenizer;
    let mut tokens: Vec<Token> = Vec::new();
    let mut parser: Parser;
    let mut ast: AST;
    let mut assembler: Assembler;
    let mut symbol_table: SymbolTable = SymbolTable::new();


    tokenizer = Tokenizer::new(source_code.as_str());
    while !tokenizer.is_done() {
        if let Some(token) = tokenizer.get_next_token() {
            tokens.push(token.clone());
        }
    }
    Tokenizer::fix_comparison_operators(&mut tokens);
    // println!("{:?}", tokens);

    parser = Parser::new(tokens, &mut symbol_table);
    ast = AST::new(vec![]);

    let mut node: ASTNode = ASTNode::Start;
    while node != ASTNode::End && node != ASTNode::Error {
        ast.add_node(node);
        node = parser.parse();
    }
    if node == ASTNode::Error {
        println!("Code has errors!");
    } else {
        ast.print_nodes();
    }

    symbol_table.print_table();
    
    assembler = Assembler::new(ast, &mut symbol_table);
    assembler.generate();

    Ok(())
}
