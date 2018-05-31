use std::str::FromStr;
use std::fmt;

pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(String),
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
            Token::Operator(ref string) => write!(f, "{}", string),
            Token::NumericLiteral(ref val) => write!(f, "{}", val),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::WhitespaceBlock(ref amt) => write!(f, "{}spaces", amt),
        }
    }
}

const KEYWORDS: [&str; 3] = [ "if", "else", "function" ];
const OPERATORS: [&str; 3] = [ "=", "+", "-" ];

// lexes a line of code
pub fn lex(line: &str) -> Vec<Box<Token>> {
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
            Some(symbol) => {
                tokens.push(
                    if KEYWORDS.contains(&symbol) {
                        Box::new(Token::Keyword(String::from(symbol)))   
                    } else if OPERATORS.contains(&symbol) {
                        Box::new(Token::Operator(String::from(symbol)))  
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
            },
            None => break, // quit loop at end
        }
    }

    tokens
}