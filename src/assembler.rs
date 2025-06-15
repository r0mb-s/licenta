use std::fmt::format;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::ast::{ASTNode, AST};
use crate::symbol_table::{self, SymbolTable};

pub struct Assembler<'a> {
    tree: AST,
    symbol_table: &'a SymbolTable,
    data_section: Vec<String>,
    bss_section: Vec<String>,
    functions: Vec<String>,
    start: Vec<String>,
    label_counter: usize,
}

impl<'a> Assembler<'a> {
    pub fn new(tree: AST, symbol_table: &'a mut SymbolTable) -> Self {
        let mut ds: Vec<String> = Vec::new();
        let mut bss: Vec<String> = Vec::new();
        for symb in &symbol_table.table {
            if symb.vtype == "int" {
                ds.push(format!("{} dd 0", symb.vname));
            } else if symb.vtype == "arr" {
                bss.push(format!("{} resd {}", symb.vname, symb.array_size));
            }
        }
        bss.push("buffer resb 12".to_string());
        symbol_table.reset_level();
        Assembler {
            tree,
            symbol_table,
            data_section: ds,
            bss_section: bss,
            functions: Vec::new(),
            start: Vec::new(),
            label_counter: 0,
        }
    }

    pub fn write_to_file(&self, file_path: &str) -> io::Result<()> {
        let path = Path::new(file_path);
        let mut file = File::create(&path)?;

        writeln!(file, "section .data")?;
        for var in &self.data_section {
            writeln!(file, "{}", var)?;
        }
        writeln!(file, "newline db 0xA")?;

        writeln!(file, "\nsection .bss")?;
        for var in &self.bss_section {
            writeln!(file, "{}", var)?;
        }

        writeln!(file, "\nsection .text")?;
        writeln!(file, "global _start")?;

        writeln!(file, "\n_start:")?;
        for line in &self.start {
            writeln!(file, "{}", line)?;
        }

        writeln!(file, "\nmov eax, 1")?;
        writeln!(file, "xor ebx, ebx")?;
        writeln!(file, "int 0x80")?;

        for line in &self.functions {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    pub fn generate(&mut self) {
        for node in self.tree.get_nodes() {
            self.generate_node(&node);
        }

        let _ = self.write_to_file("assm/out.asm");
    }

    fn new_label(&mut self, base: &str) -> String {
        let label = format!("{}_{}", base, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn generate_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Assignment { var_name, expr } => {
                self.generate_node(&expr);
                self.start.push(format!("mov [{}], eax", var_name));
            }
            ASTNode::ArrayDeclaration { arr_name, size } => {
                // self.start.push(format!("lea esi, [{}]", arr_name));
            },
            ASTNode::ArrayAssignment { arr_name, position, value } => {
                self.start.push(format!("lea esi, [{}]", arr_name));
                self.generate_node(&position);
                self.start.push(format!("mov ecx, eax"));
                self.generate_node(&value);
                self.start.push(format!("mov [esi + ecx*4], eax",));
            }
            ASTNode::BinaryOperation { op, left, right } => {
                self.generate_node(right);
                self.start.push("push eax".into());
                self.generate_node(left);
                self.start.push("pop ebx".into());

                match op.as_str() {
                    "+" => self.start.push("add eax, ebx".into()),
                    "-" => self.start.push("sub eax, ebx".into()),
                    "*" => self.start.push("imul eax, ebx".into()),
                    "/" => {
                        self.start.push("mov edx, 0".into());
                        self.start.push("div ebx".into());
                    }
                    "%" => {
                        self.start.push("mov edx, 0".into());
                        self.start.push("div ebx".into());
                        self.start.push("mov eax, edx".into());
                    }
                    _ => self.start.push("; Unknown binary operator".into()),
                }
            }
            ASTNode::Literal(value) => {
                self.start.push(format!("mov eax, {}", value));
            }
            ASTNode::Variable(name) => {
                self.start.push(format!("mov eax, [{}]", name));
            }
            ASTNode::IfOperation {
                first_half,
                comparison_op,
                second_half,
                content,
            } => {
                self.generate_node(second_half);
                self.start.push("push eax".into());
                self.generate_node(first_half);
                self.start.push("pop ebx".into());
                self.start.push("cmp eax, ebx".into());

                let endif_label = self.new_label("endif");
                match comparison_op.as_str() {
                    "==" => self.start.push(format!("jne {}", endif_label)),
                    "=!" => self.start.push(format!("je {}", endif_label)),
                    "<" => self.start.push(format!("jge {}", endif_label)),
                    ">" => self.start.push(format!("jle {}", endif_label)),
                    "=<" => self.start.push(format!("jg {}", endif_label)),
                    "=>" => self.start.push(format!("jl {}", endif_label)),
                    _ => self.start.push("; Unknown comparison".into()),
                }

                for exp in content {
                    self.generate_node(exp);
                }
                self.start.push(format!("{}:", endif_label));
            }
            ASTNode::WhileOperation {
                first_half,
                comparison_op,
                second_half,
                content,
            } => {
                let start_label = self.new_label("while_start");
                let end_label = self.new_label("while_end");

                self.start.push(format!("{}:", start_label));
                self.generate_node(second_half);
                self.start.push("push eax".into());
                self.generate_node(first_half);
                self.start.push("pop ebx".into());
                self.start.push("cmp eax, ebx".into());

                match comparison_op.as_str() {
                    "==" => self.start.push(format!("jne {}", end_label)),
                    "=!" => self.start.push(format!("je {}", end_label)),
                    "<" => self.start.push(format!("jge {}", end_label)),
                    ">" => self.start.push(format!("jle {}", end_label)),
                    "=<" => self.start.push(format!("jg {}", end_label)),
                    "=>" => self.start.push(format!("jl {}", end_label)),
                    _ => self.start.push("; Unknown comparison".into()),
                }

                for exp in content {
                    self.generate_node(exp);
                }

                self.start.push(format!("jmp {}", start_label));
                self.start.push(format!("{}:", end_label));
            }
            ASTNode::EndIf | ASTNode::EndWhile | ASTNode::Start | ASTNode::End => {}
            ASTNode::Error => {
                self.start.push("; Error node encountered".into());
            }
            ASTNode::FunctionDef { name, parameters } => todo!(),
            ASTNode::EndFunctionDef => todo!(),
            ASTNode::FuntionCall { name, parameters } => todo!(),
            ASTNode::Print(astnode) => {
                self.generate_node(astnode);
                self.start.push("call print_eax".into());

                let mut print_func: String = String::new();
                print_func.push_str("print_eax:\n");
                print_func.push_str("push ecx\n");
                print_func.push_str("push edx\n");
                print_func.push_str("mov edi, buffer + 11\n");
                print_func.push_str("mov byte [edi], 0\n");
                print_func.push_str("mov ebx, 10\n");
                print_func.push_str(".convert_loop:\n");
                print_func.push_str("dec edi\n");
                print_func.push_str("xor edx, edx\n");
                print_func.push_str("div ebx\n");
                print_func.push_str("add dl, '0'\n");
                print_func.push_str("mov [edi], dl\n");
                print_func.push_str("test eax, eax\n");
                print_func.push_str("jnz .convert_loop\n");
                print_func.push_str("mov eax, 4\n");
                print_func.push_str("mov ebx, 1\n");
                print_func.push_str("mov ecx, edi\n");
                print_func.push_str("mov edx, buffer + 11\n");
                print_func.push_str("sub edx, edi\n");
                print_func.push_str("int 0x80\n");
                print_func.push_str("mov eax, 4\n");
                print_func.push_str("mov ebx, 1\n");
                print_func.push_str("mov ecx, newline\n");
                print_func.push_str("mov edx, 1\n");
                print_func.push_str("int 0x80\n");
                print_func.push_str("pop edx\n");
                print_func.push_str("pop ecx\n");
                print_func.push_str("ret\n");

                if self.functions.is_empty() {
                    self.functions.push(print_func);
                }
            }
            ASTNode::Array { arr_name, index } => {
                self.generate_node(index);
                self.start.push(format!("lea esi, [{}]", arr_name));
                self.start.push(format!("mov eax, [esi + eax*4]"));
            }
        }
    }
}
