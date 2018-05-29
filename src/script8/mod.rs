use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Enum that can store either float or integer.
pub enum NumberLiteral {
    Integer(i64),
    Float(f64),
}

pub enum Variable {
    Numeric(NumberLiteral),
}

pub type SymbolTable = HashMap<String, Variable>;

pub struct Interpreter {
    globals: SymbolTable,
    code: String,
}

impl Interpreter {
    /// Filters a line of code, removing comments
    pub fn filter_code(line: &str) -> String {
        let mut filtered = String::new();
        let mut started = false;
        for c in line.chars() {
            if c == '#' {
                break;
            }

            filtered.push(c);
        }
        filtered
    }

    /// Constructs interpreter from given script
    pub fn from_code(lines: &str) -> Interpreter {
        let mut symbols: SymbolTable = HashMap::new();
        Interpreter{ globals: symbols, code: lines.to_string() }
    }

    pub fn evaluate(&self) {
        for line in self.code.lines() {
            let mut symbols: Vec<&str> = Vec::new();
            let mut spaces = line.split_whitespace();
            loop {
                match spaces.next() {
                    Some(symbol) => { symbols.push(symbol) },
                    None => { break }
                }
            }

            
        }
    }
}

impl Interpreter {
    fn create_var(&mut self, name: &str, val: Variable) {
        self.globals.insert(name.to_string(), val);
    }

    fn compute(&self, expression: Vec<&str>) {
        
    }
}