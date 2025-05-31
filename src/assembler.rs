use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::ast::{ASTNode, AST};
use crate::symbol_table::{self, SymbolTable};

pub struct Assembler<'a> {
    tree: AST,
    symbol_table: &'a SymbolTable,
    data_section: String,
    functions: Vec<String>, 
    assembly: Vec<String>,
    label_counter: usize,
}

impl<'a> Assembler<'a> {
    pub fn new(tree: AST, symbol_table: &'a mut SymbolTable) -> Self {
        symbol_table.reset_level();
        Assembler {
            tree,
            symbol_table,
            assembly: Vec::new(),
            label_counter: 0,
        }
    }

    pub fn write_to_file(&self, file_path: &str) -> io::Result<()> {
        let path = Path::new(file_path);
        let mut file = File::create(&path)?;

        writeln!(file, "section .data")?;
        for i in 0..self.symbol_table.table.len() {
            writeln!(file, "{} dd 0", self.symbol_table.table[i].vname)?;
        }
        writeln!(file, "buffer db 12 dup(0)")?;
        writeln!(file, "section .text")?;
        writeln!(file, "global _start")?;

        writeln!(file, "_start:")?;
        for line in &self.assembly {
            writeln!(file, "{}", line)?;
        }

        writeln!(file, "mov eax, 1")?;
        writeln!(file, "xor ebx, ebx")?;
        writeln!(file, "int 0x80")?;

        Ok(())
    }

    pub fn generate(&mut self) {
        for node in self.tree.get_nodes() {
            self.generate_node(&node);
        }

        let _ = self.write_to_file("alabala.asm");
    }

    fn new_label(&mut self, base: &str) -> String {
        let label = format!("{}_{}", base, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn generate_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Assignment { var_name, expr } => {
                self.generate_node(expr);
                self.assembly.push(format!("mov [{}], eax", var_name));
            }
            ASTNode::BinaryOperation { op, left, right } => {
                self.generate_node(right);
                self.assembly.push("push eax".into());
                self.generate_node(left);
                self.assembly.push("pop ebx".into());

                match op.as_str() {
                    "+" => self.assembly.push("add eax, ebx".into()),
                    "-" => self.assembly.push("sub eax, ebx".into()),
                    "*" => self.assembly.push("imul eax, ebx".into()),
                    "/" => {
                        self.assembly.push("mov edx, 0".into());
                        self.assembly.push("div ebx".into());
                    }
                    _ => self.assembly.push("; Unknown binary operator".into()),
                }
            }
            ASTNode::Literal(value) => {
                self.assembly.push(format!("mov eax, {}", value));
            }
            ASTNode::Variable(name) => {
                self.assembly.push(format!("mov eax, [{}]", name));
            }
            ASTNode::IfOperation {
                first_half,
                comparison_op,
                second_half,
            } => {
                self.generate_node(second_half);
                self.assembly.push("push eax".into());
                self.generate_node(first_half);
                self.assembly.push("pop ebx".into());
                self.assembly.push("cmp eax, ebx".into());

                let endif_label = self.new_label("endif");
                match comparison_op.as_str() {
                    "==" => self.assembly.push(format!("jne {}", endif_label)),
                    "!=" => self.assembly.push(format!("je {}", endif_label)),
                    "<" => self.assembly.push(format!("jge {}", endif_label)),
                    ">" => self.assembly.push(format!("jle {}", endif_label)),
                    "<=" => self.assembly.push(format!("jg {}", endif_label)),
                    ">=" => self.assembly.push(format!("jl {}", endif_label)),
                    _ => self.assembly.push("; Unknown comparison".into()),
                }

                // Body should follow this node in AST
                // A real compiler would wrap the block in a container node

                self.assembly.push(format!("{}:", endif_label));
            }
            ASTNode::WhileOperation {
                first_half,
                comparison_op,
                second_half,
            } => {
                let start_label = self.new_label("while_start");
                let end_label = self.new_label("while_end");

                self.assembly.push(format!("{}:", start_label));
                self.generate_node(second_half);
                self.assembly.push("push eax".into());
                self.generate_node(first_half);
                self.assembly.push("pop ebx".into());
                self.assembly.push("cmp eax, ebx".into());

                match comparison_op.as_str() {
                    "==" => self.assembly.push(format!("jne {}", end_label)),
                    "!=" => self.assembly.push(format!("je {}", end_label)),
                    "<" => self.assembly.push(format!("jge {}", end_label)),
                    ">" => self.assembly.push(format!("jle {}", end_label)),
                    "<=" => self.assembly.push(format!("jg {}", end_label)),
                    ">=" => self.assembly.push(format!("jl {}", end_label)),
                    _ => self.assembly.push("; Unknown comparison".into()),
                }

                // Body should follow in AST
                self.assembly.push(format!("jmp {}", start_label));
                self.assembly.push(format!("{}:", end_label));
            }
            ASTNode::EndIf | ASTNode::EndWhile | ASTNode::Start | ASTNode::End => {}
            ASTNode::Error => {
                self.assembly.push("; Error node encountered".into());
            }
            ASTNode::FunctionDef { name, parameters } => todo!(),
            ASTNode::EndFunctionDef => todo!(),
            ASTNode::FuntionCall { name, parameters } => todo!(),
            ASTNode::Print(astnode) => {
                let mut print_str: String = String::new();
                print_str += &format!("mov eax, [{}]\n", astnode);
                print_str += &format!("mov ecx, buffer + 11\n");
                print_str += &format!("mov byte [ecx], 0\n");
                print_str += &format!("convert_loop:\n");
                print_str += &format!("xor edx, edx\n");
                print_str += &format!("mov ebx, 10\n");
                print_str += &format!("div ebx\n");
                print_str += &format!("add dl, \'0\'\n");
                print_str += &format!("dec ecx\n");
                print_str += &format!("mov [ecx], dl\n");
                print_str += &format!("test eax, eax\n");
                print_str += &format!("jnz convert_loop\n");
                print_str += &format!("\n");
                print_str += &format!("mov eax, 4\n");
                print_str += &format!("mov ebx, 1\n");
                print_str += &format!("mov edx, buffer + 11\n");
                print_str += &format!("sub edx, ecx\n");
                print_str += &format!("mov esi, ecx\n");
                print_str += &format!("mov ecx, esi\n");
                print_str += &format!("int 0x80\n");

                self.assembly.push(print_str);
            }
        }
    }
}
