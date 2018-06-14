use super::lexer::Token;
use super::lexer::lex;

use std::collections::HashMap;
use std::ops::Deref;

enum Variable {
    Number(f64),
}

impl Variable {
    fn to_num(&self) -> Option<f64> {
        match self {
            &Variable::Number(x) => Some(x),
            _ => None
        }
    }
}

type SymbolTable = HashMap<String, Variable>;

struct Context {
    locals: SymbolTable,
    parent: Option<Box<Context>>,
}

impl Context {
    fn new_global() -> Context {
        Context{ locals: SymbolTable::new(), parent: None}
    }

    fn new(parent: Box<Context>) -> Context {
        Context { locals: SymbolTable::new(), parent: Some(parent) }
    }

    // adds a variable to local scope
    // note: This *will* overwrite existing values in local scope!
    fn add(&mut self, name: &str, value: Variable) {
        self.locals.insert(name.to_string(), value);
    }

    // modifies a variable
    fn set(&mut self, name: &str, value: Variable) {
        if self.locals.contains_key(name) {
            self.add(name, value);
        } else if let Some(ref mut parent) = self.parent {
            parent.set(name, value);
        } else {
            panic!("Tried to set a variable which doesn't exist.");
        }
    }

    // gets literal associated with variable name
    fn get(&self, name: &str) -> Option<&Variable> {
        if self.locals.contains_key(name) {
            self.locals.get(name)
        } else if let Some(ref parent) = self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    // restores the parent context
    fn restore_parent(self) -> Option<Context> {
        if let Some(parent) = self.parent {
            Some(*parent)
        } else {
            None
        }
    }
}

// computes an expression, like 2 * (5 - 3)
pub fn compute(tokens: Vec<Box<Token>>) -> Vec<Box<Token>> {
    // first, convert to RPN with variation on shunting-yard
    let mut output: Vec<Box<Token>> = Vec::new();
    let mut operators: Vec<Box<Token>> = Vec::new();

    for token in tokens {
        /*for token in &output {
            print!("{} ", token);
        }
        print!(";");
        for token in &operators {
            print!("{} ", token);
        }
        println!("");*/

        match *token {
            Token::Operator(_, pref) => {
                while operators.len() > 0 {
                    if let Token::Operator(_, top_pref) = *operators[operators.len() - 1] {
                        if top_pref >= pref {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }  
                }
                operators.push(token);
            },
            Token::NumericLiteral(_) => {
                output.push(token);
            },
            Token::OpenParen => {
                operators.push(token);
            },
            Token::CloseParen => {
                loop {
                    let operator = operators.pop().unwrap();
                    match *operator {
                        Token::OpenParen => { break; }
                        _ => { output.push(operator); } 
                    }
                }
            },
            _ => (),
        }
    }

    while operators.len() > 0 { 
        output.push(operators.pop().unwrap());
    }

    // now, the computing part!
    output
}