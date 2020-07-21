use std::collections::VecDeque;
use crate::lexer;
use std::fmt;
#[derive(Debug)]
pub enum Exp {Const(i32)}
#[derive(Debug)]
pub enum Statement {Return(Exp)}
#[derive(Debug)]
pub enum FunDecl {Fun(String, Statement)}
#[derive(Debug)]
pub enum Program {Prog(FunDecl)}

pub fn parse_exp (token_vec:&mut VecDeque<lexer::Token>) -> Exp
{
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::IntegerLiteral(n)) => 
        {
            debug_print!("IntegerLiteral({})", n);
            return Exp::Const(n)
        },
        _ => panic!("not valid return keyword"),
    }   
} 


pub fn parse_statement(mut token_vec:&mut VecDeque<lexer::Token>) -> Statement
{
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::ReturnKeyword) => debug_print!("ReturnKeyword"),
        _ => panic!("not valid return keyword"),
    }
    let statement_exp:Exp = parse_exp(&mut token_vec);
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::Semicolon) => debug_print!("Semicolon"),
        _ => panic!("not valid semicolon"),
    } 

    Statement::Return(statement_exp)
}  

pub fn parse_fun_decl(mut token_vec: VecDeque<lexer::Token>) -> FunDecl
{
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::IntKeyword) => debug_print!("IntKeyword"),
        _ => panic!("not valid int keyword"),
    }
    let id = token_vec.pop_front();
    let fun_string: String;
    match id
    {
        Some(lexer::Token::Identifier(s)) => fun_string = String::from(s),
        _ => panic!("not valid identifier"),
    }
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::OpenParen) => debug_print!("OpenParen"),
        _ => panic!("not valid OpenParen"),
    }
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::ClosedParen) => debug_print!("ClosedParen"),
        _ => panic!("not valid ClosedParen"),
    }
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::OpenBrace) => debug_print!("OpenBrace"),
        _ => panic!("not valid OpenBrace"),
    }
    
    
    let prog_statement = parse_statement(&mut token_vec); 
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::ClosedBrace) => debug_print!("ClosedBrace"),
        _ => debug_print!("{:?}", token_vec),
    }     


    FunDecl::Fun(fun_string, prog_statement) 
}

pub fn parse_program(token_vec:VecDeque<lexer::Token>) -> Program
{
    Program::Prog(parse_fun_decl(token_vec))
}


impl fmt::Display for Program
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Program::Prog(decl) => write!(f,"PROGRAM:\n{}", decl),
            _ => panic!("invalid program"),
        }
    }
}

impl fmt::Display for FunDecl
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            FunDecl::Fun(s, statement) => write!(f, "FUN INT {}:\n  params: ()\n  body:\n{}", s, statement),
            _ => panic!("invalid function declaration"),
        }
    }
}

impl fmt::Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Statement::Return(expr) => write!(f,"    RETURN {}", expr),
            _ => panic!("invalid statement"),
        }
    }
}

impl fmt::Display for Exp
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Exp::Const(n) => write!(f, "Int<{}>\n", n),
            _ => panic!("invalid expression"),
        }
    }
}