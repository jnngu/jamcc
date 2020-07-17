use regex::Regex;
use std::fs;


 pub enum Token {
    OpenBrace,
    ClosedBrace,
    OpenParen,
    ClosedParen,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier(String),
    IntegerLiteral(i32),
}
 

pub fn read_file(file_name: &str) -> String
{
    let contents = fs::read_to_string(file_name).expect("Unable to read file");
    contents
} 

pub fn print_tokens(x: Vec<Token>) -> ()
{
    let mut index = 0;
    let length = x.len()-1;
    print!("[");
    for elem in x
    {
        match elem
        {
            Token::OpenBrace => print!("OpenBrace"),
            Token::ClosedBrace => print!("ClosedBrace"),
            Token::OpenParen =>  print!("OpenParen"),
            Token::ClosedParen => print!("ClosedParen"),
            Token::Semicolon => print!("Semicolon"),
            Token::IntKeyword => print!("IntKeyword"),
            Token::ReturnKeyword => print!("ReturnKeyWord"),
            Token::Identifier(s) => print!("Identifier({})", s),
            Token::IntegerLiteral(i) => print!("Integer({})", i),
            _ => panic!("Invalid token"),

        }
        if index != length
        {
            index = index + 1;
            print!(", ");
        }

    }
    println!("]");
} 

pub fn parse_string(x: &str) -> Vec<Token>
{
    let re = Regex::new(r#"\A\{|\A\}|\A\(|\A\)|\A;"#).unwrap();
    let int_reg = Regex::new(r#"\A[0-9]+"#).unwrap();
    let char_reg = Regex::new(r#"\A[a-zA-Z]\w*"#).unwrap();

    let mut matches:Vec<Token> = Vec::new();
    //fix this when i know more about iterators xd


    let mut temp = String::from(x);
    while !(&temp.is_empty())
    {
        let next_match;
        if re.is_match_at(&temp, 0) 
        {
            next_match =  re.find(&temp).unwrap().as_str();
            match next_match {
                "{" => matches.push(Token::OpenBrace),
                "}" => matches.push(Token::ClosedBrace),
                "(" => matches.push(Token::OpenParen),
                ")" => matches.push(Token::ClosedParen),
                ";" => matches.push(Token::Semicolon),
                _ => panic!("not a valid token"),
            }
            println!("{}", next_match);
        }
        else if int_reg.is_match_at(&temp, 0)
        {
            next_match =  int_reg.find(&temp).unwrap().as_str();
            let num:i32 = next_match.parse().expect("Not an i32"); //TODO: do bounds checking here
            matches.push(Token::IntegerLiteral(num));
            println!("{}", next_match);
        }
        else if char_reg.is_match_at(&temp, 0)
        {
            next_match =  char_reg.find(&temp).unwrap().as_str();
            match next_match {
                "int" => matches.push(Token::IntKeyword),
                "return" => matches.push(Token::ReturnKeyword),
                _ => matches.push(Token::Identifier(next_match.to_string())),
            }
            println!("{}", next_match);
        }
        else
        {
            println!("invalid");
            break;
        }
        temp = temp.replacen(next_match, "", 1).trim_start().to_string();
    }

    matches  
}