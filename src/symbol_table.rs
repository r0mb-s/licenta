#[derive(Debug)]
pub struct SymbolTableEntry {
    pub vname: String,
    vtype: String,
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

    pub fn add_to_table(&mut self, var: String) {
        self.table.push(SymbolTableEntry {
            vname: var,
            vtype: "int".to_string(),
            level: self.curr_level,
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

    pub fn check_table(&self, var: String) -> bool {
        let mut found: bool = false;
        for symbol in &self.table {
            if symbol.vname == var && symbol.level == self.curr_level {
                found = true;
                break;
            }
        }
        found
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
