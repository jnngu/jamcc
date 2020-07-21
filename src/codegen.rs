use crate::parser;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_program(program: parser::Program, name: &str) -> ()
{
    let mut f = File::create(name.replace(".c", ".s")).unwrap();

    match program
    {
        parser::Program::Prog(decl) => generate_fun_decl(decl, &f),
        _ => panic!("Not a valid program")
    }

}

pub fn generate_fun_decl(decl: parser::FunDecl, mut f: &File) -> ()
{
    match decl
    {
        parser::FunDecl::Fun(fun_name, fun_statement) => 
        {
            f.write_all(format!("	.globl {}\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("	.type	{}, @function\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("{}:\n", fun_name).as_bytes()).unwrap(); 
            generate_statement(fun_statement, &f);
        }
        _ => panic!("Not a valid function declaration")
    }
}

pub fn generate_statement(statement: parser::Statement, mut f: &File) -> ()
{
    match statement
    {
        parser::Statement::Return(expr) => 
        {
            f.write_all(format!("	movl	${}, %eax\n", generate_exp(expr)).as_bytes()).unwrap();
            f.write_all(format!("	ret\n").as_bytes()).unwrap();
        }
        _ => panic!("Not a valid statement")
    }
}

pub fn generate_exp(expr: parser::Exp) -> String
{
    match expr
    {
        parser::Exp::Const(num) =>
        {
            return num.to_string()
        }
        _ => panic!("Not a valid expression")
    }
}