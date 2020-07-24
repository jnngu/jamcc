use std::collections::VecDeque;
use crate::lexer;
use std::fmt;


#[derive(Debug)]
pub enum Exp 
{
    Const(i32), 
    UnOp(UnaryOp, Box<Exp>), 
    BinOp(BinaryOp, Box<Exp>, Box<Exp>), 
}
#[derive(Debug)]
pub enum Statement {Return(Exp)}
#[derive(Debug)]
pub enum FunDecl {Fun(String, Statement)}
#[derive(Debug)]
pub enum Program {Prog(FunDecl)}
#[derive(Debug)]
pub enum UnaryOp
{
    Negation,
    Complement,
    LogNegation,
}
#[derive(Debug)]
pub enum BinaryOp
{
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub fn is_unop(token: &lexer::Token) -> bool
{
    match token
    {
        lexer::Token::Minus | lexer::Token::Complement | lexer::Token:: LogNegation => true,
        _ => false,
    }
}

pub fn parse_factor(mut token_vec: &mut VecDeque<lexer::Token>) -> Exp
{
    let tok = token_vec.pop_front().expect("token list empty(NONE)");

    if let lexer::Token::OpenParen = tok
    {
        debug_print!("OpenParen");
        let inner_exp = parse_exp(&mut token_vec);
        if let Some(lexer::Token::ClosedParen) = token_vec.pop_front() 
        {
            debug_print!("ClosedParen");
        }
        else 
        {
            panic!("not a closed paren");
        }
        return inner_exp
    }
    else if is_unop(&tok)
    {
        let op = get_unop(&tok);
        let inner_exp = parse_factor(&mut token_vec);
        return Exp::UnOp(op, Box::new(inner_exp)) 
    }
    else if let lexer::Token::IntegerLiteral(n) = tok
    {
        debug_print!("IntegerLiteral({})", n);
        return Exp::Const(n)
    }
    else
    {
        panic!("not valid expression");  
    }  
}

pub fn parse_muldiv(mut token_vec: &mut VecDeque<lexer::Token>) -> Exp
{
    let mut term = parse_factor(&mut token_vec);
    let mut next = token_vec.get(0).expect("empty token vector list");
    while let lexer::Token::Multiplication | lexer::Token::Division = next
    {
        let op = get_binop(token_vec.pop_front().expect("empty token vector list, binop"));
        let next_term = parse_factor(&mut token_vec);
        term = Exp::BinOp(op, Box::new(term), Box::new(next_term));
        next = token_vec.get(0).expect("empty token vector list");
    }
    term 
}

pub fn parse_addsub(mut token_vec: &mut VecDeque<lexer::Token>) -> Exp
{
    let mut term = parse_muldiv(&mut token_vec);
    let mut next = token_vec.get(0).expect("empty token vector list");
    while let lexer::Token::Addition | lexer::Token::Minus = next
    {
        let op = get_binop(token_vec.pop_front().expect("empty token vector list, binop"));
        let next_term = parse_muldiv(&mut token_vec);
        term = Exp::BinOp(op, Box::new(term), Box::new(next_term));
        next = token_vec.get(0).expect("empty token vector list");
    }
    term
}

pub fn get_binop(token: lexer::Token) -> BinaryOp
{
    match token
    {
        lexer::Token::Addition =>
        {
            debug_print!("Addition");
            BinaryOp::Addition
        }
        lexer::Token::Minus =>
        {
            debug_print!("Subtraction");
            BinaryOp::Subtraction
        }
        lexer::Token::Multiplication =>
        {
            debug_print!("Multiplication");
            BinaryOp::Multiplication
        }
        lexer::Token::Division =>
        {
            debug_print!("Division");
            BinaryOp::Division
        }
        _ => panic!("not valid binary operator"),
    }
}

pub fn get_unop(token: &lexer::Token) -> UnaryOp
{
    match token
    {
        lexer::Token::Minus => 
        {
            debug_print!("Negation");
            UnaryOp::Negation
        },
        lexer::Token::Complement => 
        {
            debug_print!("Complement");
            UnaryOp::Complement},
        lexer::Token::LogNegation => 
        {
            debug_print!("LogicalNegation");
            UnaryOp::LogNegation
        },
        _ => panic!("not valid operator"),
    }
}

pub fn parse_exp (mut token_vec:&mut VecDeque<lexer::Token>) -> Exp
{
    parse_addsub(&mut token_vec)
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
            Program::Prog(decl) => write!(f,"PROGRAM:\n{}\n", decl),
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
            Exp::Const(n) => write!(f, "Int<{}>", n),
            Exp::UnOp(x,y) => write!(f, "UnOp<{}> {}", x, *y),
            Exp::BinOp(x,y,z) => write!(f, "{} BinaryOp<{}> {}", *y, x, *z),
            _ => panic!("invalid expression"),
        }
    }
}

impl fmt::Display for UnaryOp
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            UnaryOp::Negation => write!(f, "Negation"),
            UnaryOp::Complement => write!(f, "Complement"),
            UnaryOp::LogNegation => write!(f, "LogNegation"),
            _ => panic!("invalid unary operator"),
        }
    }
}

impl fmt::Display for BinaryOp
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            BinaryOp::Addition => write!(f, "Addition"),
            BinaryOp::Subtraction => write!(f, "Subtraction"),
            BinaryOp::Multiplication => write!(f, "Multiplication"),
            BinaryOp::Division => write!(f, "Division"),
            _ => panic!("invalid binary operator"),
        }
    }
}