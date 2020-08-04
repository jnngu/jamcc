use crate::parser;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

pub fn generate_program(program: parser::Program, filename: &str,jump_count: Box<u32>) -> ()
{
    let f = File::create(filename.replace(".c", ".s")).unwrap();

    match program
    {
        parser::Program::Prog(decl) => generate_fun_decl(decl, &f, filename, jump_count),
        _ => parse_error("Not a valid program", filename)
    }

}

pub fn generate_fun_decl(decl: parser::FunDecl, mut f: &File, filename: &str,jump_count: Box<u32>) -> ()
{
    match decl
    {
        parser::FunDecl::Fun(fun_name, fun_statement) => 
        {
            f.write_all(format!("   .globl {}\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("	.type	{}, @function\n", fun_name).as_bytes()).unwrap();
            f.write_all(format!("{}:\n", fun_name).as_bytes()).unwrap(); 
            generate_statement(fun_statement, &f, filename, jump_count);
        }
        _ => parse_error("Not a valid function declaration", filename)
    }
}

pub fn generate_statement(statement: parser::Statement, mut f: &File, filename: &str, mut jump_count: Box<u32>) -> ()
{
    match statement
    {
        parser::Statement::Return(expr) => 
        {
            generate_exp(expr, &f, filename,  &mut jump_count);
            f.write_all(format!("	ret\n").as_bytes()).unwrap();
        }
        _ => parse_error("Not a valid statement", filename)
    }
}

pub fn generate_exp(expr: parser::Exp, mut f: &File,  filename: &str, jump_count: &mut Box<u32>) -> ()
{
    match expr
    {
        parser::Exp::Const(num) =>
        {
            f.write_all(format!("	movl    ${}, %eax\n", num).as_bytes()).unwrap();
        },
        parser::Exp::UnOp(op, wrap_exp) =>
        {
            generate_exp(*wrap_exp, &f, filename, jump_count);
            generate_unop(op, &f, filename);
        },
        parser::Exp::BinOp(parser::BinaryOp::LogOr, exp1, exp2) =>
        {
            let current_count:u32 = **jump_count;
            **jump_count = **jump_count + 1;
            
            generate_exp(*exp1, &f, filename, jump_count);
            f.write_all(format!("	cmpl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	je _clause{}\n", current_count).as_bytes()).unwrap();
            f.write_all(format!("	movl    $1, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	jmp _end{}\n", current_count).as_bytes()).unwrap();
            f.write_all(format!("_clause{}:\n", current_count).as_bytes()).unwrap();
            generate_exp(*exp2, &f, filename, jump_count);
            f.write_all(format!("	cmpl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	setne   %al\n").as_bytes()).unwrap();
            f.write_all(format!("_end{}:\n", current_count).as_bytes()).unwrap();
            
        },
        
        parser::Exp::BinOp(parser::BinaryOp::LogAnd, exp1, exp2) =>
        {
            let current_count = **jump_count;
            **jump_count = **jump_count + 1;
            
            generate_exp(*exp1, &f, filename, jump_count);
            f.write_all(format!("	cmpl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	jne _clause{}\n", current_count).as_bytes()).unwrap();
            f.write_all(format!("	jmp _end{}\n", current_count).as_bytes()).unwrap();
            f.write_all(format!("_clause{}:\n", current_count).as_bytes()).unwrap();
            generate_exp(*exp2, &f, filename, jump_count);
            f.write_all(format!("	cmpl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("	setne   %al\n").as_bytes()).unwrap();
            f.write_all(format!("_end{}:\n",current_count).as_bytes()).unwrap();
        },
        parser::Exp::BinOp(op, exp1, exp2) =>
        {
            generate_exp(*exp1, &f, filename, jump_count);
            f.write_all(format!("	push    %rax\n").as_bytes()).unwrap();
            generate_exp(*exp2, &f, filename, jump_count);
            f.write_all(format!("	movl    %eax, %ecx\n").as_bytes()).unwrap();
            f.write_all(format!("	pop     %rax\n").as_bytes()).unwrap();
            generate_binop(op, &f, filename);
        }
        _ => 
        { 
            parse_error("Not a valid expression", filename);
            panic!()
        } 
        

    }
}

pub fn generate_unop(op: parser::UnaryOp, mut f: &File, filename: &str) -> ()
{
    match op
    {
        parser::UnaryOp::Negation => f.write_all(format!("    neg     %eax\n").as_bytes()).unwrap(),
        parser::UnaryOp::Complement => f.write_all(format!("    not     %eax\n").as_bytes()).unwrap(),
        parser::UnaryOp::LogNegation => 
        {
            f.write_all(format!("    cmpl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    sete    %al\n").as_bytes()).unwrap();
        },
        _=> parse_error("Not a valid UnaryOp", filename),
    }
}

pub fn generate_binop(op:parser::BinaryOp, mut f: &File, filename: &str) -> ()
{
    match op
    {
        parser::BinaryOp::Addition => f.write_all(format!("    addl    %ecx, %eax\n").as_bytes()).unwrap(),
        parser::BinaryOp::Subtraction => f.write_all(format!("    subl    %ecx, %eax\n").as_bytes()).unwrap(),
        parser::BinaryOp::Multiplication => f.write_all(format!("    imul    %ecx, %eax\n").as_bytes()).unwrap(),
        parser::BinaryOp::Division => 
        {
            f.write_all(format!("    cdq\n").as_bytes()).unwrap();
            f.write_all(format!("    idivl   %ecx, %eax\n").as_bytes()).unwrap();
        },
        parser::BinaryOp::LessThan => 
        {
            f.write_all(format!("    cmpl    %ecx, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    setl    %al\n").as_bytes()).unwrap();
        },
        parser::BinaryOp::GreaterThan => 
        {
            f.write_all(format!("    cmpl    %ecx, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    setg    %al\n").as_bytes()).unwrap();
        },
        parser::BinaryOp::LessThanOrEq  => 
        {
            f.write_all(format!("    cmpl    %ecx, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    setle    %al\n").as_bytes()).unwrap();
        },
        parser::BinaryOp::GreaterThanOrEq  => 
        {
            f.write_all(format!("    cmpl    %ecx, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    setge   %al\n").as_bytes()).unwrap();
        },
        parser::BinaryOp::NotEqual => 
        {
            f.write_all(format!("    cmpl    %ecx, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    setne   %al\n").as_bytes()).unwrap();
        },
        parser::BinaryOp::Equal => 
        {
            f.write_all(format!("    cmpl    %ecx, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    movl    $0, %eax\n").as_bytes()).unwrap();
            f.write_all(format!("    sete    %al\n").as_bytes()).unwrap();
        },
        _=> parse_error("Not a valid binary op", filename),
    }
}
pub fn parse_error(s: &str, filename: &str) -> ()
{
    fs::remove_file(filename.replace(".c", ".s")).expect("file remove error");
    panic!("{}", s)
}