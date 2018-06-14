mod lexer;
mod parser;

pub fn tests() {
    let tokens = lexer::lex("1 * 2 * 3 * 4 - 5 * 6 + 7 - 8 * 9");
    let tokens = parser::compute(tokens);
    for token in tokens {
        println!("{}", *token);
    }
}