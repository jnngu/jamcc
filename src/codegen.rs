use crate::parser;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

pub fn generate_program(program: parser::prog, name: &str) -> ()
{
    let mut f = File::create(name.replace(".c", ".s")).unwrap();

    match program
    {
        parser::prog::Prog(decl) => generate_fun_decl(decl, &f),
        _ => panic!("Not a valid program")
    }

}

pub fn generate_fun_decl(decl: parser::fun_decl, mut f: &File) -> ()
{
    match decl
    {
        parser::fun_decl::Fun(fun_name, fun_statement) => 
        {
            f.write_all(format!("	.globl {}\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("	.type	{}, @function\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("{}:\n", fun_name).as_bytes()).unwrap(); 
            generate_statement(fun_statement, &f);
        }
        _ => panic!("Not a valid function declaration")
    }
}

pub fn generate_statement(statement: parser::statement, mut f: &File) -> ()
{
    match statement
    {
        parser::statement::Return(expr) => 
        {
            f.write_all(format!("	movl	${}, %eax\n", generate_exp(expr)).as_bytes()).unwrap();
            f.write_all(format!("	ret\n").as_bytes()).unwrap();
        }
        _ => panic!("Not a valid statement")
    }
}

pub fn generate_exp(expr: parser::exp) -> String
{
    match expr
    {
        parser::exp::Const(num) =>
        {
            return num.to_string()
        }
        _ => panic!("Not a valid expression")
    }
}