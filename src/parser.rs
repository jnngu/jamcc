use std::collections::VecDeque;
use crate::lexer;
use std::fmt;


#[derive(Debug)]
pub enum Exp {UnOp(Operator, Box<Exp>), Const(i32)}
#[derive(Debug)]
pub enum Statement {Return(Exp)}
#[derive(Debug)]
pub enum FunDecl {Fun(String, Statement)}
#[derive(Debug)]
pub enum Program {Prog(FunDecl)}
#[derive(Debug)]
pub enum Operator
{
    Negation,
    Complement,
    LogNegation,
}

pub fn get_operator(token: lexer::Token) -> Operator
{
    match token
    {
        lexer::Token::Negation => 
        {
            debug_print!("Negation");
            Operator::Negation
        },
        lexer::Token::Complement => 
        {
            debug_print!("Complement");
            Operator::Complement},
        lexer::Token::LogNegation => 
        {
            debug_print!("LogicalNegation");
            Operator::LogNegation
        },
        _ => panic!("not valid operator"),
    }
}

pub fn parse_exp (mut token_vec:&mut VecDeque<lexer::Token>) -> Exp
{
    //<Int>
    let tok = token_vec.pop_front();
    match tok
    {
        Some(lexer::Token::IntegerLiteral(n)) => 
        {
            debug_print!("IntegerLiteral({})", n);
            return Exp::Const(n)
        },
        Some(token) =>
        {
            let op = get_operator(token);
            let inner_exp = parse_exp(&mut token_vec);
            return Exp::UnOp(op, Box::new(inner_exp))
        },
        _ => panic!("not valid expression, NONE"),
    }   
} 


pub fn parse_statement(mut token_vec:&mut VecDeque<lexer::Token>) -> Statement
{
    //Return <Int>;
    match_token(&mut token_vec, lexer::Token::ReturnKeyword);

    let statement_exp:Exp = parse_exp(&mut token_vec);

    match_token(&mut token_vec, lexer::Token::Semicolon);

    Statement::Return(statement_exp)
}  

pub fn parse_fun_decl(mut token_vec: VecDeque<lexer::Token>) -> FunDecl
{
    //int <func_name> () {<statement>}
    match_token(&mut token_vec, lexer::Token::IntKeyword);

    let id = token_vec.pop_front();
    let fun_string: String;
    match id
    {
        Some(lexer::Token::Identifier(s)) => fun_string = String::from(s),
        _ => panic!("not valid identifier"),
    }

    match_token(&mut token_vec, lexer::Token::OpenParen);

    match_token(&mut token_vec, lexer::Token::ClosedParen);
    
    match_token(&mut token_vec, lexer::Token::OpenBrace);
    
    let prog_statement = parse_statement(&mut token_vec); 

    match_token(&mut token_vec, lexer::Token::ClosedBrace);

    FunDecl::Fun(fun_string, prog_statement) 
}

pub fn parse_program(token_vec:VecDeque<lexer::Token>) -> Program
{
    Program::Prog(parse_fun_decl(token_vec))
}


fn match_token(token_vec:&mut VecDeque<lexer::Token>, desired_token: lexer::Token) -> ()
{
    let tok = token_vec.pop_front();
    match tok
    {

        Some(x) => {
            if x == desired_token 
            {
                debug_print!("{}", desired_token);
            }
            else
            {
                panic!("not valid {}. got {} instead.", desired_token, x);
            }
        },
        None => panic!("not valid {}. got NONE instead.", desired_token),
    }
        
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
            Exp::UnOp(x,y) => write!(f, "UnOp<{}> {}", x, *y),
            _ => panic!("invalid expression"),
        }
    }
}

impl fmt::Display for Operator
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Operator::Negation => write!(f, "Negation"),
            Operator::Complement => write!(f, "Complement"),
            Operator::LogNegation => write!(f, "LogNegation"),
            _ => panic!("invalid operator"),
        }
    }
}