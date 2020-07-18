use std::collections::VecDeque;
use crate::lexer;

pub enum exp {Const(i32)}
pub enum statement {Return(exp)}
pub enum fun_decl {Fun(String, statement)}
pub enum prog {Prog(fun_decl)}


pub fn parse_exp (token_vec:&mut VecDeque<lexer::Token>) -> exp
{
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::IntegerLiteral(n)) => {println!("IntegerLiteral({})", n); return exp::Const(n)},
        _ => panic!("not valid return keyword"),
    }   
} 


pub fn parse_statement(mut token_vec:&mut VecDeque<lexer::Token>) -> statement
{
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::ReturnKeyword) => println!("ReturnKeyword"),
        _ => panic!("not valid return keyword"),
    }
    let statement_exp:exp = parse_exp(&mut token_vec);
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::Semicolon) => println!("Semicolon"),
        _ => panic!("not valid semicolon"),
    } 

    statement::Return(statement_exp)
}  

pub fn parse_fun_decl(mut token_vec: VecDeque<lexer::Token>) -> fun_decl
{
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::IntKeyword) => println!("IntKeyword"),
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
        Some(lexer::Token::OpenParen) => println!("OpenParen"),
        _ => panic!("not valid OpenParen"),
    }
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::ClosedParen) => println!("ClosedParen"),
        _ => panic!("not valid ClosedParen"),
    }
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::OpenBrace) => println!("OpenBrace"),
        _ => panic!("not valid OpenBrace"),
    }
    
    
    let prog_statement = parse_statement(&mut token_vec); 
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::ClosedBrace) => println!("ClosedBrace"),
        _ => println!("{:?}", token_vec),
    }     


    fun_decl::Fun(fun_string, prog_statement) 
}

pub fn parse_program(token_vec:VecDeque<lexer::Token>) -> prog
{
    prog::Prog(parse_fun_decl(token_vec))
}
 
