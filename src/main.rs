mod parser;
mod lexer;

use std::env;
use std::collections::VecDeque;
//TODO: Put printlns into a debug macro

fn main() {
    let args: VecDeque<String> = env::args().collect();
    let contents = lexer::read_file(&args[1]);
    println!("Contents:\n {}", contents);

    let symbol_vec:VecDeque<lexer::Token> = lexer::parse_string(&contents);
    //println!("{:?}", &symbol_vec);
    //lexer::print_tokens(symbol_vec);

    parser::parse_program(symbol_vec);

/*     let mut test = String::from("aaaaa");
    while(!(test.is_empty()))
    {
        test = test.replacen("a", "", 1);
        println!("{}", test);
    } */
} 

