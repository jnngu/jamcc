use crate::parser;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
pub fn generate_code(program: parser::prog) -> (){
    let mut f = File::create("test.s").unwrap();

    let function_decl: parser::fun_decl;
    match program
    {
        parser::prog::Prog(dec) => function_decl = dec,
        _ => panic!("not a valid program"),
    }

    let function_name: String;
    let function_statement: parser::statement;
    match function_decl
    {
        parser::fun_decl::Fun(x,y) => {
            function_name = x;
            function_statement = y;
        }
        _ => panic!("not a valid function"),
    }
    f.write_all(format!("	.globl {}\n", function_name).as_bytes()).unwrap();
    f.write_all(format!("	.type	{}, @function\n", function_name).as_bytes()).unwrap();
    f.write_all(format!("{}:\n", function_name).as_bytes()).unwrap();
    match function_statement
    {
        parser::statement::Return(parser::exp::Const(x)) => {
            f.write_all(format!("	movl	${}, %eax\n", x).as_bytes()).unwrap();
            f.write_all(format!("	ret\n").as_bytes()).unwrap();
        }
    }
}
