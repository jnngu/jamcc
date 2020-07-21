
#[macro_use] mod macros;
mod parser;
mod lexer;
mod codegen;
use std::env;
use std::collections::VecDeque;
//TODO: Put printlns into a debug macro


fn main() {
    let args: VecDeque<String> = env::args().collect();
    let contents = lexer::read_file(&args[1]);
    debug_print!("Contents:\n {}\n", contents);
    debug_print!("Lexer:");
    let symbol_vec:VecDeque<lexer::Token> = lexer::parse_string(&contents);
    debug_print!("\nLexer Output: {:?}\n", symbol_vec);
    debug_print!("Parser:");
    let parsed_prog:parser::Program = parser::parse_program(symbol_vec);
    debug_print!("\nParser Output: \n{}\n", parsed_prog);
    codegen::generate_program(parsed_prog, &args[1]);
} 

