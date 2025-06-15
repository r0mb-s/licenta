use std::array;

#[derive(Debug, Clone)]
pub struct SymbolTableEntry {
    pub vname: String,
    pub vtype: String,
    pub array_size: u32,
    level: u32,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub table: Vec<SymbolTableEntry>,
    curr_level: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: Vec::new(),
            curr_level: 0,
        }
    }

    pub fn add_to_table(&mut self, var: String, var_type: String, array_size: u32) {
        self.table.push(SymbolTableEntry {
            vname: var,
            vtype: var_type,
            level: self.curr_level,
            array_size: array_size,
        });
    }

    pub fn reset_level(&mut self) {
        self.curr_level = 0;
    }

    pub fn down(&mut self) {
        self.curr_level -= 1;
    }

    pub fn up(&mut self) {
        self.curr_level += 1;
    }

    pub fn check_table(&self, var: String) -> Option<&SymbolTableEntry> {
        for symbol in &self.table {
            if symbol.vname == var && symbol.level <= self.curr_level {
                return Some(symbol);
            }
        }
        None
    }

    pub fn print_table(&mut self) {
        println!("Current level: {}", self.curr_level);
        for symb in &self.table {
            println!(
                "Symbol: {}, Type: {}, Level: {}",
                symb.vname, symb.vtype, symb.level
            );
        }
    }
}
