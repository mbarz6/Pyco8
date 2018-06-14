use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;

pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(String, u8), // opname + precedence
    NumericLiteral(f64),
    OpenParen,
    CloseParen,
    WhitespaceBlock(u8), // stores size of whitespace block
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Identifier(ref string) => write!(f, "{}", string),
            Token::Keyword(ref string) => write!(f, "{}", string),
            Token::Operator(ref string, prec) => write!(f, "({} {})", string, prec),
            Token::NumericLiteral(ref val) => write!(f, "{}", val),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::WhitespaceBlock(ref amt) => write!(f, "{}spaces", amt),
        }
    }
}

const KEYWORDS: [&str; 4] = [ "if", "else", "function", "=" ];

// lexes a line of code
pub fn lex(line: &str) -> Vec<Box<Token>> {
    let OPERATORS: HashMap<&str, u8> = [ ("+", 1), ("-", 1), ("*", 2) ].iter().cloned().collect();

    let mut tokens: Vec<Box<Token>> = Vec::new();
    
    let mut chars = line.chars();
    let mut whitespace_count = 0;
    loop {
        match chars.next() {
            Some(' ') => whitespace_count += 1,
            _ => break,
        }
    }
    if whitespace_count > 0 {
        tokens.push(Box::new(Token::WhitespaceBlock(whitespace_count)));
    }        

    let mut symbols = line.split_whitespace(); // iterable list of words
    // loop over symbols
    loop {
        match symbols.next() {
            Some(mut symbol) => {
                if let Some(c) = symbol.chars().next() {
                    if c == '(' { 
                        tokens.push(Box::new(Token::OpenParen));
                        symbol = &symbol[1..];
                    } 
                }
                let mut should_close_paren = false;
                if let Some(c) = symbol.chars().last() {
                    if c == ')' {
                        symbol = &symbol[0..(symbol.len() - 1)];
                        should_close_paren = true;
                    }
                }
                

                tokens.push(
                    if KEYWORDS.contains(&symbol) {
                        Box::new(Token::Keyword(String::from(symbol)))   
                    } else if OPERATORS.contains_key(&symbol) {
                        Box::new(Token::Operator(String::from(symbol), OPERATORS[symbol]))  
                    } else if symbol.chars().next().unwrap().is_digit(10) || symbol.chars().next().unwrap() == '.' {
                        Box::new(Token::NumericLiteral(f64::from_str(symbol).unwrap()))
                    } else if symbol.chars().next().unwrap() == '(' {
                        Box::new(Token::OpenParen)
                    } else if symbol.chars().next().unwrap() == ')' {
                        Box::new(Token::CloseParen)
                    } else {
                        Box::new(Token::Identifier(String::from(symbol)))
                    }
                );
                if should_close_paren {
                    tokens.push(Box::new(Token::CloseParen));
                }
            },
            None => break, // quit loop at end
        }
    }

    tokens
}