mod lexer;
use std::env;

//TODO: Put printlns into a debug macro

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = lexer::read_file(&args[1]);
    println!("Contents:\n {}", contents);

    let symbol_vec:Vec<String> = lexer::parse_string(&contents);
    println!("{:?}", &symbol_vec);
}

