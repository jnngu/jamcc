use crate::parser;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
//TODO: generate file at very end instead of beginning
pub fn generate_program(program: parser::Program, filename: &str) -> ()
{
    let mut f = File::create(filename.replace(".c", ".s")).unwrap();

    match program
    {
        parser::Program::Prog(decl) => generate_fun_decl(decl, &f, filename),
        _ => parse_error("Not a valid program", filename)
    }

}

pub fn generate_fun_decl(decl: parser::FunDecl, mut f: &File, filename: &str) -> ()
{
    match decl
    {
        parser::FunDecl::Fun(fun_name, fun_statement) => 
        {
            f.write_all(format!("   .globl {}\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("	.type	{}, @function\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("{}:\n", fun_name).as_bytes()).unwrap(); 
            generate_statement(fun_statement, &f, filename);
        }
        _ => parse_error("Not a valid function declaration", filename)
    }
}

pub fn generate_statement(statement: parser::Statement, mut f: &File, filename: &str) -> ()
{
    match statement
    {
        parser::Statement::Return(expr) => 
        {
            generate_exp(expr, &f, filename);
            f.write_all(format!("	ret\n").as_bytes()).unwrap();
        }
        _ => parse_error("Not a valid statement", filename)
    }
}

pub fn generate_exp(expr: parser::Exp, mut f: &File,  filename: &str) -> ()
{
    match expr
    {
        parser::Exp::Const(num) =>
        {
            f.write_all(format!("	movl    ${}, %eax\n", num).as_bytes()).unwrap();
        },
        parser::Exp::UnOp(op, wrapExp) =>
        {
            generate_exp(*wrapExp, &f, filename);
            generate_op(op, &f, filename);
        }
        _ => 
        { 
            parse_error("Not a valid expression", filename);
            panic!()
        }


    }
}

pub fn generate_op(op: parser::Operator, mut f: &File, filename: &str) -> ()
{
    match op
    {
        parser::Operator::Negation => f.write_all(format!("    neg     %eax\n").as_bytes()).unwrap(),
        parser::Operator::Complement => f.write_all(format!("    not     %eax\n").as_bytes()).unwrap(),
        parser::Operator::LogNegation => 
        {
            f.write_all(format!("    cmpl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    sete    %al\n").as_bytes()).unwrap();
        },
        _=> parse_error("Not a valid operator", filename),
    }
}

pub fn parse_error(s: &str, filename: &str) -> ()
{
    fs::remove_file(filename.replace(".c", ".s"));
    panic!("{}", s)
}